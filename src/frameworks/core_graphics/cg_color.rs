/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CGColor.h`

use std::ops::{Add, Mul, Sub};

use crate::dyld::{export_c_func, ConstantExports, FunctionExports, HostConstant};
use crate::frameworks::core_foundation::{CFRelease, CFRetain, CFTypeRef};
use crate::frameworks::core_graphics::cg_color_space::{
    kCGColorSpaceGenericRGB, CGColorSpaceHostObject, CGColorSpaceCreateWithName, CGColorSpaceRef,
};
use crate::frameworks::core_graphics::CGFloat;
use crate::frameworks::foundation::ns_string::to_rust_string;
use crate::mem::{guest_size_of, MutPtr};
use crate::objc::{objc_classes, ClassExports, HostObject, ObjC};
use crate::Environment;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// CGColor seems to be a CFType-based type, but in our implementation
// those are just Objective-C types, so we need a class for it, but its name is
// not visible anywhere.
@implementation _touchHLE_CGColor: NSObject
@end

};

#[derive(Copy, Clone)]
pub struct CGColorHostObject {
    pub color_space_name: &'static str,
    // this assumes usage of CGColorSpaceGenericRGB
    // TODO: support other color spaces
    pub r: CGFloat,
    pub g: CGFloat,
    pub b: CGFloat,
    pub a: CGFloat,
}
impl HostObject for CGColorHostObject {}
// Implemented to aid animation code.
// Theres are the operations needed for the interpolation.
impl Mul<f32> for CGColorHostObject {
    type Output = CGColorHostObject;

    fn mul(self, rhs: f32) -> Self::Output {
        CGColorHostObject {
            color_space_name: self.color_space_name,
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}
impl Add<CGColorHostObject> for CGColorHostObject {
    type Output = CGColorHostObject;

    fn add(self, rhs: CGColorHostObject) -> Self::Output {
        CGColorHostObject {
            color_space_name: self.color_space_name,
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}
impl Sub<CGColorHostObject> for CGColorHostObject {
    type Output = CGColorHostObject;

    fn sub(self, rhs: CGColorHostObject) -> Self::Output {
        CGColorHostObject {
            color_space_name: self.color_space_name,
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

pub type CGColorRef = CFTypeRef;

pub struct CGColorState {
    pub constant_white: Option<CGColorRef>,
    pub constant_black: Option<CGColorRef>,
    pub constant_clear: Option<CGColorRef>,
}

pub fn CGColorRelease(env: &mut Environment, c: CGColorRef) {
    if !c.is_null() {
        CFRelease(env, c);
    }
}
pub fn CGColorRetain(env: &mut Environment, c: CGColorRef) -> CGColorRef {
    if !c.is_null() {
        CFRetain(env, c)
    } else {
        c
    }
}

fn CGColorCreate(
    env: &mut Environment,
    space: CGColorSpaceRef,
    components: MutPtr<CGFloat>,
) -> CGColorRef {
    let color_space = env.objc.borrow::<CGColorSpaceHostObject>(space).name;
    assert_eq!(color_space, kCGColorSpaceGenericRGB);
    let r = env.mem.read(components);
    let g = env.mem.read(components + 1);
    let b = env.mem.read(components + 2);
    let a = env.mem.read(components + 3);
    from_rgba(env, (r, g, b, a))
}

fn CGColorCreateGenericRGB(
    env: &mut Environment,
    r: CGFloat,
    g: CGFloat,
    b: CGFloat,
    a: CGFloat,
) -> CGColorRef {
    from_rgba(env, (r, g, b, a))
}

fn CGColorGetComponents(
    env: &mut Environment,
    color: CGColorRef,
) -> MutPtr<CGFloat> {
    let (r, g, b, a) = to_rgba(&env.objc, color);

    let ptr: MutPtr<CGFloat> = env.mem
        .alloc(guest_size_of::<CGFloat>() * 4)
        .cast();

    env.mem.write(ptr + 0, r);
    env.mem.write(ptr + 1, g);
    env.mem.write(ptr + 2, b);
    env.mem.write(ptr + 3, a);

    ptr
}

fn CGColorGetNumberOfComponents(
    _env: &mut Environment,
    _color: CGColorRef,
) -> u32 {
    4
}

fn CGColorGetAlpha(
    env: &mut Environment,
    color: CGColorRef,
) -> CGFloat {
    if color.is_null() {
        return 0.0;
    }
    env.objc.borrow::<CGColorHostObject>(color).a
}

fn CGColorGetColorSpace(
    env: &mut Environment,
    color: CGColorRef,
) -> CGColorSpaceRef {
    if color.is_null() {
        return MutPtr::null();
    }

    let space_name =
        env.objc.borrow::<CGColorHostObject>(color).color_space_name;

    // Currently only GenericRGB is supported
    assert_eq!(space_name, kCGColorSpaceGenericRGB);

    // Caller owns the returned object (Create rule)
    let name = crate::frameworks::foundation::ns_string::get_static_str(
        env,
        kCGColorSpaceGenericRGB,
    );

    CGColorSpaceCreateWithName(env, name)
}

fn CGColorEqualToColor(
    env: &mut Environment,
    c1: CGColorRef,
    c2: CGColorRef,
) -> bool {
    if c1 == c2 {
        return true;
    }
    if c1.is_null() || c2.is_null() {
        return false;
    }

    let a = env.objc.borrow::<CGColorHostObject>(c1);
    let b = env.objc.borrow::<CGColorHostObject>(c2);

    a.color_space_name == b.color_space_name
        && a.r == b.r
        && a.g == b.g
        && a.b == b.b
        && a.a == b.a
}

fn CGColorGetConstantColor(
    env: &mut Environment,
    name: CFTypeRef,
) -> CGColorRef {
    if name.is_null() {
        return MutPtr::null();
    }

    let name = to_rust_string(env, name);

    match &name[..] {
        "kCGColorWhite" => from_rgba(env, (1.0, 1.0, 1.0, 1.0)),
        "kCGColorBlack" => from_rgba(env, (0.0, 0.0, 0.0, 1.0)),
        "kCGColorClear" => from_rgba(env, (0.0, 0.0, 0.0, 0.0)),
        _ => MutPtr::null(),
    }
}

pub const kCGColorWhite: &str = "kCGColorWhite";
pub const kCGColorBlack: &str = "kCGColorBlack";
pub const kCGColorClear: &str = "kCGColorClear";

pub const CONSTANTS: ConstantExports = &[
    (
        "_kCGColorWhite",
        HostConstant::NSString(kCGColorWhite),
    ),
    (
        "_kCGColorBlack",
        HostConstant::NSString(kCGColorBlack),
    ),
    (
        "_kCGColorClear",
        HostConstant::NSString(kCGColorClear),
    ),
];

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CGColorRetain(_)),
    export_c_func!(CGColorRelease(_)),
    export_c_func!(CGColorCreate(_, _)),
    export_c_func!(CGColorCreateGenericRGB(_, _, _, _)),
    export_c_func!(CGColorGetComponents(_)),
    export_c_func!(CGColorGetNumberOfComponents(_)),
    export_c_func!(CGColorGetAlpha(_)),
    export_c_func!(CGColorGetColorSpace(_)),
    export_c_func!(CGColorEqualToColor(_, _)),
    export_c_func!(CGColorGetConstantColor(_)),
];

/// Shortcut for use by `UIColor`: directly construct a `CGColor` instance from
/// an rgba tuple of CGFloats.
pub fn from_rgba(env: &mut Environment, rgba: (CGFloat, CGFloat, CGFloat, CGFloat)) -> CGColorRef {
    let (r, g, b, a) = rgba;
    let host_obj = Box::new(CGColorHostObject {
        color_space_name: kCGColorSpaceGenericRGB,
        r,
        g,
        b,
        a,
    });
    let class = env.objc.get_known_class("_touchHLE_CGColor", &mut env.mem);
    env.objc.alloc_object(class, host_obj, &mut env.mem)
}

/// Shortcut for use by `UIColor`
pub fn to_rgba(objc: &ObjC, color: CGColorRef) -> (CGFloat, CGFloat, CGFloat, CGFloat) {
    let &CGColorHostObject {
        color_space_name,
        r,
        g,
        b,
        a,
        ..
    } = objc.borrow(color);
    assert_eq!(color_space_name, kCGColorSpaceGenericRGB);
    (r, g, b, a)
}
