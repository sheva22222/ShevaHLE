/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIImageView`.

use crate::frameworks::core_graphics::cg_image::CGImageRef;
use crate::frameworks::core_graphics::{CGPoint, CGRect, CGSize};
use crate::frameworks::foundation::NSTimeInterval;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

#[derive(Default)]
struct UIImageViewHostObject {
    superclass: super::UIViewHostObject,
    image: id,
    animation_images: id,          // NSArray<UIImage*>*
    animation_duration: NSTimeInterval,
    animation_repeat_count: i32,
    animating: bool,
    highlighted_image: id,
    highlighted: bool,
}

impl_HostObject_with_superclass!(UIImageViewHostObject);

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIImageView: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIImageViewHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithFrame:(CGRect)frame {
    let this: id = msg_super![env; this initWithFrame:frame];
    // Not sure if UIImageView does this unconditionally, or only for images
    // with alpha channels.
    () = msg![env; this setOpaque:false];
    this
}

- (())dealloc {
    let &UIImageViewHostObject {
        superclass: _,
        image,
        animation_images,
        animation_duration,
        animation_repeat_count,
        animating,
        highlighted_image,
        highlighted,
    } = env.objc.borrow(this);
    release(env, image);
    msg_super![env; this dealloc]
}

// TODO: initWithCoder:

- (id)initWithImage:(id)image { // UIImage*
    let size: CGSize = msg![env; image size];
    let frame = CGRect {
        origin: CGPoint { x: 0.0, y: 0.0 },
        size
    };
    let this = msg_super![env; this initWithFrame:frame];
    () = msg![env; this setImage:image];
    // Not sure if UIImageView does this unconditionally, or only for images
    // with alpha channels.
    () = msg![env; this setOpaque:false];
    this
}

- (id)image {
    env.objc.borrow::<UIImageViewHostObject>(this).image
}

- (())setImage:(id)new_image { // UIImage*
    let host_obj = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    let old_image = std::mem::replace(&mut host_obj.image, new_image);
    retain(env, new_image);
    release(env, old_image);

    let layer: id = msg![env; this layer];
    let cg_image: CGImageRef = msg![env; new_image CGImage];
    () = msg![env; layer setContents:cg_image];
}

- (id)animationImages {
    env.objc.borrow::<UIImageViewHostObject>(this).animation_images
}

- (())setAnimationImages:(id)images {
    let host = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    let old = std::mem::replace(&mut host.animation_images, images);
    retain(env, images);
    release(env, old);
}

- (NSTimeInterval)animationDuration {
    env.objc.borrow::<UIImageViewHostObject>(this).animation_duration
}

- (())setAnimationDuration:(NSTimeInterval)duration {
    env.objc.borrow_mut::<UIImageViewHostObject>(this).animation_duration = duration;
}

- (i32)animationRepeatCount {
    env.objc.borrow::<UIImageViewHostObject>(this).animation_repeat_count
}

- (())setAnimationRepeatCount:(i32)count {
    env.objc.borrow_mut::<UIImageViewHostObject>(this).animation_repeat_count = count;
}

- (())startAnimating {
    let host = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    host.animating = true;

    // Optional: display first frame
    if host.animation_images != nil {
        let first: id = msg![env; host.animation_images objectAtIndex:0u32];
        () = msg![env; this setImage:first];
    }
}

- (())stopAnimating {
    env.objc.borrow_mut::<UIImageViewHostObject>(this).animating = false;
}

- (bool)isAnimating {
    env.objc.borrow::<UIImageViewHostObject>(this).animating
}

- (())setHighlightedImage:(id)image {
    let host = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    let old = std::mem::replace(&mut host.highlighted_image, image);
    retain(env, image);
    release(env, old);
}

- (id)highlightedImage {
    env.objc.borrow::<UIImageViewHostObject>(this).highlighted_image
}

- (())setHighlighted:(bool)flag {
    let host = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    host.highlighted = flag;

    let image = if flag && host.highlighted_image != nil {
        host.highlighted_image
    } else {
        host.image
    };

    if image != nil {
        let layer: id = msg![env; this layer];
        let cg_image: CGImageRef = msg![env; image CGImage];
        () = msg![env; layer setContents:cg_image];
    }
}

- (bool)isHighlighted {
    env.objc.borrow::<UIImageViewHostObject>(this).highlighted
}

- (i32)contentMode {
    msg_super![env; this contentMode]
}

- (())setContentMode:(i32)mode {
    msg_super![env; this setContentMode:mode]
}

- (CGSize)intrinsicContentSize {
    let image = env.objc.borrow::<UIImageViewHostObject>(this).image;
    if image != nil {
        msg![env; image size]
    } else {
        CGSize { width: 0.0, height: 0.0 }
    }
}

- (bool)isAccessibilityElement {
    true
}

@end

};
