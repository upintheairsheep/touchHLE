/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `printf` function family. The implementation is also used by `NSLog` etc.

use crate::abi::{DotDotDot, VaList};
use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::foundation::ns_string;
use crate::libc::posix_io::FileDescriptor;
use crate::libc::stdio::FILE;
use crate::mem::{ConstPtr, GuestUSize, Mem, MutPtr, MutVoidPtr};
use crate::objc::{id, msg};
use crate::Environment;
use std::io::Write;

/// String formatting implementation for `printf` and `NSLog` function families.
///
/// `NS_LOG` is [true] for the `NSLog` format string type, or [false] for the
/// `printf` format string type.
///
/// `get_format_char` is a callback that returns the byte at a given index in
/// the format string, or `'\0'` if the index is one past the last byte.
pub fn printf_inner<const NS_LOG: bool, F: Fn(&Mem, GuestUSize) -> u8>(
    env: &mut Environment,
    get_format_char: F,
    mut args: VaList,
) -> Vec<u8> {
    let mut res = Vec::<u8>::new();

    let mut format_char_idx = 0;

    loop {
        let c = get_format_char(&env.mem, format_char_idx);
        format_char_idx += 1;

        if c == b'\0' {
            break;
        }
        if c != b'%' {
            res.push(c);
            continue;
        }

        let mut pad_char = if get_format_char(&env.mem, format_char_idx) == b'0' {
            format_char_idx += 1;
            '0'
        } else {
            ' '
        };
        if get_format_char(&env.mem, format_char_idx) == b'.' {
            format_char_idx += 1;
            pad_char = '0';
        }
        let pad_width = {
            let mut pad_width = 0;
            while let c @ b'0'..=b'9' = get_format_char(&env.mem, format_char_idx) {
                pad_width = pad_width * 10 + (c - b'0') as usize;
                format_char_idx += 1;
            }
            pad_width
        };

        let precision = {
            let mut precision = 0;
            if get_format_char(&env.mem, format_char_idx) == b'.' {
                format_char_idx += 1;
                while let c @ b'0'..=b'9' = get_format_char(&env.mem, format_char_idx) {
                    precision = precision * 10 + (c - b'0') as usize;
                    format_char_idx += 1;
                }
            }
            precision
        };

        let specifier = get_format_char(&env.mem, format_char_idx);
        format_char_idx += 1;

        assert!(specifier != b'\0');
        if specifier == b'%' {
            res.push(b'%');
            continue;
        }

        match specifier {
            b'c' => {
                let c: u8 = args.next(env);
                assert!(pad_char == ' ' && pad_width == 0); // TODO
                res.push(c);
            }
            b's' => {
                let c_string: ConstPtr<u8> = args.next(env);
                //assert!(pad_char == ' ' && pad_width == 0); // TODO
                res.extend_from_slice(env.mem.cstr_at(c_string));
            }
            b'd' | b'i' | b'u' => {
                let int: i64 = if specifier == b'u' {
                    let uint: u32 = args.next(env);
                    uint.into()
                } else {
                    let int: i32 = args.next(env);
                    int.into()
                };
                if pad_width > 0 {
                    if pad_char == '0' {
                        write!(&mut res, "{:01$}", int, pad_width).unwrap();
                    } else {
                        write!(&mut res, "{:1$}", int, pad_width).unwrap();
                    }
                } else {
                    write!(&mut res, "{}", int).unwrap();
                }
            }
            b'f' => {
                let float: f64 = args.next(env);
                if pad_width > 0 {
                    if pad_char == '0' {
                        write!(&mut res, "{:01$}", float, pad_width).unwrap();
                    } else {
                        if precision > 0 {
                            write!(&mut res, "{:1$.2$}", float, pad_width, precision).unwrap();
                        } else {
                            write!(&mut res, "{:1$}", float, pad_width).unwrap();
                        }
                    }
                } else {
                    write!(&mut res, "{}", float).unwrap();
                }
            }
            b'@' if NS_LOG => {
                let object: id = args.next(env);
                // TODO: use localized description if available?
                let description: id = msg![env; object description];
                // TODO: avoid copy
                // TODO: what if the description isn't valid UTF-16?
                let description = ns_string::to_rust_string(env, description);
                write!(&mut res, "{}", description).unwrap();
            }
            b'x' => {
                let int: i32 = args.next(env);
                res.extend_from_slice(format!("{:x}", int).as_bytes());
            }
            b'p' => {
                let ptr: MutVoidPtr = args.next(env);
                res.extend_from_slice(format!("{:?}", ptr).as_bytes());
            }
            // TODO: more specifiers
            _ => unimplemented!("Format character '{}'", specifier as char),
        }
    }

    log_dbg!("=> {:?}", std::str::from_utf8(&res));

    res
}

fn vsnprintf(
    env: &mut Environment,
    dest: MutPtr<u8>,
    n: GuestUSize,
    format: ConstPtr<u8>,
    arg: VaList,
) -> i32 {
    log_dbg!(
        "vsnprintf({:?} {:?} {:?})",
        dest,
        format,
        env.mem.cstr_at_utf8(format)
    );

    let res = printf_inner::<false, _>(env, |mem, idx| mem.read(format + idx), arg);
    let middle = if ((n - 1) as usize) < res.len() {
        &res[..(n - 1) as usize]
    } else {
        &res[..]
    };

    let dest_slice = env.mem.bytes_at_mut(dest, n);
    for (i, &byte) in middle.iter().chain(b"\0".iter()).enumerate() {
        dest_slice[i] = byte;
    }

    res.len().try_into().unwrap()
}

