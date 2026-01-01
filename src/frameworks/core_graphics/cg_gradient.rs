
use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::core_foundation::cf_array::{CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef};
use crate::frameworks::core_foundation::{CFRelease, CFRetain, CFTypeRef};
use crate::frameworks::core_graphics::CGFloat;
use crate::frameworks::core_graphics::cg_color::{CGColorHostObject, CGColorRef, to_rgba};
use crate::frameworks::core_graphics::cg_color_space::{CGColorSpaceHostObject, CGColorSpaceRef, kCGColorSpaceGenericRGB};
use crate::objc::{objc_classes, ClassExports, HostObject, ObjC};
use crate::mem::ConstPtr;
use crate::Environment;

pub struct CGGradientHostObject {
    pub color_space_name: &'static str,
    pub colors: Vec<CGColorHostObject>,
    pub locations: Option<Vec<CGFloat>>,
}

impl HostObject for CGGradientHostObject {}

pub type CGGradientRef = CFTypeRef;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation _touchHLE_CGGradient: NSObject
@end

fn CGGradientCreateWithColors(
    env: &mut Environment,
    space: CGColorSpaceRef,
    colors: CFArrayRef,
    locations: ConstPtr<CGFloat>,
) -> CGGradientRef {
    let space_name = env
        .objc
        .borrow::<crate::frameworks::core_graphics::cg_color_space::CGColorSpaceHostObject>(space)
        .name;

    // touchHLE limitation: only GenericRGB supported
    assert_eq!(space_name, kCGColorSpaceGenericRGB);

    let count = CFArrayGetCount(env, colors) as usize;
    assert!(count > 0);

    // Copy colors
    let mut out_colors = Vec::with_capacity(count);
    for i in 0..count {
        let color_ref: CGColorRef =
            CFArrayGetValueAtIndex(env, colors, i as i32)
                .cast()
                .cast_mut();

        let (r, g, b, a) = to_rgba(&env.objc, color_ref);

        out_colors.push(CGColorHostObject {
            color_space_name: kCGColorSpaceGenericRGB,
            r,
            g,
            b,
            a,
        });
    }

    // Copy locations if provided
    let out_locations = if locations.is_null() {
        None
    } else {
        let mut v = Vec::with_capacity(count);
        for i in 0..count {
            v.push(env.mem.read(locations + (i as u32)));
        }
        Some(v)
    };

    let host_obj = Box::new(CGGradientHostObject {
        color_space_name: kCGColorSpaceGenericRGB,
        colors: out_colors,
        locations: out_locations,
    });

    let class = env
        .objc
        .get_known_class("_touchHLE_CGGradient", &mut env.mem);

    env.objc.alloc_object(class, host_obj, &mut env.mem)
}

fn CGGradientCreateWithColorComponents(
    env: &mut Environment,
    space: CGColorSpaceRef,
    components: ConstPtr<CGFloat>,
    locations: ConstPtr<CGFloat>,
    count: u32,
) -> CGGradientRef {
    let count: usize = count.try_into().unwrap();

    let space_name = env
        .objc
        .borrow::<crate::frameworks::core_graphics::cg_color_space::CGColorSpaceHostObject>(space)
        .name;

    assert_eq!(space_name, kCGColorSpaceGenericRGB);
    assert!(count > 0);
    assert!(!components.is_null());

    // Copy colors
    let mut out_colors = Vec::with_capacity(count);
    for i in 0..count {
        let base = (i * 4) as u32;

        let r = env.mem.read(components + base);
        let g = env.mem.read(components + base + 1);
        let b = env.mem.read(components + base + 2);
        let a = env.mem.read(components + base + 3);

        out_colors.push(CGColorHostObject {
            color_space_name: kCGColorSpaceGenericRGB,
            r,
            g,
            b,
            a,
        });
    }

    let out_locations = if locations.is_null() {
        None
    } else {
        let mut v = Vec::with_capacity(count);
        for i in 0..count {
            v.push(env.mem.read(locations + (i as u32)));
        }
        Some(v)
    };

    let host_obj = Box::new(CGGradientHostObject {
        color_space_name: kCGColorSpaceGenericRGB,
        colors: out_colors,
        locations: out_locations,
    });

    let class = env
        .objc
        .get_known_class("_touchHLE_CGGradient", &mut env.mem);

    env.objc.alloc_object(class, host_obj, &mut env.mem)
}

pub fn CGGradientRelease(env: &mut Environment, gradient: CGGradientRef) {
    if !gradient.is_null() {
        CFRelease(env, gradient);
    }
}

pub fn CGGradientRetain(env: &mut Environment, gradient: CGGradientRef) -> CGGradientRef {
    if !gradient.is_null() {
        CFRetain(env, gradient)
    } else {
        gradient
    }
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CGGradientCreateWithColors(_, _, _)),
    export_c_func!(CGGradientCreateWithColorComponents(_, _, _, _)),
    export_c_func!(CGGradientRetain(_)),
    export_c_func!(CGGradientRelease(_)),
];
