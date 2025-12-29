/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CFType` (type-generic functions etc).

use super::{CFHashCode, CFIndex};
use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::foundation::NSUInteger;
use crate::objc::Class;
use crate::{msg, nil, objc};
use crate::{msg_class, Environment};

pub type CFTypeRef = objc::id;

pub fn CFRetain(env: &mut Environment, object: CFTypeRef) -> CFTypeRef {
    assert!(!object.is_null()); // not allowed, unlike for normal objc objects
    objc::retain(env, object)
}
pub fn CFRelease(env: &mut Environment, object: CFTypeRef) {
    objc::release(env, object);
}

pub fn CFGetRetainCount(env: &mut Environment, object: CFTypeRef) -> CFIndex {
    let count: NSUInteger = msg![env; object retainCount];
    count as CFIndex
}

pub fn CFEqual(env: &mut Environment, object1: CFTypeRef, object2: CFTypeRef) -> bool {
    if object1 == object2 {
        return true;
    }
    // TODO: other classes
    let str_class: Class = msg_class![env; NSString class];
    let object1_class: Class = msg![env; object1 class];
    assert!(msg![env; object1_class isKindOfClass:str_class]);
    let object2_class: Class = msg![env; object2 class];
    assert!(msg![env; object2_class isKindOfClass:str_class]);
    // TODO: use isEqual: once it is fixed
    msg![env; object1 isEqualToString:object2]
}

pub fn CFHash(env: &mut Environment, object: CFTypeRef) -> CFHashCode {
    msg![env; object hash]
}

pub fn CFGetTypeID(_env: &mut Environment, object: CFTypeRef) -> usize {
    if object.is_null() {
        0
    } else {
        // Use class pointer identity as a stable type ID
        unsafe { objc::object_getClass(object) as usize }
    }
}

pub fn CFShow(env: &mut Environment, object: CFTypeRef) {
    if object.is_null() {
        log!("CFShow(NULL)");
        return;
    }
    let desc: CFTypeRef = msg![env; object description];
    log!("CFShow({:?})", desc);
}

pub fn CFCopyDescription(env: &mut Environment, object: CFTypeRef) -> CFTypeRef {
    if object.is_null() {
        return nil
    }
    let desc: CFTypeRef = msg![env; object description];
    msg![env; desc copy]
}

pub fn CFGetAllocator(_env: &mut Environment, _object: CFTypeRef) -> CFTypeRef {
    // Default allocator (treated as NULL/default elsewhere)
    nil
}

pub fn CFMakeCollectable(_env: &mut Environment, object: CFTypeRef) -> CFTypeRef {
    object
}

pub fn CFIsCollectable(_env: &mut Environment, _object: CFTypeRef) -> bool {
    false
}

pub fn CFGetRetainCountNullable(env: &mut Environment, object: CFTypeRef) -> CFIndex {
    if object.is_null() {
        0
    } else {
        let count: NSUInteger = msg![env; object retainCount];
        count as CFIndex
    }
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFRetain(_)),
    export_c_func!(CFRelease(_)),
    export_c_func!(CFGetRetainCount(_)),
    export_c_func!(CFEqual(_, _)),
    export_c_func!(CFHash(_)),
    export_c_func!(CFGetTypeID(_)),
    export_c_func!(CFShow(_)),
    export_c_func!(CFCopyDescription(_)),
    export_c_func!(CFGetAllocator(_)),
    export_c_func!(CFMakeCollectable(_)),
    export_c_func!(CFIsCollectable(_)),
    export_c_func!(CFGetRetainCountNullable(_)),
];
