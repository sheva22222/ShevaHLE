/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::dyld::FunctionExports;
use crate::environment::Environment;
use crate::export_c_func;
use crate::libc::errno::set_errno;
use crate::mem::{ConstVoidPtr, MutVoidPtr, Ptr};

fn sigaction(env: &mut Environment, signum: i32, act: ConstVoidPtr, oldact: MutVoidPtr) -> i32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    log!("TODO: sigaction({:?}, {:?}, {:?})", signum, act, oldact);
    0
}

fn signal(env: &mut Environment, signum: i32, handler: MutVoidPtr) -> MutVoidPtr {
    // TODO: handle errno properly
    set_errno(env, 0);

    log!("TODO: signal({:?}, {:?})", signum, handler);
    Ptr::null()
}

fn sigprocmask(
    env: &mut Environment,
    how: i32,
    set: ConstVoidPtr,
    oldset: MutVoidPtr,
) -> i32 {
    set_errno(env, 0);
    log!("TODO: sigprocmask({}, {:?}, {:?})", how, set, oldset);
    0
}

fn sigemptyset(env: &mut Environment, set: MutVoidPtr) -> i32 {
    set_errno(env, 0);
    log!("TODO: sigemptyset({:?})", set);
    0
}

fn sigfillset(env: &mut Environment, set: MutVoidPtr) -> i32 {
    set_errno(env, 0);
    log!("TODO: sigfillset({:?})", set);
    0
}

fn sigaddset(env: &mut Environment, set: MutVoidPtr, signum: i32) -> i32 {
    set_errno(env, 0);
    log!("TODO: sigaddset({:?}, {})", set, signum);
    0
}

fn sigdelset(env: &mut Environment, set: MutVoidPtr, signum: i32) -> i32 {
    set_errno(env, 0);
    log!("TODO: sigdelset({:?}, {})", set, signum);
    0
}

fn sigismember(env: &mut Environment, set: ConstVoidPtr, signum: i32) -> i32 {
    set_errno(env, 0);
    log!("TODO: sigismember({:?}, {})", set, signum);
    0 // not a member
}

fn sigpending(env: &mut Environment, set: MutVoidPtr) -> i32 {
    set_errno(env, 0);
    log!("TODO: sigpending({:?})", set);
    0
}

fn sigsuspend(env: &mut Environment, mask: ConstVoidPtr) -> i32 {
    set_errno(env, 0);
    log!("TODO: sigsuspend({:?})", mask);
    -1 // POSIX: returns -1 with EINTR, but we ignore
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(sigaction(_, _, _)),
    export_c_func!(signal(_, _)),
    export_c_func!(sigprocmask(_, _, _)),
    export_c_func!(sigemptyset(_)),
    export_c_func!(sigfillset(_)),
    export_c_func!(sigaddset(_, _)),
    export_c_func!(sigdelset(_, _)),
    export_c_func!(sigismember(_, _)),
    export_c_func!(sigpending(_)),
    export_c_func!(sigsuspend(_)),
];
