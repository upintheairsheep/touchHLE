/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::dyld::FunctionExports;
use crate::environment::Environment;
use crate::export_c_func;
use crate::libc::posix_io;
use crate::libc::posix_io::{off_t, FileDescriptor, SEEK_SET};
use crate::mem::{GuestUSize, MutVoidPtr};

/// Our implementation of mmap is really simple: it's just load entirety of file in memory!
fn mmap(
    env: &mut Environment,
    addr: MutVoidPtr,
    len: GuestUSize,
    _prot: i32,
    _flags: i32,
    fd: FileDescriptor,
    offset: off_t,
) -> MutVoidPtr {
    assert!(addr.is_null());
    assert_eq!(offset, 0);
    let new_offset = posix_io::lseek(env, fd, offset, SEEK_SET);
    assert_eq!(new_offset, offset);
    let ptr = env.mem.alloc(len);
    let read = posix_io::read(env, fd, ptr, len);
    assert_eq!(read as u32, len);
    ptr
}

pub const FUNCTIONS: FunctionExports = &[export_c_func!(mmap(_, _, _, _, _, _))];