fn snprintf(
    env: &mut Environment,
    dest: MutPtr<u8>,
    n: GuestUSize,
    format: ConstPtr<u8>,
    args: DotDotDot,
) -> i32 {
    vsnprintf(env, dest, n, format, args.start())
}

fn vprintf(env: &mut Environment, format: ConstPtr<u8>, arg: VaList) -> i32 {
    log_dbg!(
        "vprintf({:?} ({:?}), ...)",
        format,
        env.mem.cstr_at_utf8(format)
    );

    let res = printf_inner::<false, _>(env, |mem, idx| mem.read(format + idx), arg);
    // TODO: I/O error handling
    let _ = std::io::stdout().write_all(&res);
    res.len().try_into().unwrap()
}

fn vsprintf(env: &mut Environment, dest: MutPtr<u8>, format: ConstPtr<u8>, arg: VaList) -> i32 {
    log_dbg!(
        "vsprintf({:?}, {:?} ({:?}), ...)",
        dest,
        format,
        env.mem.cstr_at_utf8(format)
    );

    let res = printf_inner::<false, _>(env, |mem, idx| mem.read(format + idx), arg);

    let dest_slice = env
        .mem
        .bytes_at_mut(dest, (res.len() + 1).try_into().unwrap());
    for (i, &byte) in res.iter().chain(b"\0".iter()).enumerate() {
        dest_slice[i] = byte;
    }

    res.len().try_into().unwrap()
}

fn sprintf(env: &mut Environment, dest: MutPtr<u8>, format: ConstPtr<u8>, args: DotDotDot) -> i32 {
    log_dbg!(
        "sprintf({:?}, {:?} ({:?}), ...)",
        dest,
        format,
        env.mem.cstr_at_utf8(format)
    );

    let res = printf_inner::<false, _>(env, |mem, idx| mem.read(format + idx), args.start());

    let dest_slice = env
        .mem
        .bytes_at_mut(dest, (res.len() + 1).try_into().unwrap());
    for (i, &byte) in res.iter().chain(b"\0".iter()).enumerate() {
        dest_slice[i] = byte;
    }

    res.len().try_into().unwrap()
}

fn printf(env: &mut Environment, format: ConstPtr<u8>, args: DotDotDot) -> i32 {
    log_dbg!(
        "printf({:?} ({:?}), ...)",
        format,
        env.mem.cstr_at_utf8(format)
    );

    let res = printf_inner::<false, _>(env, |mem, idx| mem.read(format + idx), args.start());
    // TODO: I/O error handling
    let _ = std::io::stdout().write_all(&res);
    res.len().try_into().unwrap()
}

// TODO: more printf variants

fn sscanf(env: &mut Environment, src: ConstPtr<u8>, format: ConstPtr<u8>, args: DotDotDot) -> i32 {
    log_dbg!(
        "sscanf({:?}, {:?} ({:?}), ...)",
        src,
        format,
        env.mem.cstr_at_utf8(format)
    );

    let mut args = args.start();

    let mut src_ptr = src.cast_mut();
    let mut format_char_idx = 0;

    let mut matched_args = 0;

    loop {
        let c = env.mem.read(format + format_char_idx);
        format_char_idx += 1;

        if c == b'\0' {
            break;
        }
        if c != b'%' {
            let cc = env.mem.read(src_ptr);
            if c != cc {
                return matched_args - 1;
            }
            src_ptr += 1;
            continue;
        }

        let specifier = env.mem.read(format + format_char_idx);
        format_char_idx += 1;

        match specifier {
            b'd' => {
                let mut val: i32 = 0;
                while let c @ b'0'..=b'9' = env.mem.read(src_ptr) {
                    val = val * 10 + (c - b'0') as i32;
                    src_ptr += 1;
                }
                let c_int_ptr: ConstPtr<i32> = args.next(env);
                env.mem.write(c_int_ptr.cast_mut(), val);
            }
            // TODO: more specifiers
            _ => unimplemented!("Format character '{}'", specifier as char),
        }

        matched_args += 1;
    }

    matched_args
}

fn setbuf(_env: &mut Environment, _stream: MutPtr<FILE>, _buf: ConstPtr<u8>) {
    // TODO
}

fn fprintf(
    env: &mut Environment,
    _stream: MutPtr<FILE>,
    format: ConstPtr<u8>,
    args: DotDotDot,
) -> i32 {
    // TODO: assert that stream is stdio
    printf(env, format, args)
}

fn isatty(env: &mut Environment, fd: FileDescriptor) -> i32 {
    1
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(sscanf(_, _, _)),
    export_c_func!(vsnprintf(_, _, _, _)),
    export_c_func!(snprintf(_, _, _, _)),
    export_c_func!(vprintf(_, _)),
    export_c_func!(vsprintf(_, _, _)),
    export_c_func!(sprintf(_, _, _)),
    export_c_func!(printf(_, _)),
    export_c_func!(fprintf(_, _, _)),
    export_c_func!(setbuf(_, _)),
    export_c_func!(isatty(_)),
];
