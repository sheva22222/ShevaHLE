/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CFRunLoop`.
//!
//! This is not even toll-free bridged to `NSRunLoop` in Apple's implementation,
//! but here it is the same type.

use crate::dyld::{export_c_func, ConstantExports, FunctionExports, HostConstant};
use crate::frameworks::core_foundation::time::CFTimeInterval;
use crate::frameworks::foundation::ns_run_loop::run_run_loop_single_iteration;
use crate::frameworks::foundation::ns_string;
use crate::objc::{id, msg, msg_class};
use crate::Environment;

pub type CFRunLoopRef = super::CFTypeRef;
pub type CFRunLoopMode = super::cf_string::CFStringRef;

fn CFRunLoopGetCurrent(env: &mut Environment) -> CFRunLoopRef {
    msg_class![env; NSRunLoop currentRunLoop]
}

pub fn CFRunLoopGetMain(env: &mut Environment) -> CFRunLoopRef {
    msg_class![env; NSRunLoop mainRunLoop]
}

fn CFRunLoopRunInMode(
    env: &mut Environment,
    mode: CFRunLoopMode,
    seconds: CFTimeInterval,
    _return_something: bool,
) -> i32 {
    let default_mode = ns_string::get_static_str(env, kCFRunLoopDefaultMode);
    let common_modes = ns_string::get_static_str(env, kCFRunLoopCommonModes);
    // TODO: handle other modes
    assert!(
        msg![env; mode isEqualToString:default_mode]
            || msg![env; mode isEqualToString:common_modes]
    );
    let current_run_loop = CFRunLoopGetCurrent(env);
    if seconds == 0.0 {
        run_run_loop_single_iteration(env, current_run_loop);
    } else {
        let limit_date: id = msg_class![env; NSDate dateWithTimeIntervalSinceNow:seconds];
        () = msg![env; current_run_loop runUntilDate:limit_date];
    }
    1 // kCFRunLoopRunFinished
}

fn CFRunLoopRun(env: &mut Environment) {
    let run_loop = CFRunLoopGetCurrent(env);
    loop {
        run_run_loop_single_iteration(env, run_loop);
    }
}

fn CFRunLoopStop(_env: &mut Environment, _run_loop: CFRunLoopRef) {
    // No-op for now
    // NSRunLoop backend nie ma explicit stop flag
}

fn CFRunLoopWakeUp(_env: &mut Environment, _run_loop: CFRunLoopRef) {
    // No-op — run loop is always "awake" in this model
}

fn CFRunLoopIsWaiting(_env: &mut Environment, _run_loop: CFRunLoopRef) -> bool {
    false
}

fn CFRunLoopCopyCurrentMode(env: &mut Environment, _run_loop: CFRunLoopRef) -> CFRunLoopMode {
    ns_string::get_static_str(env, kCFRunLoopDefaultMode)
}

fn CFRunLoopAddCommonMode(
    _env: &mut Environment,
    _run_loop: CFRunLoopRef,
    _mode: CFRunLoopMode,
) {
    // No-op
}

fn CFRunLoopRunUntilDate(env: &mut Environment, limit_date: id) {
    let run_loop = CFRunLoopGetCurrent(env);
    () = msg![env; run_loop runUntilDate:limit_date];
}

fn CFRunLoopGetNextTimerFireDate(
    _env: &mut Environment,
    _run_loop: CFRunLoopRef,
    _mode: CFRunLoopMode,
) -> CFTimeInterval {
    0.0
}

fn CFRunLoopContainsSource(
    _env: &mut Environment,
    _run_loop: CFRunLoopRef,
    _source: super::CFTypeRef,
    _mode: CFRunLoopMode,
) -> bool {
    false
}

pub const kCFRunLoopCommonModes: &str = "kCFRunLoopCommonModes";
pub const kCFRunLoopDefaultMode: &str = "kCFRunLoopDefaultMode";

pub const CONSTANTS: ConstantExports = &[
    (
        "_kCFRunLoopCommonModes",
        HostConstant::NSString(kCFRunLoopCommonModes),
    ),
    (
        "_kCFRunLoopDefaultMode",
        HostConstant::NSString(kCFRunLoopDefaultMode),
    ),
];

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFRunLoopGetCurrent()),
    export_c_func!(CFRunLoopGetMain()),
    export_c_func!(CFRunLoopRun()),
    export_c_func!(CFRunLoopStop(_)),
    export_c_func!(CFRunLoopWakeUp(_)),
    export_c_func!(CFRunLoopIsWaiting(_)),
    export_c_func!(CFRunLoopCopyCurrentMode(_)),
    export_c_func!(CFRunLoopAddCommonMode(_, _)),
    export_c_func!(CFRunLoopRunUntilDate(_)),
    export_c_func!(CFRunLoopGetNextTimerFireDate(_, _)),
    export_c_func!(CFRunLoopContainsSource(_, _, _)),
    export_c_func!(CFRunLoopRunInMode(_, _, _)),
];

