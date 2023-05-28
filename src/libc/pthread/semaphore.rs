/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `semaphore.h`

use std::collections::{HashMap, HashSet};
use sdl2::libc::usleep;
use crate::dyld::{export_c_func, FunctionExports};
use crate::libc::posix_io::stat::mode_t;
use crate::mem::{ConstPtr, MutPtr, Ptr};
use crate::{Environment, ThreadID};
use crate::libc::unistd;

#[derive(Default)]
pub struct State {
    pub semaphores: HashMap<MutPtr<sem_t>, SemaphoreHostObject>,
}
impl State {
    fn get(env: &mut Environment) -> &mut Self {
        &mut env.libc_state.pthread.semaphore
    }
}

#[allow(non_camel_case_types)]
type sem_t = i32;

const SEM_FAILED: i32 = -1;

pub struct SemaphoreHostObject {
    pub value: i32,
    pub waiting: HashSet<ThreadID>
}

fn sem_open(
    env: &mut Environment,
    _name: ConstPtr<u8>,
    _oflag: i32,
    _mode: mode_t,
    value: u32,
) -> MutPtr<sem_t> {
    let sem = env.mem.alloc_and_write(0);

    assert!(!State::get(env).semaphores.contains_key(&sem));
    State::get(env).semaphores.insert(
        sem,
        SemaphoreHostObject {
            value: value as i32,
            waiting: HashSet::new()
        },
    );

    sem
}

fn sem_post(env: &mut Environment, sem: MutPtr<sem_t>) -> i32 {
    //unistd::usleep(_env, 1000);

    //let host_object: &mut _ = State::get(env).semaphores.get_mut(&sem).unwrap();

    env.unsleep_sem(sem);

    // // TODO: ensure that this is an atomic operation?
    // host_object.value += 1;
    //
    // if host_object.value > 0 {
    //     let mut set = &host_object.waiting;
    //     for thread_id in set {
    //         // let thread = &mut env.threads[*thread_id];
    //         // assert!(thread.sleeping_until.is_some());
    //         // thread.sleeping_until = None;
    //     }
    //     //host_object.waiting.clear();
    // }

    0
}

fn sem_wait(env: &mut Environment, sem: MutPtr<sem_t>) -> i32 {
    env.sleep_sem(sem, true);
    0
}

fn sem_trywait(env: &mut Environment, sem: MutPtr<sem_t>) -> i32 {
    if env.sleep_sem(sem, false) {
        0
    } else {
        -1
    }
}

fn sem_unlink(_env: &mut Environment, _name: ConstPtr<u8>,) -> i32 {
    0
}

fn sem_close(env: &mut Environment, sem: MutPtr<sem_t>) -> i32 {
    State::get(env).semaphores.remove(&sem);
    env.mem.free(sem.cast());
    0
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(sem_open(_, _, _, _)),
    export_c_func!(sem_post(_)),
    export_c_func!(sem_wait(_)),
    export_c_func!(sem_trywait(_)),
    export_c_func!(sem_unlink(_)),
    export_c_func!(sem_close(_)),
];
