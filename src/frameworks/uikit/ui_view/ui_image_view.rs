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
    id, impl_HostObject_with_superclass, msg, msg_super, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

#[derive(Default)]
struct UIImageViewHostObject {
    superclass: super::UIViewHostObject,
    /// `UIImage*`
    image: id,
    /// `UIImage*` used when highlighted
    highlighted_image: id,
    /// whether the image view is highlighted
    highlighted: bool,
    /// `NSArray<UIImage *>*`
    animation_images: id,
    /// animation duration
    animation_duration: NSTimeInterval,
    /// animation repeat count
    animation_repeat_count: u32,
    /// whether the view is currently animating
    animating: bool,
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
    let mut host = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    release(env, host.image);
    release(env, host.highlighted_image);
    release(env, host.animation_images);
    // other fields are primitives, nothing to release
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

- (id)initWithImage:(id)image highlightedImage:(id)highlightedImage {
    let this: id = msg![env; this initWithImage:image];
    () = msg![env; this setHighlightedImage:highlightedImage];
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

    // If not highlighted, update layer contents immediately
    if !host_obj.highlighted {
        let layer: id = msg![env; this layer];
        let cg_image: CGImageRef = msg![env; new_image CGImage];
        () = msg![env; layer setContents:cg_image];
    }
}

- (id)highlightedImage {
    env.objc.borrow::<UIImageViewHostObject>(this).highlighted_image
}

- (())setHighlightedImage:(id)new_image {
    let host_obj = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    let old = std::mem::replace(&mut host_obj.highlighted_image, new_image);
    retain(env, new_image);
    release(env, old);

    // If currently highlighted, update displayed contents
    if host_obj.highlighted {
        let layer: id = msg![env; this layer];
        if new_image as usize != 0 {
            let cg_image: CGImageRef = msg![env; new_image CGImage];
            () = msg![env; layer setContents:cg_image];
        } else {
            // fall back to normal image
            if host_obj.image as usize != 0 {
                let cg_image: CGImageRef = msg![env; host_obj.image CGImage];
                () = msg![env; layer setContents:cg_image];
            } else {
                () = msg![env; layer setContents:0usize];
            }
        }
    }
}

- (bool)isHighlighted {
    env.objc.borrow::<UIImageViewHostObject>(this).highlighted
}

- (())setHighlighted:(bool)h {
    let mut host_obj = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    host_obj.highlighted = h;

    let layer: id = msg![env; this layer];
    if h {
        if host_obj.highlighted_image as usize != 0 {
            let cg_image: CGImageRef = msg![env; host_obj.highlighted_image CGImage];
            () = msg![env; layer setContents:cg_image];
            return;
        }
    }

    // either not highlighted or no highlighted image -> show normal image (or clear)
    if host_obj.image as usize != 0 {
        let cg_image: CGImageRef = msg![env; host_obj.image CGImage];
        () = msg![env; layer setContents:cg_image];
    } else {
        () = msg![env; layer setContents:0usize];
    }
}

// Animation images

- (())setAnimationImages:(id)images { // NSArray<UIImage *>*
    let host = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    let old = std::mem::replace(&mut host.animation_images, images);
    retain(env, images);
    release(env, old);

    // If provided an array, use the first image as the current image (if any)
    if images as usize != 0 {
        let count: usize = msg![env; images count];
        if count > 0 {
            let first_image: id = msg![env; images objectAtIndex:0u32];
            // setImage will update layer if not highlighted
            () = msg![env; this setImage:first_image];
        }
    }
}

- (id)animationImages {
    env.objc.borrow::<UIImageViewHostObject>(this).animation_images
}

- (NSTimeInterval)animationDuration {
    env.objc.borrow::<UIImageViewHostObject>(this).animation_duration
}

- (())setAnimationDuration:(NSTimeInterval)duration {
    let mut host = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    host.animation_duration = duration;
}

- (u32)animationRepeatCount {
    env.objc.borrow::<UIImageViewHostObject>(this).animation_repeat_count
}

- (())setAnimationRepeatCount:(u32)count {
    let mut host = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    host.animation_repeat_count = count;
}

- (())startAnimating {
    let mut host = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    if host.animating {
        return;
    }
    host.animating = true;

    // We don't currently implement a full animation loop / timer here.
    // Provide minimal behaviour: if animationImages exists, show the first image.
    if host.animation_images as usize != 0 {
        let count: usize = msg![env; host.animation_images count];
        if count > 0 {
            let first_image: id = msg![env; host.animation_images objectAtIndex:0u32];
            () = msg![env; this setImage:first_image];
            log!("UIImageView startAnimating: showing first of {} images (duration={} repeatCount={}) for {:?}", count, host.animation_duration, host.animation_repeat_count, this);
        }
    } else {
        log!("UIImageView startAnimating: no animation images for {:?}", this);
    }
}

- (())stopAnimating {
    let mut host = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    if !host.animating {
        return;
    }
    host.animating = false;
    // Reset to the primary image
    if host.image as usize != 0 {
        () = msg![env; this setImage:host.image];
    }
    log!("UIImageView stopAnimating: stopped for {:?}", this);
}

- (bool)isAnimating {
    env.objc.borrow::<UIImageViewHostObject>(this).animating
}

@end

};
