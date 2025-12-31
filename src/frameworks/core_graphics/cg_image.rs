/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CGImage.h`

use super::cg_color_space::{
    kCGColorSpaceGenericRGB, CGColorSpaceCreateWithName, CGColorSpaceGetModel, CGColorSpaceRef,
};
use super::cg_data_provider::{self, CGDataProviderRef};
use super::{CGFloat, CGRect};
use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::core_foundation::{CFRelease, CFRetain, CFTypeRef};
use crate::frameworks::foundation::ns_string;
use crate::image::Image;
use crate::mem::{ConstPtr, GuestUSize};
use crate::objc::{autorelease, nil, objc_classes, ClassExports, HostObject, ObjC};
use crate::Environment;

pub type CGImageAlphaInfo = u32;
pub const kCGImageAlphaNone: CGImageAlphaInfo = 0;
pub const kCGImageAlphaPremultipliedLast: CGImageAlphaInfo = 1;
pub const kCGImageAlphaPremultipliedFirst: CGImageAlphaInfo = 2;
pub const kCGImageAlphaLast: CGImageAlphaInfo = 3;
pub const kCGImageAlphaFirst: CGImageAlphaInfo = 4;
pub const kCGImageAlphaNoneSkipLast: CGImageAlphaInfo = 5;
pub const kCGImageAlphaNoneSkipFirst: CGImageAlphaInfo = 6;
pub const kCGImageAlphaOnly: CGImageAlphaInfo = 7;

pub type CGImageByteOrderInfo = u32;
pub const kCGImageByteOrderMask: CGImageByteOrderInfo = 0x7000;
pub const kCGImageByteOrderDefault: CGImageByteOrderInfo = 0 << 12;
#[allow(dead_code)]
pub const kCGImageByteOrder16Little: CGImageByteOrderInfo = 1 << 12;
#[allow(dead_code)]
pub const kCGImageByteOrder32Little: CGImageByteOrderInfo = 2 << 12;
#[allow(dead_code)]
pub const kCGImageByteOrder16Big: CGImageByteOrderInfo = 3 << 12;
pub const kCGImageByteOrder32Big: CGImageByteOrderInfo = 4 << 12;

pub type CGBitmapInfo = u32;
pub const kCGBitmapAlphaInfoMask: CGBitmapInfo = 0x1F; // huh, it's not 0x7?
pub const kCGBitmapByteOrderMask: CGBitmapInfo = kCGImageByteOrderMask;
// TODO: other stuff in this enum (for now, always assert the rest is 0)

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// CGImage seems to be a CFType-based type, but in our implementation those
// are just Objective-C types, so we need a class for it, but its name is not
// visible anywhere.
@implementation _touchHLE_CGImage: NSObject
@end

};

struct CGImageHostObject {
    image: Image,
}
impl HostObject for CGImageHostObject {}

pub type CGImageRef = CFTypeRef;
pub fn CGImageRelease(env: &mut Environment, c: CGImageRef) {
    if !c.is_null() {
        CFRelease(env, c);
    }
}
pub fn CGImageRetain(env: &mut Environment, c: CGImageRef) -> CGImageRef {
    if !c.is_null() {
        CFRetain(env, c)
    } else {
        c
    }
}

/// Shortcut for use by `UIImage`: directly construct a `CGImage` instance from
/// an [Image] instance.
pub fn from_image(env: &mut Environment, image: Image) -> CGImageRef {
    let host_obj = Box::new(CGImageHostObject { image });
    let class = env.objc.get_known_class("_touchHLE_CGImage", &mut env.mem);
    env.objc.alloc_object(class, host_obj, &mut env.mem)
}

/// Shortcut for use by `CGBitmapContext` etc: borrow the [Image] from a
/// `CGImage` instance.
pub fn borrow_image(objc: &ObjC, image: CGImageRef) -> &Image {
    &objc.borrow::<CGImageHostObject>(image).image
}

/// Shortcut used by the app picker, counterpart to [borrow_image].
/// FIXME: This should not exist!
pub fn borrow_image_mut(objc: &mut ObjC, image: CGImageRef) -> &mut Image {
    &mut objc.borrow_mut::<CGImageHostObject>(image).image
}

// TODO: More create methods.

fn CGImageCreateCopyWithColorSpace(
    env: &mut Environment,
    image: CGImageRef,
    color_space: CGColorSpaceRef,
) -> CGImageRef {
    let image_color_space = CGImageGetColorSpace(env, image);
    assert_eq!(
        CGColorSpaceGetModel(env, image_color_space),
        CGColorSpaceGetModel(env, color_space)
    );
    // If color space matches, we could just create a copy.
    let new_image = env.objc.borrow::<CGImageHostObject>(image).image.clone();
    from_image(env, new_image)
}

fn CGImageCreateWithPNGDataProvider(
    env: &mut Environment,
    source: CGDataProviderRef,
    decode: ConstPtr<CGFloat>,
    _should_interpolate: bool, // TODO
    _intent: i32,              // TODO (should be CGColorRenderingIntent)
) -> CGImageRef {
    assert!(decode.is_null()); // TODO

    let bytes = cg_data_provider::borrow_bytes(env, source);
    let Ok(image) = Image::from_bytes(bytes) else {
        // Docs don't say what happens on failure, but this would make sense.
        return nil;
    };

    from_image(env, image)
}

fn CGImageCreateWithJPEGDataProvider(
    env: &mut Environment,
    source: CGDataProviderRef,
    decode: ConstPtr<CGFloat>,
    _should_interpolate: bool, // TODO
    _intent: i32,              // TODO (should be CGColorRenderingIntent)
) -> CGImageRef {
    assert!(decode.is_null());

    let bytes = cg_data_provider::borrow_bytes(env, source);
    let Ok(image) = Image::from_bytes(bytes) else {
        // Docs don't say what happens on failure, but this would make sense.
        return nil;
    };

    from_image(env, image)
}

fn CGImageGetAlphaInfo(_env: &mut Environment, _image: CGImageRef) -> CGImageAlphaInfo {
    // our Image type always returns premultiplied RGBA
    // (the premultiplied part must match what the real UIImage does, but
    // considering CgBI's design, maybe the order doesn't?)
    kCGImageAlphaPremultipliedLast
}

fn CGImageGetColorSpace(env: &mut Environment, _image: CGImageRef) -> CGColorSpaceRef {
    // Caller must release
    // FIXME: what if a loaded image is not sRGB?

    let srgb_name = ns_string::get_static_str(env, kCGColorSpaceGenericRGB);
    CGColorSpaceCreateWithName(env, srgb_name)
}

pub fn CGImageGetWidth(env: &mut Environment, image: CGImageRef) -> GuestUSize {
    let (width, _height) = env
        .objc
        .borrow::<CGImageHostObject>(image)
        .image
        .dimensions();
    width
}
pub fn CGImageGetHeight(env: &mut Environment, image: CGImageRef) -> GuestUSize {
    let (_width, height) = env
        .objc
        .borrow::<CGImageHostObject>(image)
        .image
        .dimensions();
    height
}
fn CGImageGetBitsPerPixel(_env: &mut Environment, _image: CGImageRef) -> GuestUSize {
    32
}
fn CGImageGetBytesPerRow(env: &mut Environment, image: CGImageRef) -> GuestUSize {
    let (width, _height) = env
        .objc
        .borrow::<CGImageHostObject>(image)
        .image
        .dimensions();
    width * 4
}

fn CGImageGetDataProvider(env: &mut Environment, image: CGImageRef) -> CGDataProviderRef {
    // CGImageGetDataProvider() seems to be intended to return the underlying
    // data provider that is retained by the CGImage. That's not how CGImage is
    // implemented here though, so instead we make a data provider that
    // retains the CGImage: exactly the opposite approach!
    let cg_data_provider = cg_data_provider::from_cg_image(env, image);
    // CGImageGetDataProvider() isn't meant to return a new object, so the
    // caller won't free this. The CGImage can't retain the CGDataProvider
    // without causing a cycle, so let's autorelease it instead.
    autorelease(env, cg_data_provider)
}

fn CGImageGetBitsPerComponent(_: &mut Environment, _: CGImageRef) -> GuestUSize {
    8 // Fix this when we support anything else
}

fn CGImageCreateCopy(
    env: &mut Environment,
    image: CGImageRef,
) -> CGImageRef {
    if image.is_null() {
        return nil;
    }

    let new_image = env
        .objc
        .borrow::<CGImageHostObject>(image)
        .image
        .clone();

    from_image(env, new_image)
}

fn CGImageCreateCopyWithAlpha(
    env: &mut Environment,
    image: CGImageRef,
    alpha_info: CGImageAlphaInfo,
) -> CGImageRef {
    assert!(
        alpha_info == kCGImageAlphaPremultipliedLast
            || alpha_info == kCGImageAlphaLast
            || alpha_info == kCGImageAlphaNoneSkipLast
    );

    // Your Image is always RGBA premultiplied; just clone.
    let new_image = env
        .objc
        .borrow::<CGImageHostObject>(image)
        .image
        .clone();

    from_image(env, new_image)
}

fn CGImageIsMask(_env: &mut Environment, _image: CGImageRef) -> bool {
    false
}

fn CGImageGetBitmapInfo(
    _env: &mut Environment,
    _image: CGImageRef,
) -> CGBitmapInfo {
    // RGBA, premultiplied last, default byte order
    kCGImageAlphaPremultipliedLast | kCGImageByteOrderDefault
}

fn CGImageGetRenderingIntent(
    _env: &mut Environment,
    _image: CGImageRef,
) -> i32 {
    // kCGRenderingIntentDefault
    0
}

fn CGImageGetShouldInterpolate(
    _env: &mut Environment,
    _image: CGImageRef,
) -> bool {
    true
}

fn CGImageCreateWithImageInRect(
    env: &mut Environment,
    image: CGImageRef,
    rect: CGRect,
) -> CGImageRef {
    if image.is_null() {
        return nil;
    }

    let src = &env.objc.borrow::<CGImageHostObject>(image).image;

    let x = rect.origin.x as u32;
    let y = rect.origin.y as u32;
    let w = rect.size.width as u32;
    let h = rect.size.height as u32;

    let cropped = src.crop(x, y, w, h);
    from_image(env, cropped)
}

fn CGImageCreate(
    env: &mut Environment,
    width: GuestUSize,
    height: GuestUSize,
    bits_per_component: GuestUSize,
    bits_per_pixel: GuestUSize,
    bytes_per_row: GuestUSize,
    color_space: CGColorSpaceRef,
    bitmap_info: CGBitmapInfo,
    provider: CGDataProviderRef,
    decode: ConstPtr<CGFloat>,
    _should_interpolate: bool,
    _intent: i32,
) -> CGImageRef {
    // Validate supported format
    if bits_per_component != 8 || bits_per_pixel != 32 {
        return nil;
    }
    if !decode.is_null() {
        return nil;
    }

    let model = CGColorSpaceGetModel(env, color_space);
    if model != super::cg_color_space::kCGColorSpaceModelRGB {
        return nil;
    }

    // Load bytes
    let data = cg_data_provider::borrow_bytes(env, provider);
    let expected = (height * bytes_per_row) as usize;
    if data.len() < expected {
        return nil;
    }

    // Convert to Image
    let image = Image::from_rgba_bytes(
        width as u32,
        height as u32,
        bytes_per_row as usize,
        &data[..expected],
    );

    from_image(env, image)
}

fn CGImageCreateMask(
    env: &mut Environment,
    width: GuestUSize,
    height: GuestUSize,
    _bits_per_component: GuestUSize,
    _bits_per_pixel: GuestUSize,
    bytes_per_row: GuestUSize,
    provider: CGDataProviderRef,
    _decode: ConstPtr<CGFloat>,
    _should_interpolate: bool,
) -> CGImageRef {
    let bytes = cg_data_provider::borrow_bytes(env, provider);

    let image = Image::from_alpha_mask(
        width as u32,
        height as u32,
        bytes_per_row as usize,
        bytes,
    );

    from_image(env, image)
}

fn CGImageCreateWithMask(
    env: &mut Environment,
    image: CGImageRef,
    mask: CGImageRef,
) -> CGImageRef {
    if image.is_null() || mask.is_null() {
        return nil;
    }

    let src = borrow_image(&env.objc, image);
    let mask_img = borrow_image(&env.objc, mask);

    let (w, h) = src.dimensions();
    if mask_img.dimensions() != (w, h) {
        return nil;
    }

    let src_pixels = src.pixels();
    let mask_pixels = mask_img.pixels();

    let mut out = Vec::with_capacity(src_pixels.len());

    for i in (0..src_pixels.len()).step_by(4) {
        let r = src_pixels[i];
        let g = src_pixels[i + 1];
        let b = src_pixels[i + 2];
        let a = src_pixels[i + 3];

        let ma = mask_pixels[i + 3] as u16;
        let new_a = (a as u16 * ma / 255) as u8;

        out.extend_from_slice(&[r, g, b, new_a]);
    }

    let out_img = Image::from_pixel_vec(out, (w as u32, h as u32));
    from_image(env, out_img)
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CGImageRelease(_)),
    export_c_func!(CGImageRetain(_)),
    export_c_func!(CGImageCreateCopy(_)),
    export_c_func!(CGImageCreateCopyWithAlpha(_, _)),
    export_c_func!(CGImageCreateCopyWithColorSpace(_, _)),
    export_c_func!(CGImageCreateWithPNGDataProvider(_, _, _, _)),
    export_c_func!(CGImageCreateWithJPEGDataProvider(_, _, _, _)),
    export_c_func!(CGImageCreateWithImageInRect(_, _)),
    export_c_func!(CGImageGetAlphaInfo(_)),
    export_c_func!(CGImageGetBitmapInfo(_)),
    export_c_func!(CGImageGetColorSpace(_)),
    export_c_func!(CGImageGetWidth(_)),
    export_c_func!(CGImageGetHeight(_)),
    export_c_func!(CGImageGetBitsPerPixel(_)),
    export_c_func!(CGImageGetBitsPerComponent(_)),
    export_c_func!(CGImageGetBytesPerRow(_)),
    export_c_func!(CGImageGetDataProvider(_)),
    export_c_func!(CGImageGetRenderingIntent(_)),
    export_c_func!(CGImageGetShouldInterpolate(_)),
    export_c_func!(CGImageIsMask(_)),
    export_c_func!(CGImageCreate(_, _, _, _, _, _, _, _, _, _, _)),
    export_c_func!(CGImageCreateMask(_, _, _, _, _, _, _, _)),
    export_c_func!(CGImageCreateWithMask(_, _)),
];
