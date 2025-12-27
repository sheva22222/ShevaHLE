/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CFArray` and `CFMutableArray`.
//!
//! These are toll-free bridged to `NSArray` and `NSMutableArray` in Apple's
//! implementation. Here they are the same types.

use super::cf_allocator::{kCFAllocatorDefault, CFAllocatorRef};
use super::CFIndex;
use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::core_foundation::CFRange;
use crate::frameworks::foundation::NSUInteger;
use crate::mem::{ConstVoidPtr, MutPtr, Ptr};
use crate::objc::{id, msg, msg_class};
use crate::Environment;
use std::ops::Add;

#[allow(dead_code)]
pub type CFArrayRef = super::CFTypeRef;
pub type CFMutableArrayRef = super::CFTypeRef;

fn CFArrayCreateMutable(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    capacity: CFIndex,
    callbacks: ConstVoidPtr, // TODO, should be `const CFArrayCallBacks*`
) -> CFMutableArrayRef {
    assert!(allocator == kCFAllocatorDefault); // unimplemented
    assert!(capacity == 0); // TODO: fixed capacity support
    assert!(callbacks.is_null()); // TODO: support retaining etc

    msg_class![env; _touchHLE_NSMutableArray_non_retaining new]
}

fn CFArrayCreateMutableCopy(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    capacity: CFIndex,
    array: CFArrayRef,
) -> CFMutableArrayRef {
    assert!(allocator == kCFAllocatorDefault);
    assert!(capacity == 0); // fixed-capacity arrays not supported yet

    let copy: id = msg![env; array mutableCopy];
    copy
}

fn CFArrayCreate(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    values: Ptr<ConstVoidPtr, false>,
    num_values: CFIndex,
    callbacks: ConstVoidPtr,
) -> CFArrayRef {
    assert!(allocator == kCFAllocatorDefault);
    assert!(callbacks.is_null());

    let array: id = msg_class![env; NSArray alloc];
    let array: id = msg![env; array init];

    if num_values > 0 {
        for i in 0..num_values {
            let value: ConstVoidPtr = env.mem.read(values + i.try_into().unwrap());
            let mut out: MutPtr<ConstVoidPtr> = values.cast_mut();

            for i in 0..count {
                env.mem.write(out + i, value);
            }
            
            let obj: id = value.cast().cast_mut();
            msg![env; array addObject:obj];
        }
    }

    array
}

fn CFArrayCreateCopy(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    array: CFArrayRef,
) -> CFArrayRef {
    assert!(allocator == kCFAllocatorDefault);

    let copy: id = msg![env; array copy];
    copy
}

fn CFArrayGetCount(env: &mut Environment, array: CFArrayRef) -> CFIndex {
    let count: NSUInteger = msg![env; array count];
    count.try_into().unwrap()
}

fn CFArrayGetValueAtIndex(env: &mut Environment, array: CFArrayRef, idx: CFIndex) -> ConstVoidPtr {
    let idx: NSUInteger = idx.try_into().unwrap();
    let value: id = msg![env; array objectAtIndex:idx];
    value.cast().cast_const()
}

fn CFArrayGetValues(
    env: &mut Environment,
    array: CFArrayRef,
    range: CFRange,
    values: MutPtr<ConstVoidPtr>,
) {
    let count = range.length; // ← THIS WAS MISSING

    let mut out: MutPtr<ConstVoidPtr> = values;

    for i in 0..count {
        let idx: NSUInteger = (range.location + i).try_into().unwrap();
        let obj: id = msg![env; array objectAtIndex:idx];

        env.mem.write(out + i, obj.cast().cast_const());
    }
}

fn CFArrayAppendValue(env: &mut Environment, array: CFMutableArrayRef, value: ConstVoidPtr) {
    let value: id = value.cast().cast_mut();
    msg![env; array addObject:value]
}

fn CFArrayInsertValueAtIndex(
    env: &mut Environment,
    array: CFMutableArrayRef,
    idx: CFIndex,
    value: ConstVoidPtr,
) {
    let idx: NSUInteger = idx.try_into().unwrap();
    let value: id = value.cast().cast_mut();
    msg![env; array insertObject:value atIndex:idx]
}

fn CFArraySetValueAtIndex(
    env: &mut Environment,
    array: CFMutableArrayRef,
    idx: CFIndex,
    value: ConstVoidPtr,
) {
    let idx: NSUInteger = idx.try_into().unwrap();
    let value: id = value.cast().cast_mut();

    msg![env; array replaceObjectAtIndex:idx withObject:value]
}

fn CFArrayRemoveValueAtIndex(env: &mut Environment, array: CFMutableArrayRef, idx: CFIndex) {
    let idx: NSUInteger = idx.try_into().unwrap();
    msg![env; array removeObjectAtIndex:idx]
}

fn CFArrayRemoveAllValues(env: &mut Environment, array: CFMutableArrayRef) {
    msg![env; array removeAllObjects]
}

fn CFArrayContainsValue(
    env: &mut Environment,
    array: CFArrayRef,
    range: super::CFRange,
    value: ConstVoidPtr,
) -> bool {
    CFArrayGetFirstIndexOfValue(env, array, range, value) != -1
}

fn CFArrayGetFirstIndexOfValue(
    env: &mut Environment,
    array: CFArrayRef,
    range: super::CFRange,
    value: ConstVoidPtr,
) -> CFIndex {
    let value: id = value.cast().cast_mut();
    let idx: NSUInteger = msg![
        env;
        array indexOfObject:value
        inRange:range
    ];

    if idx == NSUInteger::MAX {
        -1
    } else {
        idx.try_into().unwrap()
    }
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFArrayCreateMutable(_, _, _)),
    export_c_func!(CFArrayCreate(_, _, _, _)),
    export_c_func!(CFArrayCreateCopy(_, _)),
    export_c_func!(CFArrayCreateMutableCopy(_, _, _)),
    export_c_func!(CFArrayGetCount(_)),
    export_c_func!(CFArrayGetValueAtIndex(_, _)),
    export_c_func!(CFArrayGetValues(_, _, _)),
    export_c_func!(CFArrayAppendValue(_, _)),
    export_c_func!(CFArrayInsertValueAtIndex(_, _, _)),
    export_c_func!(CFArraySetValueAtIndex(_, _, _)),
    export_c_func!(CFArrayRemoveValueAtIndex(_, _)),
    export_c_func!(CFArrayRemoveAllValues(_)),
    export_c_func!(CFArrayContainsValue(_, _, _)),
    export_c_func!(CFArrayGetFirstIndexOfValue(_, _, _)),
];


