/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0.
 */

use crate::frameworks::uikit::ui_view::UIViewHostObject;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg_super, nil, objc_classes, ClassExports, HostObject, NSZonePtr,
    retain, release,
};
use crate::Environment;

pub struct UIProgressViewHostObject {
    superclass: UIViewHostObject,

    pub progress: f32,
    pub progress_tint_color: id,
    pub track_tint_color: id,
    pub style: id,
}

impl_HostObject_with_superclass!(UIProgressViewHostObject);


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
    let host = UIProgressViewHostObject {
        superclass: UIViewHostObject::new(),
        progress: 0.0,
        progress_tint_color: nil,
        track_tint_color: nil,
        style: nil,
    };

    env.objc.alloc_object(this, Box::new(host), &mut env.mem)
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

- (i32)progressViewStyle {
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
    let color = retain(env, color);

    let old = {
        let host = env.objc.borrow_mut::<UIProgressViewHostObject>(this);
        let old = host.progress_tint_color;
        host.progress_tint_color = color;
        old
    };

    if old != nil {
        release(env, old);
    }
}

- (id)progressTintColor {
    env.objc.borrow::<UIProgressViewHostObject>(this).progress_tint_color
}

- (())setTrackTintColor:(id)color {
    let color = retain(env, color);

    let old = {
        let host = env.objc.borrow_mut::<UIProgressViewHostObject>(this);
        let old = host.track_tint_color;
        host.track_tint_color = color;
        old
    };

    if old != nil {
        release(env, old);
    }
}

- (id)trackTintColor {
    env.objc.borrow::<UIProgressViewHostObject>(this).track_tint_color
}

- (())dealloc {
    let (progress_tint, track_tint) = {
        let host = env.objc.borrow::<UIProgressViewHostObject>(this);
        (host.progress_tint_color, host.track_tint_color)
    };

    if progress_tint != nil {
        release(env, progress_tint);
    }
    if track_tint != nil {
        release(env, track_tint);
    }

    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};
