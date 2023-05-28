/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `errno.h`

use crate::dyld::FunctionExports;
use crate::environment::Environment;
use crate::export_c_func;
use crate::mem::{ConstPtr, MutPtr};
use std::io::Write;

pub const EPERM: i32 = 1;
pub const EDEADLK: i32 = 11;
pub const EINVAL: i32 = 22;

#[derive(Default)]
pub struct State {
    errnos: std::collections::HashMap<crate::ThreadID, MutPtr<i32>>,
}
impl State {
    fn errno_for_thread(
        &mut self,
        mem: &mut crate::mem::Mem,
        thread: crate::ThreadID,
    ) -> MutPtr<i32> {
        // TODO: "real" errno implementation
        *self
            .errnos
            .entry(thread)
            .or_insert_with(|| mem.alloc_and_write(0i32))
    }
}

fn __error(env: &mut Environment) -> MutPtr<i32> {
    env.libc_state
        .errno
        .errno_for_thread(&mut env.mem, env.current_thread)
}

fn perror(env: &mut Environment, s: ConstPtr<u8>) {
    // TODO: errno mapping
    // TODO: null checks
    let _ = std::io::stderr().write_all(env.mem.cstr_at(s));
}

pub const FUNCTIONS: FunctionExports = &[export_c_func!(__error()), export_c_func!(perror(_))];
