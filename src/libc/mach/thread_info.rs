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
use crate::mem::{guest_size_of, GuestUSize, MutPtr, MutVoidPtr, SafeRead};
use crate::Environment;
use std::sync::atomic::{AtomicU32, Ordering};

// TODO: Move these common definitions into separate modules
pub type kern_return_t = i32;
pub const KERN_SUCCESS: kern_return_t = 0;
pub const KERN_FAILURE: kern_return_t = 5;

pub type mach_port_t = u32;

type thread_inspect_t = mach_port_t;
type thread_flavor_t = natural_t;
type thread_info_t = MutPtr<integer_t>;
pub type mach_msg_type_number_t = natural_t;
pub type mach_msg_return_t = kern_return_t;
pub type mach_msg_size_t = u32;

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

type task_flavor_t = natural_t;
type task_info_t = MutPtr<integer_t>;

const TASK_BASIC_INFO: task_flavor_t = 4;

#[repr(C, packed)]
struct task_basic_info {
    virtual_size: u64,
    resident_size: u64,
    resident_size_max: u64,
    user_time: u64,
    system_time: u64,
    policy: integer_t,
    suspend_count: integer_t,
}
unsafe impl SafeRead for task_basic_info {}

const TH_STATE_RUNNING: integer_t = 1;
const TH_STATE_STOPPED: integer_t = 2;

pub const MACH_PORT_RIGHT_RECEIVE: i32 = 1;
pub const MACH_MSG_TYPE_MAKE_SEND: i32 = 20;

pub struct MachState {
    next_port: mach_port_t,
}

impl MachState {
    pub fn next_port(&mut self) -> mach_port_t {
        self.next_port += 1;
        self.next_port
    }
}

static NEXT_MACH_PORT: AtomicU32 = AtomicU32::new(100);

pub const MACH_MSG_SUCCESS: mach_msg_return_t = 0;

/* ---- mach_msg option flags (subset) ---- */

pub type mach_msg_option_t = integer_t;

pub const MACH_SEND_MSG: mach_msg_option_t = 0x00000001;
pub const MACH_RCV_MSG: mach_msg_option_t  = 0x00000002;
pub const MACH_RCV_TIMEOUT: mach_msg_option_t = 0x00000100;

#[repr(C, packed)]
pub struct mach_msg_header_t {
    pub msgh_bits: u32,
    pub msgh_size: u32,
    pub msgh_remote_port: mach_port_t,
    pub msgh_local_port: mach_port_t,
    pub msgh_reserved: u32,
    pub msgh_id: i32,
    pub _private: u32,
}
unsafe impl SafeRead for mach_msg_header_t {}

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

pub type vm_map_t = u32;
pub type vm_address_t = MutVoidPtr;
pub type vm_size_t = GuestUSize;

fn vm_deallocate(
    env: &mut Environment,
    _target_task: vm_map_t,
    address: vm_address_t,
    size: vm_size_t,
) -> kern_return_t {
    if address.is_null() || size == 0 {
        return KERN_FAILURE;
    }

    // Your Mem implementation already owns the allocation
    env.mem.free(address);

    KERN_SUCCESS
}

fn mach_port_allocate(
    env: &mut Environment,
    _task: ipc_space_t,
    _right: i32,
    name: MutPtr<mach_port_t>,
) -> kern_return_t {
    let port = NEXT_MACH_PORT.fetch_add(1, Ordering::Relaxed);
    env.mem.write(name, port);
    KERN_SUCCESS
}

fn mach_port_insert_right(
    _env: &mut Environment,
    _task: ipc_space_t,
    _name: mach_port_t,
    _poly: mach_port_t,
    _poly_poly: i32,
) -> kern_return_t {
    KERN_SUCCESS
}

pub const VM_FLAGS_ANYWHERE: i32 = 1;

fn vm_allocate(
    env: &mut Environment,
    _target_task: vm_map_t,
    address: MutPtr<MutVoidPtr>,
    size: vm_size_t,
    _flags: i32,
) -> kern_return_t {
    if size == 0 {
        return KERN_FAILURE;
    }

    let ptr = env.mem.alloc(size);
    env.mem.write(address, ptr);

    KERN_SUCCESS
}

fn task_info(
    env: &mut Environment,
    _task: mach_port_t,
    flavor: task_flavor_t,
    task_info_out: task_info_t,
    task_info_out_count: MutPtr<mach_msg_type_number_t>,
) -> kern_return_t {
    let out_count = env.mem.read(task_info_out_count);

    match flavor {
        TASK_BASIC_INFO => {
            let expected =
                guest_size_of::<task_basic_info>() / guest_size_of::<integer_t>();

            if out_count < expected as u32 {
                return KERN_FAILURE;
            }

            env.mem.write(
                task_info_out.cast(),
                task_basic_info {
                    virtual_size: 0,
                    resident_size: 0,
                    resident_size_max: 0,
                    user_time: 0,
                    system_time: 0,
                    policy: 0,
                    suspend_count: 0,
                },
            );

            env.mem.write(task_info_out_count, expected as u32);
            KERN_SUCCESS
        }

        _ => {
            log!("task_info: unsupported flavor {}", flavor);
            KERN_FAILURE
        }
    }
}

fn mach_msg(
    env: &mut Environment,
    msg: MutPtr<mach_msg_header_t>,
    option: mach_msg_option_t,
    send_size: mach_msg_size_t,
    rcv_size: mach_msg_size_t,
    _rcv_name: mach_port_t,
    _timeout: natural_t,
    _notify: mach_port_t,
) -> mach_msg_return_t {
    log_dbg!(
        "mach_msg(msg={:?}, option=0x{:x}, send={}, recv={})",
        msg,
        option,
        send_size,
        rcv_size
    );

    /* ---- receive path ---- */
    if (option & MACH_RCV_MSG) != 0 && !msg.is_null() {
        // Zero out the receive buffer header at minimum
        let header = mach_msg_header_t {
            msgh_bits: 0,
            msgh_size: rcv_size,
            msgh_remote_port: 0,
            msgh_local_port: 0,
            msgh_reserved: 0,
            msgh_id: 0,
            _private: 0,
        };
        env.mem.write(msg, header);
    }

    MACH_MSG_SUCCESS
}

fn exc_server(
    _env: &mut Environment,
    _in_msg: MutPtr<mach_msg_header_t>,
    _out_msg: MutPtr<mach_msg_header_t>,
) -> boolean_t {
    // We do not handle Mach exceptions
    0 // FALSE
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
    export_c_func!(vm_deallocate(_, _, _)),
    export_c_func!(mach_port_allocate(_, _, _)),
    export_c_func!(mach_port_insert_right(_, _, _, _)),
    export_c_func!(task_info(_, _, _, _)),
    export_c_func!(mach_msg(_, _, _, _, _, _, _)),
    export_c_func!(exc_server(_, _)),
];
