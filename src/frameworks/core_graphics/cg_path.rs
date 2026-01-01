use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::core_foundation::{CFRelease, CFRetain};
use crate::mem::Ptr;
use crate::objc::{objc_classes, ClassExports};
use crate::Environment;
use std::ffi::c_void;

pub type CGPathRef = Ptr<c_void, false>;
pub type CGMutablePathRef = Ptr<c_void, true>

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation _touchHLE_CGPath: NSObject
@end
};

fn CGPathCreateMutable(env: &mut Environment) -> CGMutablePathRef {
    let host_obj = Box::new(CGPathHostObject {
        elements: Vec::new(),
    });

    let class = env
        .objc
        .get_known_class("_touchHLE_CGPath", &mut env.mem);

    env.objc
        .alloc_object(class, host_obj, &mut env.mem)
        .cast::<c_void>()
        .cast_mut()
}

pub fn CGPathRetain(env: &mut Environment, path: CGPathRef) -> CGPathRef {
    if !path.is_null() {
        CFRetain(env, path.cast());
    }
    path
}

pub fn CGPathRelease(env: &mut Environment, path: CGPathRef) {
    if !path.is_null() {
        CFRelease(env, path.cast());
    }
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CGPathCreateMutable()),
    export_c_func!(CGPathRetain(_)),
    export_c_func!(CGPathRelease(_)),
];
