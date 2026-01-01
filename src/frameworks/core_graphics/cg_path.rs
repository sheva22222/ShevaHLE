use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::core_foundation::{CFRelease, CFRetain};
use crate::frameworks::core_graphics::{CGFloat, CGPoint};
use crate::frameworks::core_graphics::cg_affine_transform::CGAffineTransform;
use crate::mem::{ConstPtr, Ptr};
use crate::objc::{objc_classes, ClassExports, HostObject};
use crate::Environment;
use std::ffi::c_void;

pub type CGPathRef = Ptr<c_void, false>;
pub type CGMutablePathRef = Ptr<c_void, true>;

#[derive(Default)]
pub struct CGPathHostObject {
    pub elements: Vec<PathElement>,
}

impl HostObject for CGPathHostObject {}

#[derive(Clone)]
pub enum PathElement {
    MoveTo(CGPoint),
    LineTo(CGPoint),
    QuadCurveTo(CGPoint, CGPoint),
    CurveTo(CGPoint, CGPoint, CGPoint),
    CloseSubpath,
}

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
        CFRetain(env, path.cast::<objc_object>().cast_mut());
    }
    path
}

pub fn CGPathRelease(env: &mut Environment, path: CGPathRef) {
    if !path.is_null() {
        CFRelease(env, path.cast::<objc_object>().cast_mut());
    }
}

fn CGPathMoveToPoint(
    env: &mut Environment,
    path: CGMutablePathRef,
    _transform: ConstPtr<CGAffineTransform>,
    x: CGFloat,
    y: CGFloat,
) {
    let host = env
        .objc
        .borrow_mut::<CGPathHostObject>(path);

    host.elements.push(PathElement::MoveTo(CGPoint { x, y }));
}

fn CGPathAddLineToPoint(
    env: &mut Environment,
    path: CGMutablePathRef,
    _transform: ConstPtr<CGAffineTransform>,
    x: CGFloat,
    y: CGFloat,
) {
    let host = env
        .objc
        .borrow_mut::<CGPathHostObject>(path);

    host.elements.push(PathElement::LineTo(CGPoint { x, y }));
}

fn CGPathAddCurveToPoint(
    env: &mut Environment,
    path: CGMutablePathRef,
    _transform: ConstPtr<CGAffineTransform>,
    cp1x: CGFloat,
    cp1y: CGFloat,
    cp2x: CGFloat,
    cp2y: CGFloat,
    x: CGFloat,
    y: CGFloat,
) {
    let host = env
        .objc
        .borrow_mut::<CGPathHostObject>(path);

    host.elements.push(PathElement::CurveTo(
        CGPoint { x: cp1x, y: cp1y },
        CGPoint { x: cp2x, y: cp2y },
        CGPoint { x, y },
    ));
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CGPathCreateMutable()),
    export_c_func!(CGPathRetain(_)),
    export_c_func!(CGPathRelease(_)),
    export_c_func!(CGPathMoveToPoint(_, _, _, _)),
    export_c_func!(CGPathAddLineToPoint(_, _, _, _)),
    export_c_func!(CGPathAddCurveToPoint(_, _, _, _, _, _, _)),
];
