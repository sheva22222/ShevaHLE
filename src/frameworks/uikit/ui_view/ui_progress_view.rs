/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0.
 */

use crate::objc::{
    id, msg_super, nil, objc_classes, ClassExports, HostObject, NSZonePtr,
    retain, release,
};
use crate::Environment;

struct UIProgressViewHostObject {
    progress: f32,
    progress_tint_color: id, // UIColor*
    track_tint_color: id,    // UIColor*
    style: i32,              // UIProgressViewStyle
}

impl HostObject for UIProgressViewHostObject {}

impl Default for UIProgressViewHostObject {
    fn default() -> Self {
        Self {
            progress: 0.0,
            progress_tint_color: nil,
            track_tint_color: nil,
            style: 0, // UIProgressViewStyleDefault
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIProgressView : UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIProgressViewHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithProgressViewStyle:(i32)style {
    let this: id = msg_super![env; this init];
    if this == nil {
        return nil;
    }

    env.objc.borrow_mut::<UIProgressViewHostObject>(this).style = style;
    this
}

- (())setProgressViewStyle:(i32)style {
    env.objc.borrow_mut::<UIProgressViewHostObject>(this).style = style;
}

- (f32)progressViewStyle {
    env.objc.borrow::<UIProgressViewHostObject>(this).style
}

- (())setProgress:(f32)progress {
    env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress = progress;
}

- (())setProgress:(f32)progress animated:(bool)animated {
    let host = env.objc.borrow_mut::<UIProgressViewHostObject>(this);

    // Animation is ignored for now (UIKit-compatible stub)
    host.progress = progress;

    log_dbg!(
        "[(UIProgressView*){:?} setProgress:{} animated:{}]",
        this,
        progress,
        animated
    );
}

- (f32)progress {
    env.objc.borrow::<UIProgressViewHostObject>(this).progress
}

- (())setProgressTintColor:(id)color {
    let host = env.objc.borrow_mut::<UIProgressViewHostObject>(this);

    if host.progress_tint_color != nil {
        release(env, host.progress_tint_color);
    }

    host.progress_tint_color = color;
    if color != nil {
        retain(env, color);
    }
}

- (id)progressTintColor {
    env.objc.borrow::<UIProgressViewHostObject>(this).progress_tint_color
}

- (())setTrackTintColor:(id)color {
    let host = env.objc.borrow_mut::<UIProgressViewHostObject>(this);

    if host.track_tint_color != nil {
        release(env, host.track_tint_color);
    }

    host.track_tint_color = color;
    if color != nil {
        retain(env, color);
    }
}

- (id)trackTintColor {
    env.objc.borrow::<UIProgressViewHostObject>(this).track_tint_color
}

- (())dealloc {
    let host = env.objc.borrow::<UIProgressViewHostObject>(this);

    if host.progress_tint_color != nil {
        release(env, host.progress_tint_color);
    }
    if host.track_tint_color != nil {
        release(env, host.track_tint_color);
    }

    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};
