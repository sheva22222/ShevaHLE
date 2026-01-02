use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::core_foundation::{CFRelease, CFRetain};
use crate::frameworks::core_graphics::{CGFloat, CGPoint, CGRect};
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
        .cast()
}

pub fn CGPathRetain(env: &mut Environment, path: CGPathRef) -> CGPathRef {
    if !path.is_null() {
        CFRetain(env, path.cast().cast_mut());
    }
    path
}


pub fn CGPathRelease(env: &mut Environment, path: CGPathRef) {
    if !path.is_null() {
        CFRelease(env, path.cast().cast_mut());
    }
}

pub fn CGPathAddLines(
    env: &mut Environment,
    path: CGMutablePathRef,
    transform: ConstPtr<CGAffineTransform>,
    points: ConstPtr<CGPoint>,
    count: u32,
) {
    if path.is_null() || points.is_null() || count == 0 {
        return;
    }

    let host = env
        .objc
        .borrow_mut::<CGPathHostObject>(path.cast());

    let t = if transform.is_null() {
        CGAffineTransform {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    } else {
        env.mem.read(transform)
    };

    // First point → MoveTo
    let p0 = t.apply_to_point(env.mem.read(points));
    host.elements.push(PathElement::MoveTo(p0));

    // Remaining points → LineTo
    for i in 1..count {
        let p = t.apply_to_point(env.mem.read(points + i));
        host.elements.push(PathElement::LineTo(p));
    }
}

pub fn CGPathCloseSubpath(
    env: &mut Environment,
    path: CGMutablePathRef,
) {
    if path.is_null() {
        return;
    }

    let host = env
        .objc
        .borrow_mut::<CGPathHostObject>(path.cast());

    host.elements.push(PathElement::CloseSubpath);
}

pub fn CGPathAddRect(
    env: &mut Environment,
    path: CGMutablePathRef,
    transform: ConstPtr<CGAffineTransform>,
    rect: CGRect,
) {
    if path.is_null() {
        return;
    }

    let host = env
        .objc
        .borrow_mut::<CGPathHostObject>(path.cast());

    let t = if transform.is_null() {
        CGAffineTransform {
            a: 1.0, b: 0.0,
            c: 0.0, d: 1.0,
            tx: 0.0, ty: 0.0,
        }
    } else {
        env.mem.read(transform)
    };

    let min_x = rect.origin.x;
    let min_y = rect.origin.y;
    let max_x = rect.origin.x + rect.size.width;
    let max_y = rect.origin.y + rect.size.height;

    let p0 = t.apply_to_point(CGPoint { x: min_x, y: min_y });
    let p1 = t.apply_to_point(CGPoint { x: max_x, y: min_y });
    let p2 = t.apply_to_point(CGPoint { x: max_x, y: max_y });
    let p3 = t.apply_to_point(CGPoint { x: min_x, y: max_y });

    host.elements.push(PathElement::MoveTo(p0));
    host.elements.push(PathElement::LineTo(p1));
    host.elements.push(PathElement::LineTo(p2));
    host.elements.push(PathElement::LineTo(p3));
    host.elements.push(PathElement::CloseSubpath);
}

pub fn CGPathCreateCopy(
    env: &mut Environment,
    path: CGPathRef,
) -> CGPathRef {
    if path.is_null() {
        return path;
    }

    // ⬇️ Step 1: cast immutable CGPathRef → mutable ObjC id
    let src = env
        .objc
        .borrow::<CGPathHostObject>(path.cast_mut());

    // ⬇️ Step 2: clone elements
    let host_obj = Box::new(CGPathHostObject {
        elements: src.elements.clone(),
    });

    let class = env
        .objc
        .get_known_class("_touchHLE_CGPath", &mut env.mem);

    // ⬇️ Step 3: alloc gives mutable ObjC pointer
    let new_obj = env.objc.alloc_object(class, host_obj, &mut env.mem);

    // ⬇️ Step 4: return as immutable CGPathRef
    new_obj.cast()
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CGPathCreateMutable()),
    export_c_func!(CGPathRetain(_)),
    export_c_func!(CGPathRelease(_)),
    export_c_func!(CGPathAddLines(_, _, _, _)),
    export_c_func!(CGPathCloseSubpath(_)),
    export_c_func!(CGPathAddRect(_, _, _)),
    export_c_func!(CGPathCreateCopy(_)),
];
