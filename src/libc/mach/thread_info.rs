/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `mach/thread_info.h`
//!
//! This is extremely undocumented. :(

#![allow(non_camel_case_types)]

use crate::dyld::{export_c_func, FunctionExports};
use crate::libc::mach::core_types::{boolean_t, integer_t, natural_t};
use crate::libc::mach::init::mach_task_self;
use crate::mem::{guest_size_of, MutPtr, SafeRead};
use crate::Environment;

// TODO: Move these common definitions into separate modules
pub type kern_return_t = i32;
pub const KERN_SUCCESS: kern_return_t = 0;
pub const KERN_FAILURE: kern_return_t = 5;

pub type mach_port_t = u32;

type thread_inspect_t = mach_port_t;
type thread_flavor_t = natural_t;
type thread_info_t = MutPtr<integer_t>;
pub type mach_msg_type_number_t = natural_t;

type policy_t = i32;
const POLICY_TIMESHARE: policy_t = 1;

const THREAD_BASIC_INFO: thread_flavor_t = 3;
const THREAD_SCHED_TIMESHARE_INFO: thread_flavor_t = 10;

#[repr(C, packed)]
struct time_value_t {
    seconds: integer_t,
    microseconds: integer_t,
}
unsafe impl SafeRead for time_value_t {}

#[repr(C, packed)]
struct thread_basic_info {
    user_time: time_value_t,
    system_time: time_value_t,
    cpu_usage: integer_t,
    policy: policy_t,
    run_state: integer_t,
    flags: integer_t,
    suspend_count: integer_t,
    sleep_time: integer_t,
}
unsafe impl SafeRead for thread_basic_info {}

#[repr(C, packed)]
struct policy_timeshare_info {
    max_priority: integer_t,
    base_priority: integer_t,
    cur_priority: integer_t,
    depressed: boolean_t,
    depress_priority: integer_t,
}
unsafe impl SafeRead for policy_timeshare_info {}

const TH_STATE_RUNNING: integer_t = 1;
const TH_STATE_STOPPED: integer_t = 2;

/// Undocumented Darwin function that returns information about a thread.
///
/// I swear these are the correct type names, the API is just... like this.
fn thread_info(
    env: &mut Environment,
    target_act: thread_inspect_t,
    flavor: thread_flavor_t,
    thread_info_out: thread_info_t,
    thread_info_out_count: MutPtr<mach_msg_type_number_t>,
) -> kern_return_t {
    let thread = env.threads.get(target_act as usize).unwrap();

    let out_size_available = env.mem.read(thread_info_out_count);

    match flavor {
        THREAD_BASIC_INFO => {
            let out_size_expected =
                guest_size_of::<thread_basic_info>() / guest_size_of::<integer_t>();
            assert!(out_size_expected == out_size_available);
            env.mem.write(
                thread_info_out.cast(),
                thread_basic_info {
                    user_time: time_value_t {
                        seconds: 0,
                        microseconds: 0,
                    },
                    system_time: time_value_t {
                        seconds: 0,
                        microseconds: 0,
                    },
                    cpu_usage: 0,
                    policy: POLICY_TIMESHARE, // no idea if this is realistic
                    run_state: if thread.active {
                        TH_STATE_RUNNING
                    } else {
                        TH_STATE_STOPPED
                    },
                    flags: 0, // FIXME
                    suspend_count: 0,
                    sleep_time: 0,
                },
            );
        }
        THREAD_SCHED_TIMESHARE_INFO => {
            let out_size_expected =
                guest_size_of::<policy_timeshare_info>() / guest_size_of::<integer_t>();
            assert!(out_size_expected == out_size_available);
            env.mem.write(
                thread_info_out.cast(),
                policy_timeshare_info {
                    max_priority: 0,
                    base_priority: 0,
                    cur_priority: 0,
                    depressed: 0,
                    depress_priority: 0,
                },
            );
        }
        _ => unimplemented!("TODO: flavor {:?}", flavor),
    }

    KERN_SUCCESS
}

type thread_t = mach_port_t;
type thread_policy_flavor_t = natural_t;
type thread_policy_t = MutPtr<integer_t>;

