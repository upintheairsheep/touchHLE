/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::dyld::FunctionExports;
use crate::environment::Environment;
use crate::export_c_func;
use crate::libc::errno::{set_errno, EINVAL};
use crate::libc::posix_io;
use crate::libc::posix_io::{off_t, FileDescriptor, SEEK_SET};
use crate::mem::{GuestUSize, MutVoidPtr};
use std::collections::HashMap;

#[allow(dead_code)]
const MAP_FILE: i32 = 0x0000;
const MAP_ANON: i32 = 0x1000;

#[derive(Default)]
pub struct State {
    /// Keeping track of `mmap` allocations
    allocations: HashMap<MutVoidPtr, GuestUSize>,
}

/// Our implementation of mmap is really simple: it's just load entirety of
/// file in memory!
fn mmap(
    env: &mut Environment,
    addr: MutVoidPtr,
    len: GuestUSize,
    _prot: i32,
    flags: i32,
    fd: FileDescriptor,
    offset: off_t,
) -> MutVoidPtr {
    // TODO: handle errno properly
    set_errno(env, 0);

    log_dbg!("mmap len {}", len);

    assert!(addr.is_null());
    assert_eq!(offset, 0);
    assert_eq!((flags & MAP_ANON), 0);
    let new_offset = posix_io::lseek(env, fd, offset, SEEK_SET);
    assert_eq!(new_offset, offset);
    let ptr = env.mem.alloc(len);

    assert!(!env.libc_state.mmap.allocations.contains_key(&ptr));
    env.libc_state.mmap.allocations.insert(ptr, len);

    let read = posix_io::read(env, fd, ptr, len);
    assert_eq!(read as u32, len);
    ptr
}

fn munmap(env: &mut Environment, addr: MutVoidPtr, len: GuestUSize) -> i32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    log_dbg!("munmap len {}", len);

    if len == 0 {
        set_errno(env, EINVAL);
        // TODO: should we clear allocations for `addr` here too?
        log!("Warning: munmap({:?}, {}) failed, returning -1", addr, len);
        return -1;
    }
    assert_eq!(*env.libc_state.mmap.allocations.get(&addr).unwrap(), len);
    env.mem.free(addr);
    env.libc_state.mmap.allocations.remove(&addr);
    0 // success
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(mmap(_, _, _, _, _, _)),
    export_c_func!(munmap(_, _)),
];
