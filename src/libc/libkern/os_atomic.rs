/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `libkern/OSAtomic.h`
//!
//! Atomic operations.
//!
//! Right now touchHLE is a single host thread application.
//! Thus, the execution of host functions couldn't be interrupted
//! by other threads. So we consider host functions to be atomic!

use crate::dyld::FunctionExports;
use crate::export_c_func;
use crate::mem::{MutPtr, MutVoidPtr};
use crate::Environment;

fn OSAtomicAdd32(env: &mut Environment, amount: i32, value_ptr: MutPtr<i32>) -> i32 {
    OSAtomicAdd32Barrier(env, amount, value_ptr)
}

fn OSAtomicAdd32Barrier(env: &mut Environment, the_amount: i32, the_value: MutPtr<i32>) -> i32 {
    let curr = env.mem.read(the_value);
    let new = curr + the_amount;
    env.mem.write(the_value, new);
    new
}

fn OSAtomicCompareAndSwap32(
    env: &mut Environment,
    old_value: i32,
    new_value: i32,
    the_value: MutPtr<i32>,
) -> bool {
    OSAtomicCompareAndSwap32Barrier(env, old_value, new_value, the_value)
}

fn OSAtomicCompareAndSwapIntBarrier(
    env: &mut Environment,
    old_value: i32,
    new_value: i32,
    the_value: MutPtr<i32>,
) -> bool {
    OSAtomicCompareAndSwap32Barrier(env, old_value, new_value, the_value)
}

fn OSAtomicCompareAndSwap32Barrier(
    env: &mut Environment,
    old_value: i32,
    new_value: i32,
    the_value: MutPtr<i32>,
) -> bool {
    if old_value == env.mem.read(the_value) {
        env.mem.write(the_value, new_value);
        true
    } else {
        false
    }
}

fn OSAtomicCompareAndSwapPtrBarrier(
    env: &mut Environment,
    old_value: MutVoidPtr,
    new_value: MutVoidPtr,
    the_value: MutPtr<MutVoidPtr>,
) -> bool {
    if old_value == env.mem.read(the_value) {
        env.mem.write(the_value, new_value);
        true
    } else {
        false
    }
}

/* --- 32-bit arithmetic --- */

fn OSAtomicIncrement32(env: &mut Environment, value: MutPtr<i32>) -> i32 {
    OSAtomicIncrement32Barrier(env, value)
}

fn OSAtomicIncrement32Barrier(env: &mut Environment, value: MutPtr<i32>) -> i32 {
    let v = env.mem.read(value) + 1;
    env.mem.write(value, v);
    v
}

fn OSAtomicDecrement32(env: &mut Environment, value: MutPtr<i32>) -> i32 {
    OSAtomicDecrement32Barrier(env, value)
}

fn OSAtomicDecrement32Barrier(env: &mut Environment, value: MutPtr<i32>) -> i32 {
    let v = env.mem.read(value) - 1;
    env.mem.write(value, v);
    v
}

/* --- 64-bit arithmetic --- */

fn OSAtomicAdd64(env: &mut Environment, amount: i64, value: MutPtr<i64>) -> i64 {
    OSAtomicAdd64Barrier(env, amount, value)
}

fn OSAtomicAdd64Barrier(env: &mut Environment, amount: i64, value: MutPtr<i64>) -> i64 {
    let v = env.mem.read(value) + amount;
    env.mem.write(value, v);
    v
}

/* --- Compare-and-swap --- */

fn OSAtomicCompareAndSwap64(
    env: &mut Environment,
    old: i64,
    new: i64,
    value: MutPtr<i64>,
) -> bool {
    OSAtomicCompareAndSwap64Barrier(env, old, new, value)
}

fn OSAtomicCompareAndSwap64Barrier(
    env: &mut Environment,
    old: i64,
    new: i64,
    value: MutPtr<i64>,
) -> bool {
    if env.mem.read(value) == old {
        env.mem.write(value, new);
        true
    } else {
        false
    }
}

fn OSAtomicCompareAndSwapPtr(
    env: &mut Environment,
    old: MutVoidPtr,
    new: MutVoidPtr,
    value: MutPtr<MutVoidPtr>,
) -> bool {
    OSAtomicCompareAndSwapPtrBarrier(env, old, new, value)
}

/* --- Bit operations --- */

fn OSAtomicTestAndSet(
    env: &mut Environment,
    bit: u32,
    value: MutPtr<u32>,
) -> bool {
    let mask = 1u32 << bit;
    let cur = env.mem.read(value);
    let was_set = cur & mask != 0;
    env.mem.write(value, cur | mask);
    was_set
}

fn OSAtomicTestAndClear(
    env: &mut Environment,
    bit: u32,
    value: MutPtr<u32>,
) -> bool {
    let mask = 1u32 << bit;
    let cur = env.mem.read(value);
    let was_set = cur & mask != 0;
    env.mem.write(value, cur & !mask);
    was_set
}

/* --- Memory barriers (no-op by design) --- */

fn OSMemoryBarrier(_env: &mut Environment) {}
fn OSAtomicBarrier(_env: &mut Environment) {}

pub const FUNCTIONS: FunctionExports = &[
    /* existing */
    export_c_func!(OSAtomicAdd32(_, _)),
    export_c_func!(OSAtomicAdd32Barrier(_, _)),
    export_c_func!(OSAtomicCompareAndSwap32(_, _, _)),
    export_c_func!(OSAtomicCompareAndSwapIntBarrier(_, _, _)),
    export_c_func!(OSAtomicCompareAndSwap32Barrier(_, _, _)),
    export_c_func!(OSAtomicCompareAndSwapPtrBarrier(_, _, _)),

    /* new */
    export_c_func!(OSAtomicIncrement32(_)),
    export_c_func!(OSAtomicIncrement32Barrier(_)),
    export_c_func!(OSAtomicDecrement32(_)),
    export_c_func!(OSAtomicDecrement32Barrier(_)),
    export_c_func!(OSAtomicAdd64(_, _)),
    export_c_func!(OSAtomicAdd64Barrier(_, _)),
    export_c_func!(OSAtomicCompareAndSwap64(_, _, _)),
    export_c_func!(OSAtomicCompareAndSwap64Barrier(_, _, _)),
    export_c_func!(OSAtomicCompareAndSwapPtr(_, _, _)),
    export_c_func!(OSAtomicTestAndSet(_, _)),
    export_c_func!(OSAtomicTestAndClear(_, _)),
    export_c_func!(OSMemoryBarrier()),
    export_c_func!(OSAtomicBarrier()),
];
