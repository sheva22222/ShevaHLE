/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CGContext.h`

use super::cg_affine_transform::CGAffineTransform;
use super::cg_image::CGImageRef;
use super::{cg_bitmap_context, CGFloat, CGPoint, CGRect};
use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::core_foundation::{CFRelease, CFRetain, CFTypeRef};
use crate::frameworks::core_graphics::cg_bitmap_context::{
    CGBitmapContextGetHeight, CGBitmapContextGetWidth,
};
use crate::frameworks::core_graphics::cg_geometry::CGPointZero;
use crate::mem::{ConstPtr, GuestUSize, Ptr};
use crate::objc::{objc_classes, ClassExports, HostObject};
use crate::Environment;

type CGInterpolationQuality = i32;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// CGContext seems to be a CFType-based type, but in our implementation those
// are just Objective-C types, so we need a class for it, but its name is not
// visible anywhere.
@implementation _touchHLE_CGContext: NSObject

- (())dealloc {
    let host_obj = env.objc.borrow::<CGContextHostObject>(this);
    let CGContextSubclass::CGBitmapContext(bitmap_data) = host_obj.subclass;
    if bitmap_data.data_is_owned {
        env.mem.free(bitmap_data.data);
    }

    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};

pub(super) struct CGContextHostObject {
    pub(super) subclass: CGContextSubclass,
    pub(super) rgb_fill_color: (CGFloat, CGFloat, CGFloat, CGFloat),
    /// Current transform.
    pub(super) transform: CGAffineTransform,
    // TODO: keep more states saved once they are implemented
    pub(super) state_stack: Vec<((CGFloat, CGFloat, CGFloat, CGFloat), CGAffineTransform)>,
}
impl HostObject for CGContextHostObject {}

pub(super) enum CGContextSubclass {
    CGBitmapContext(cg_bitmap_context::CGBitmapContextData),
}

pub type CGContextRef = CFTypeRef;
type CGBlendMode = i32;
type CGLineCap = i32;
type CGLineJoin = i32;
type CGFontRef = CFTypeRef;
type CGPathDrawingMode = i32;
type CGGlyph = u16;
type CGTextEncoding = i32;
type CGTextDrawingMode = i32;

pub fn CGContextRelease(env: &mut Environment, c: CGContextRef) {
    if !c.is_null() {
        CFRelease(env, c);
    }
}
pub fn CGContextRetain(env: &mut Environment, c: CGContextRef) -> CGContextRef {
    if !c.is_null() {
        CFRetain(env, c)
    } else {
        c
    }
}

pub fn CGContextSetRGBFillColor(
    env: &mut Environment,
    context: CGContextRef,
    red: CGFloat,
    green: CGFloat,
    blue: CGFloat,
    alpha: CGFloat,
) {
    let color = (red, green, blue, alpha);
    env.objc
        .borrow_mut::<CGContextHostObject>(context)
        .rgb_fill_color = color;
}

fn CGContextSetGrayFillColor(
    env: &mut Environment,
    context: CGContextRef,
    gray: CGFloat,
    alpha: CGFloat,
) {
    let color = (gray, gray, gray, alpha);
    env.objc
        .borrow_mut::<CGContextHostObject>(context)
        .rgb_fill_color = color;
}

pub fn CGContextFillRect(env: &mut Environment, context: CGContextRef, rect: CGRect) {
    cg_bitmap_context::fill_rect(env, context, rect, /* clear: */ false);
}

pub fn CGContextClearRect(env: &mut Environment, context: CGContextRef, rect: CGRect) {
    cg_bitmap_context::fill_rect(env, context, rect, /* clear: */ true);
}

fn CGContextClipToRect(env: &mut Environment, context: CGContextRef, rect: CGRect) {
    if rect.origin == CGPointZero
        && rect.size.height == CGBitmapContextGetHeight(env, context) as f32
        && rect.size.width == CGBitmapContextGetWidth(env, context) as f32
    {
        assert!(env
            .objc
            .borrow_mut::<CGContextHostObject>(context)
            .transform
            .is_identity());
        // All good, clipping is not needed!
        return;
    }
    todo!();
}

pub fn CGContextConcatCTM(
    env: &mut Environment,
    context: CGContextRef,
    transform: CGAffineTransform,
) {
    log_dbg!("CGContextConcatCTM({:?})", transform);
    let host_obj = env.objc.borrow_mut::<CGContextHostObject>(context);
    host_obj.transform = transform.concat(host_obj.transform);
}
pub fn CGContextGetCTM(env: &mut Environment, context: CGContextRef) -> CGAffineTransform {
    let res = env.objc.borrow::<CGContextHostObject>(context).transform;
    log_dbg!("CGContextGetCTM() => {:?}", res);
    res
}
pub fn CGContextRotateCTM(env: &mut Environment, context: CGContextRef, angle: CGFloat) {
    log_dbg!("CGContextRotateCTM({:?})", angle);
    let host_obj = env.objc.borrow_mut::<CGContextHostObject>(context);
    host_obj.transform = host_obj.transform.rotate(angle);
}
pub fn CGContextScaleCTM(env: &mut Environment, context: CGContextRef, x: CGFloat, y: CGFloat) {
    log_dbg!("CGContextScaleCTM({:?})", (x, y));
    let host_obj = env.objc.borrow_mut::<CGContextHostObject>(context);
    host_obj.transform = host_obj.transform.scale(x, y);
}
pub fn CGContextTranslateCTM(
    env: &mut Environment,
    context: CGContextRef,
    tx: CGFloat,
    ty: CGFloat,
) {
    log_dbg!("CGContextTranslateCTM({:?})", (tx, ty));
    let host_obj = env.objc.borrow_mut::<CGContextHostObject>(context);
    host_obj.transform = host_obj.transform.translate(tx, ty);
}

pub fn CGContextDrawImage(
    env: &mut Environment,
    context: CGContextRef,
    rect: CGRect,
    image: CGImageRef,
) {
    cg_bitmap_context::draw_image(env, context, rect, image);
}

fn CGContextSaveGState(env: &mut Environment, context: CGContextRef) {
    let host_obj = env.objc.borrow_mut::<CGContextHostObject>(context);
    host_obj
        .state_stack
        .push((host_obj.rgb_fill_color, host_obj.transform));
}

fn CGContextRestoreGState(env: &mut Environment, context: CGContextRef) {
    let host_obj = env.objc.borrow_mut::<CGContextHostObject>(context);
    let state = host_obj.state_stack.pop().unwrap();
    host_obj.rgb_fill_color = state.0;
    host_obj.transform = state.1;
}

fn CGContextSetInterpolationQuality(
    _env: &mut Environment,
    context: CGContextRef,
    quality: CGInterpolationQuality,
) {
    log!(
        "TODO: CGContextSetInterpolationQuality({:?}, {:?})",
        context,
        quality
    );
}

fn CGContextSetLineWidth(
    _env: &mut Environment,
    context: CGContextRef,
    width: CGFloat,
) {
    log!(
        "TODO: CGContextSetLineWidth({:?}, {:?})",
        context,
        width
    );
}

fn CGContextSetRGBStrokeColor(
    _env: &mut Environment,
    context: CGContextRef,
    red: CGFloat,
    green: CGFloat,
    blue: CGFloat,
    alpha: CGFloat,
) {
    log!(
        "TODO: CGContextSetRGBStrokeColor({:?}, {:?})",
        context,
        (red, green, blue, alpha)
    );
}

fn CGContextStrokeRect(
    _env: &mut Environment,
    context: CGContextRef,
    rect: CGRect,
) {
    log!(
        "TODO: CGContextStrokeRect({:?}, {:?})",
        context,
        rect
    );
}

fn CGContextSetBlendMode(
    _env: &mut Environment,
    context: CGContextRef,
    mode: CGBlendMode,
) {
    log!(
        "TODO: CGContextSetBlendMode({:?}, {:?})",
        context,
        mode
    );
}

fn CGContextSetAlpha(
    _env: &mut Environment,
    context: CGContextRef,
    alpha: CGFloat,
) {
    log!(
        "TODO: CGContextSetAlpha({:?}, {:?})",
        context,
        alpha
    );
}

fn CGContextBeginPath(_env: &mut Environment, context: CGContextRef) {
    log!("TODO: CGContextBeginPath({:?})", context);
}

fn CGContextAddRect(
    _env: &mut Environment,
    context: CGContextRef,
    rect: CGRect,
) {
    log!(
        "TODO: CGContextAddRect({:?}, {:?})",
        context,
        rect
    );
}

fn CGContextClosePath(_env: &mut Environment, context: CGContextRef) {
    log!("TODO: CGContextClosePath({:?})", context);
}

fn CGContextFillPath(_env: &mut Environment, context: CGContextRef) {
    log!("TODO: CGContextFillPath({:?})", context);
}

fn CGContextStrokePath(_env: &mut Environment, context: CGContextRef) {
    log!("TODO: CGContextStrokePath({:?})", context);
}

fn CGContextSetShouldAntialias(
    _env: &mut Environment,
    context: CGContextRef,
    should_antialias: bool,
) {
    log!(
        "TODO: CGContextSetShouldAntialias({:?}, {:?})",
        context,
        should_antialias
    );
}

fn CGContextFlush(_env: &mut Environment, context: CGContextRef) {
    log!("CGContextFlush({:?})", context);
}

fn CGContextSetTextPosition(
    _env: &mut Environment,
    context: CGContextRef,
    x: CGFloat,
    y: CGFloat,
) {
    log!(
        "TODO: CGContextSetTextPosition({:?}, {}, {})",
        context,
        x,
        y
    );
}

fn CGContextShowTextAtPoint(
    _env: &mut Environment,
    context: CGContextRef,
    x: CGFloat,
    y: CGFloat,
    text: Ptr<u8, true>,
    length: u32,
) {
    log!(
        "TODO: CGContextShowTextAtPoint({:?}, {}, {}, len={})",
        context,
        x,
        y,
        length
    );
}

fn CGContextDrawLinearGradient(
    _env: &mut Environment,
    context: CGContextRef,
    _gradient: CFTypeRef,
    start: CGPoint,
    end: CGPoint,
    options: u32,
) {
    log!(
        "TODO: CGContextDrawLinearGradient({:?}, {:?} -> {:?}, opts={})",
        context,
        start,
        end,
        options
    );
}

fn CGContextClip(_env: &mut Environment, context: CGContextRef) {
    log!("TODO: CGContextClip({:?})", context);
}

fn CGContextAddEllipseInRect(
    _env: &mut Environment,
    context: CGContextRef,
    rect: CGRect,
) {
    log!(
        "TODO: CGContextAddEllipseInRect({:?}, {:?})",
        context,
        rect
    );
}

fn CGContextSetLineCap(
    _env: &mut Environment,
    context: CGContextRef,
    cap: CGLineCap,
) {
    log!(
        "TODO: CGContextSetLineCap({:?}, {:?})",
        context,
        cap
    );
}

fn CGContextSetLineJoin(
    _env: &mut Environment,
    context: CGContextRef,
    join: CGLineJoin,
) {
    log!(
        "TODO: CGContextSetLineJoin({:?}, {:?})",
        context,
        join
    );
}

fn CGContextSetFont(
    _env: &mut Environment,
    context: CGContextRef,
    font: CGFontRef,
) {
    log!(
        "TODO: CGContextSetFont({:?}, {:?})",
        context,
        font
    );
}

fn CGContextSetFontSize(
    _env: &mut Environment,
    context: CGContextRef,
    size: CGFloat,
) {
    log!(
        "TODO: CGContextSetFontSize({:?}, {:?})",
        context,
        size
    );
}

fn CGContextShowGlyphsAtPoint(
    _env: &mut Environment,
    context: CGContextRef,
    x: CGFloat,
    y: CGFloat,
    glyphs: Ptr<CGGlyph, true>,
    count: u32,
) {
    log!(
        "TODO: CGContextShowGlyphsAtPoint({:?}, {}, {}, count={})",
        context,
        x,
        y,
        count
    );
}

fn CGContextAddArc(
    _env: &mut Environment,
    context: CGContextRef,
    x: CGFloat,
    y: CGFloat,
    radius: CGFloat,
    start_angle: CGFloat,
    end_angle: CGFloat,
    clockwise: bool,
) {
    log!(
        "TODO: CGContextAddArc({:?}, center=({}, {}), r={}, {}→{}, cw={})",
        context,
        x,
        y,
        radius,
        start_angle,
        end_angle,
        clockwise
    );
}

fn CGContextFillEllipseInRect(
    _env: &mut Environment,
    context: CGContextRef,
    rect: CGRect,
) {
    log!(
        "TODO: CGContextFillEllipseInRect({:?}, {:?})",
        context,
        rect
    );
}

fn CGContextDrawPath(
    _env: &mut Environment,
    context: CGContextRef,
    mode: CGPathDrawingMode,
) {
    log!(
        "TODO: CGContextDrawPath({:?}, mode={})",
        context,
        mode
    );
}

fn CGContextSetTextMatrix(
    _env: &mut Environment,
    context: CGContextRef,
    matrix: CGAffineTransform,
) {
    log!(
        "TODO: CGContextSetTextMatrix({:?}, {:?})",
        context,
        matrix
    );
}

fn CGContextSelectFont(
    _env: &mut Environment,
    context: CGContextRef,
    name: Ptr<u8, true>,
    size: CGFloat,
    encoding: CGTextEncoding,
) {
    log!(
        "TODO: CGContextSelectFont({:?}, size={}, encoding={})",
        context,
        size,
        encoding
    );
}

fn CGContextAddLines(
    _env: &mut Environment,
    context: CGContextRef,
    points: Ptr<CGPoint, true>,
    count: u32,
) {
    log!(
        "TODO: CGContextAddLines({:?}, count={})",
        context,
        count
    );
}

fn CGContextAddCurveToPoint(
    _env: &mut Environment,
    context: CGContextRef,
    cp1x: CGFloat,
    cp1y: CGFloat,
    cp2x: CGFloat,
    cp2y: CGFloat,
    x: CGFloat,
    y: CGFloat,
) {
    log!(
        "TODO: CGContextAddCurveToPoint({:?}, cp1=({}, {}), cp2=({}, {}), end=({}, {}))",
        context,
        cp1x,
        cp1y,
        cp2x,
        cp2y,
        x,
        y
    );
}

fn CGContextSetLineDash(
    _env: &mut Environment,
    context: CGContextRef,
    phase: CGFloat,
    lengths: Ptr<CGFloat, true>,
    count: u32,
) {
    log!(
        "TODO: CGContextSetLineDash({:?}, phase={}, count={})",
        context,
        phase,
        count
    );
}

fn CGContextSetTextDrawingMode(
    _env: &mut Environment,
    context: CGContextRef,
    mode: CGTextDrawingMode,
) {
    log!(
        "TODO: CGContextSetTextDrawingMode({:?}, mode={})",
        context,
        mode
    );
}

fn CGContextShowText(
    _env: &mut Environment,
    context: CGContextRef,
    string: ConstPtr<u8>,
    length: GuestUSize,
) {
    log!(
        "TODO: CGContextShowText({:?}, string={:?}, length={})",
        context,
        string,
        length
    );
}

fn CGContextAddArcToPoint(
    _env: &mut Environment,
    context: CGContextRef,
    x1: CGFloat,
    y1: CGFloat,
    x2: CGFloat,
    y2: CGFloat,
    radius: CGFloat,
) {
    log!(
        "TODO: CGContextAddArcToPoint({:?}, p1=({}, {}), p2=({}, {}), r={})",
        context,
        x1,
        y1,
        x2,
        y2,
        radius
    );
}

fn CGContextSetFlatness(
    _env: &mut Environment,
    context: CGContextRef,
    flatness: CGFloat,
) {
    log!(
        "TODO: CGContextSetFlatness({:?}, {})",
        context,
        flatness
    );
}

fn CGContextEOFillPath(
    _env: &mut Environment,
    context: CGContextRef,
) {
    log!(
        "TODO: CGContextEOFillPath({:?})",
        context
    );
}

fn CGContextReplacePathWithStrokedPath(
    _env: &mut Environment,
    context: CGContextRef,
) {
    log!(
        "TODO: CGContextReplacePathWithStrokedPath({:?})",
        context
    );
}

fn CGContextMoveToPoint(
    _env: &mut Environment,
    context: CGContextRef,
    x: CGFloat,
    y: CGFloat,
) {
    log!(
        "TODO: CGContextMoveToPoint({:?}, {}, {})",
        context,
        x,
        y
    );
}

fn CGContextAddLineToPoint(
    _env: &mut Environment,
    context: CGContextRef,
    x: CGFloat,
    y: CGFloat,
) {
    log!(
        "TODO: CGContextAddLineToPoint({:?}, {}, {})",
        context,
        x,
        y
    );
}

fn CGContextSetStrokeColorWithColor(
    _env: &mut Environment,
    context: CGContextRef,
    color: CFTypeRef, // CGColorRef
) {
    log!(
        "TODO: CGContextSetStrokeColorWithColor({:?}, {:?})",
        context,
        color
    );
}

fn CGContextSetFillColorWithColor(
    _env: &mut Environment,
    context: CGContextRef,
    color: CFTypeRef, // CGColorRef
) {
    log!(
        "TODO: CGContextSetFillColorWithColor({:?}, {:?})",
        context,
        color
    );
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CGContextRetain(_)),
    export_c_func!(CGContextRelease(_)),
    export_c_func!(CGContextSetRGBFillColor(_, _, _, _, _)),
    export_c_func!(CGContextSetGrayFillColor(_, _, _)),
    export_c_func!(CGContextFillRect(_, _)),
    export_c_func!(CGContextClearRect(_, _)),
    export_c_func!(CGContextClipToRect(_, _)),
    export_c_func!(CGContextConcatCTM(_, _)),
    export_c_func!(CGContextGetCTM(_)),
    export_c_func!(CGContextRotateCTM(_, _)),
    export_c_func!(CGContextScaleCTM(_, _, _)),
    export_c_func!(CGContextTranslateCTM(_, _, _)),
    export_c_func!(CGContextDrawImage(_, _, _)),
    export_c_func!(CGContextSaveGState(_)),
    export_c_func!(CGContextRestoreGState(_)),
    export_c_func!(CGContextSetInterpolationQuality(_, _)),
    export_c_func!(CGContextSetLineWidth(_, _)),
    export_c_func!(CGContextSetRGBStrokeColor(_, _, _, _, _)),
    export_c_func!(CGContextStrokeRect(_, _)),
    export_c_func!(CGContextSetBlendMode(_, _)),
    export_c_func!(CGContextSetAlpha(_, _)),
    export_c_func!(CGContextBeginPath(_)),
    export_c_func!(CGContextAddRect(_, _)),
    export_c_func!(CGContextClosePath(_)),
    export_c_func!(CGContextFillPath(_)),
    export_c_func!(CGContextStrokePath(_)),
    export_c_func!(CGContextSetShouldAntialias(_, _)),
    export_c_func!(CGContextFlush(_)),
    export_c_func!(CGContextSetTextPosition(_, _, _)),
    export_c_func!(CGContextShowTextAtPoint(_, _, _, _, _)),
    export_c_func!(CGContextClip(_)),
    export_c_func!(CGContextAddEllipseInRect(_, _)),
    export_c_func!(CGContextSetLineCap(_, _)),
    export_c_func!(CGContextSetLineJoin(_, _)),
    export_c_func!(CGContextSetFont(_, _)),
    export_c_func!(CGContextSetFontSize(_, _)),
    export_c_func!(CGContextShowGlyphsAtPoint(_, _, _, _, _)),
    export_c_func!(CGContextAddArc(_, _, _, _, _, _, _)),
    export_c_func!(CGContextFillEllipseInRect(_, _)),
    export_c_func!(CGContextDrawPath(_, _)),
    export_c_func!(CGContextSetTextMatrix(_, _)),
    export_c_func!(CGContextSelectFont(_, _, _, _)),
    export_c_func!(CGContextAddLines(_, _, _)),
    export_c_func!(CGContextAddCurveToPoint(_, _, _, _, _, _, _)),
    export_c_func!(CGContextSetLineDash(_, _, _, _)),
    export_c_func!(CGContextDrawLinearGradient(_, _, _, _, _)),
    export_c_func!(CGContextSetTextDrawingMode(_, _)),
    export_c_func!(CGContextShowText(_, _, _)),
    export_c_func!(CGContextAddArcToPoint(_, _, _, _, _, _)),
    export_c_func!(CGContextSetFlatness(_, _)),
    export_c_func!(CGContextEOFillPath(_)),
    export_c_func!(CGContextReplacePathWithStrokedPath(_)),
    export_c_func!(CGContextMoveToPoint(_, _, _)),
    export_c_func!(CGContextAddLineToPoint(_, _, _)),
    export_c_func!(CGContextSetStrokeColorWithColor(_, _)),
    export_c_func!(CGContextSetFillColorWithColor(_, _)),

];
