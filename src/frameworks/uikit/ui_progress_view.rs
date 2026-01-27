/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! UIProgressView
//!
//! Minimal, stateful UIProgressView implementation for touchHLE.
//! This implements the commonly-used API surface (storing state, returning
//! sensible defaults). Rendering/animation is not performed by this stub.

use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, nil, release, retain, todo_objc_setter, ClassExports, HostObject, NSZonePtr,
};
use crate::objc_classes;
use crate::Environment;

/// Host object holding UIProgressView state.
struct UIProgressViewHostObject {
    progress: f32,
    style: NSInteger,
    progress_tint_color: Option<id>,
    track_tint_color: Option<id>,
    progress_image: Option<id>,
    track_image: Option<id>,
}
impl HostObject for UIProgressViewHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIProgressView: UIView

// initWithProgressViewStyle:
- (id)initWithProgressViewStyle:(NSInteger)style {
    // Create and initialize host object fields
    let host_object = UIProgressViewHostObject {
        progress: 0.0,
        style,
        progress_tint_color: None,
        track_tint_color: None,
        progress_image: None,
        track_image: None,
    };
    env.objc.alloc_object(this, Box::new(host_object), &mut env.mem);
    // Typical init pattern: retain/return self
    retain(env, this);
    this
}

// init (fallback)
- (id)init {
    // Default to style 0
    msg![env; this initWithProgressViewStyle: 0]
}

// setProgress:animated:
// We ignore animation and just store the value.
- (())setProgress:(f32)progress animated:(bool)animated {
    let clamped = if progress < 0.0 { 0.0 } else if progress > 1.0 { 1.0 } else { progress };
    env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress = clamped;
}

// setProgress:
- (())setProgress:(f32)progress {
    msg![env; this setProgress:progress animated:false]
}

// progress getter
- (f32)progress {
    env.objc.borrow::<UIProgressViewHostObject>(this).progress
}

// setProgressTintColor:
- (())setProgressTintColor:(id)color {
    // release previous
    {
        let host = env.objc.borrow::<UIProgressViewHostObject>(this);
        if let Some(prev) = host.progress_tint_color {
            release(env, prev);
        }
    }
    if !color.is_null() {
        retain(env, color);
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress_tint_color = Some(color);
    } else {
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress_tint_color = None;
    }
}

// progressTintColor
- (id)progressTintColor {
    if let Some(c) = env.objc.borrow::<UIProgressViewHostObject>(this).progress_tint_color {
        c
    } else {
        nil
    }
}

// setTrackTintColor:
- (())setTrackTintColor:(id)color {
    {
        let host = env.objc.borrow::<UIProgressViewHostObject>(this);
        if let Some(prev) = host.track_tint_color {
            release(env, prev);
        }
    }
    if !color.is_null() {
        retain(env, color);
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).track_tint_color = Some(color);
    } else {
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).track_tint_color = None;
    }
}

// trackTintColor
- (id)trackTintColor {
    if let Some(c) = env.objc.borrow::<UIProgressViewHostObject>(this).track_tint_color {
        c
    } else {
        nil
    }
}

// setProgressImage:
- (())setProgressImage:(id)image {
    {
        let host = env.objc.borrow::<UIProgressViewHostObject>(this);
        if let Some(prev) = host.progress_image {
            release(env, prev);
        }
    }
    if !image.is_null() {
        retain(env, image);
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress_image = Some(image);
    } else {
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress_image = None;
    }
}

// progressImage
- (id)progressImage {
    if let Some(img) = env.objc.borrow::<UIProgressViewHostObject>(this).progress_image {
        img
    } else {
        nil
    }
}

// setTrackImage:
- (())setTrackImage:(id)image {
    {
        let host = env.objc.borrow::<UIProgressViewHostObject>(this);
        if let Some(prev) = host.track_image {
            release(env, prev);
        }
    }
    if !image.is_null() {
        retain(env, image);
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).track_image = Some(image);
    } else {
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).track_image = None;
    }
}

// trackImage
- (id)trackImage {
    if let Some(img) = env.objc.borrow::<UIProgressViewHostObject>(this).track_image {
        img
    } else {
        nil
    }
}

// Convenience setter for tintColor (maps to progressTintColor)
- (())setTintColor:(id)color {
    msg![env; this setProgressTintColor: color];
}

// dealloc - release retained resources
- (())dealloc {
    let &UIProgressViewHostObject {
        progress_tint_color,
        track_tint_color,
        progress_image,
        track_image,
        ..
    } = env.objc.borrow(this);

    if let Some(c) = progress_tint_color {
        release(env, c);
    }
    if let Some(c) = track_tint_color {
        release(env, c);
    }
    if let Some(i) = progress_image {
        release(env, i);
    }
    if let Some(i) = track_image {
        release(env, i);
    }

    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};