// This is actually from the thread policy file.
fn thread_policy_set(
    _env: &mut Environment,
    thread: thread_t,
    flavor: thread_policy_flavor_t,
    policy_info: thread_policy_t,
    count: mach_msg_type_number_t,
) -> kern_return_t {
    log!(
        "TODO: thread_policy_set({}, {}, {:?}, {}) (ignored)",
        thread,
        flavor,
        policy_info,
        count
    );
    KERN_SUCCESS
}

fn thread_resume(_env: &mut Environment, _thread: thread_t) -> kern_return_t {
    KERN_SUCCESS
}

fn thread_suspend(_env: &mut Environment, _thread: thread_t) -> kern_return_t {
    KERN_SUCCESS
}

fn thread_terminate(_env: &mut Environment, _thread: thread_t) -> kern_return_t {
    log!("thread_terminate: ignored");
    KERN_SUCCESS
}

type thread_state_flavor_t = natural_t;
type thread_state_t = MutPtr<integer_t>;

fn thread_get_state(
    _env: &mut Environment,
    _thread: thread_t,
    _flavor: thread_state_flavor_t,
    _state: thread_state_t,
    _count: MutPtr<mach_msg_type_number_t>,
) -> kern_return_t {
    KERN_SUCCESS
}

fn thread_set_state(
    _env: &mut Environment,
    _thread: thread_t,
    _flavor: thread_state_flavor_t,
    _state: thread_state_t,
    _count: mach_msg_type_number_t,
) -> kern_return_t {
    KERN_SUCCESS
}

fn thread_abort(_env: &mut Environment, _thread: thread_t) -> kern_return_t {
    KERN_SUCCESS
}

fn thread_abort_safely(_env: &mut Environment, _thread: thread_t) -> kern_return_t {
    KERN_SUCCESS
}

type exception_mask_t = u32;
type exception_handler_t = mach_port_t;
type exception_behavior_t = i32;

fn thread_get_exception_ports(
    _env: &mut Environment,
    _thread: thread_t,
    _mask: exception_mask_t,
    _masks: MutPtr<exception_mask_t>,
    _count: MutPtr<mach_msg_type_number_t>,
    _ports: MutPtr<exception_handler_t>,
    _behaviors: MutPtr<exception_behavior_t>,
    _flavors: MutPtr<thread_state_flavor_t>,
) -> kern_return_t {
    KERN_SUCCESS
}

type task_t = u32;
type thread_act_array_t = MutPtr<thread_t>;

fn task_threads(
    env: &mut Environment,
    task: task_t,
    threads_out: MutPtr<thread_act_array_t>,
    thread_count_out: MutPtr<mach_msg_type_number_t>,
) -> kern_return_t {
    // Only support current task
    if task != mach_task_self(env) {
        return KERN_FAILURE;
    }

    let count = env.threads.len() as mach_msg_type_number_t;

    // Allocate array manually
    let array = env
        .mem
        .alloc((count as u32 * guest_size_of::<thread_t>()) as u32)
        .cast::<thread_t>();

    // Write thread ports (index == mach port)
    for i in 0..count {
        env.mem.write(array + i, i as thread_t);
    }

    env.mem.write(threads_out, array);
    env.mem.write(thread_count_out, count);

    KERN_SUCCESS
}

fn mach_thread_self(env: &mut Environment) -> mach_port_t {
    env.current_thread as mach_port_t
}

type ipc_space_t = mach_port_t;
type mach_port_name_t = mach_port_t;

fn mach_port_deallocate(
    _env: &mut Environment,
    _task: ipc_space_t,
    _name: mach_port_name_t,
) -> kern_return_t {
    // Early iOS behavior:
    // - Always succeeds
    // - No actual port rights tracking
    KERN_SUCCESS
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(thread_info(_, _, _, _)),
    export_c_func!(thread_policy_set(_, _, _, _)),
    export_c_func!(thread_resume(_)),
    export_c_func!(thread_suspend(_)),
    export_c_func!(thread_terminate(_)),
    export_c_func!(thread_get_state(_, _, _, _)),
    export_c_func!(thread_set_state(_, _, _, _)),
    export_c_func!(thread_abort(_)),
    export_c_func!(thread_abort_safely(_)),
    export_c_func!(thread_get_exception_ports(_, _, _, _, _, _, _)),
    export_c_func!(task_threads(_, _, _)),
    export_c_func!(mach_thread_self()),
    export_c_func!(mach_port_deallocate(_, _)),
];
