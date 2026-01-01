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
use crate::mem::{GuestISize, MutPtr, MutVoidPtr};
use crate::Environment;

type OSSpinLock = i32;

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

fn OSAtomicOr32(
    env: &mut Environment,
    mask: u32,
    value: MutPtr<u32>,
) -> u32 {
    let cur = env.mem.read(value);
    let new = cur | mask;
    env.mem.write(value, new);
    new
}

fn OSAtomicAnd32(
    env: &mut Environment,
    mask: u32,
    value: MutPtr<u32>,
) -> u32 {
    let cur = env.mem.read(value);
    let new = cur & mask;
    env.mem.write(value, new);
    new
}

fn OSAtomicXor32(
    env: &mut Environment,
    mask: u32,
    value: MutPtr<u32>,
) -> u32 {
    let cur = env.mem.read(value);
    let new = cur ^ mask;
    env.mem.write(value, new);
    new
}

fn OSAtomicIncrement64(env: &mut Environment, value: MutPtr<i64>) -> i64 {
    OSAtomicIncrement64Barrier(env, value)
}

fn OSAtomicIncrement64Barrier(env: &mut Environment, value: MutPtr<i64>) -> i64 {
    let v = env.mem.read(value) + 1;
    env.mem.write(value, v);
    v
}

fn OSAtomicDecrement64(env: &mut Environment, value: MutPtr<i64>) -> i64 {
    OSAtomicDecrement64Barrier(env, value)
}

fn OSAtomicDecrement64Barrier(env: &mut Environment, value: MutPtr<i64>) -> i64 {
    let v = env.mem.read(value) - 1;
    env.mem.write(value, v);
    v
}

fn OSAtomicAddPtr(
    env: &mut Environment,
    amount: GuestISize,
    value: MutPtr<MutVoidPtr>,
) -> MutVoidPtr {
    OSAtomicAddPtrBarrier(env, amount, value)
}

fn OSAtomicAddPtrBarrier(
    env: &mut Environment,
    amount: GuestISize, // or i32
    value: MutPtr<MutVoidPtr>,
) -> MutVoidPtr {
    let cur = env.mem.read(value);
    let new = MutVoidPtr::from_bits(
        cur.to_bits().wrapping_add(amount as u32)
    );
    env.mem.write(value, new);
    new
}


fn OSAtomicCompareAndSwapInt(
    env: &mut Environment,
    old: i32,
    new: i32,
    value: MutPtr<i32>,
) -> bool {
    OSAtomicCompareAndSwap32Barrier(env, old, new, value)
}

fn OSSpinLockLock(env: &mut Environment, lock: MutPtr<OSSpinLock>) {
    let cur = env.mem.read(lock);
    if cur == 0 {
        env.mem.write(lock, 1);
    } else {
        // In a real system we'd spin.
        // In touchHLE, contention cannot occur.
        env.mem.write(lock, 1);
    }
}

fn OSSpinLockTry(env: &mut Environment, lock: MutPtr<OSSpinLock>) -> bool {
    let cur = env.mem.read(lock);
    if cur == 0 {
        env.mem.write(lock, 1);
        true
    } else {
        false
    }
}

fn OSSpinLockUnlock(env: &mut Environment, lock: MutPtr<OSSpinLock>) {
    env.mem.write(lock, 0);
}

fn OSSpinLockLockBarrier(env: &mut Environment, lock: MutPtr<OSSpinLock>) {
    OSSpinLockLock(env, lock)
}

fn OSSpinLockUnlockBarrier(env: &mut Environment, lock: MutPtr<OSSpinLock>) {
    OSSpinLockUnlock(env, lock)
}

fn OSSpinLockTryBarrier(env: &mut Environment, lock: MutPtr<OSSpinLock>) -> bool {
    OSSpinLockTry(env, lock)
}

/* --- Memory barriers (no-op by design) --- */

fn OSMemoryBarrier(_env: &mut Environment) {}
fn OSAtomicBarrier(_env: &mut Environment) {}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(OSAtomicAdd32(_, _)),
    export_c_func!(OSAtomicAdd32Barrier(_, _)),
    export_c_func!(OSAtomicCompareAndSwap32(_, _, _)),
    export_c_func!(OSAtomicCompareAndSwapIntBarrier(_, _, _)),
    export_c_func!(OSAtomicCompareAndSwap32Barrier(_, _, _)),
    export_c_func!(OSAtomicCompareAndSwapPtrBarrier(_, _, _)),
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
    export_c_func!(OSAtomicOr32(_, _)),
    export_c_func!(OSAtomicAnd32(_, _)),
    export_c_func!(OSAtomicXor32(_, _)),
    export_c_func!(OSAtomicIncrement64(_)),
    export_c_func!(OSAtomicIncrement64Barrier(_)),
    export_c_func!(OSAtomicDecrement64(_)),
    export_c_func!(OSAtomicDecrement64Barrier(_)),
    export_c_func!(OSAtomicAddPtr(_, _)),
    export_c_func!(OSAtomicAddPtrBarrier(_, _)),
    export_c_func!(OSAtomicCompareAndSwapInt(_, _, _)),
    export_c_func!(OSSpinLockLock(_)),
    export_c_func!(OSSpinLockUnlock(_)),
    export_c_func!(OSSpinLockTry(_)),
    export_c_func!(OSSpinLockLockBarrier(_)),
    export_c_func!(OSSpinLockUnlockBarrier(_)),
    export_c_func!(OSSpinLockTryBarrier(_)),
    export_c_func!(OSMemoryBarrier()),
    export_c_func!(OSAtomicBarrier()),
];
