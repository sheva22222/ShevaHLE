/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0.
 */

use crate::objc::{
    id, nil, msg, objc_classes, retain, release,
    ClassExports, HostObject, NSZonePtr,
};
use crate::Environment;

pub struct UINavigationBarHostObject {
    delegate: id,
    items: Vec<id>,
    bar_tint_color: id,
    tint_color: id,
    translucent: bool,
}

impl HostObject for UINavigationBarHostObject {}

impl UINavigationBarHostObject {
    fn new() -> Self {
        Self {
            delegate: nil,
            items: Vec::new(),
            bar_tint_color: nil,
            tint_color: nil,
            translucent: true,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UINavigationBar : NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host = UINavigationBarHostObject::new();
    env.objc.alloc_object(this, Box::new(host), &mut env.mem)
}

- (id)init {
    msg![env; this init]
}

- (())setDelegate:(id)delegate {
    let host = env.objc.borrow_mut::<UINavigationBarHostObject>(this);

    if host.delegate != nil {
        release(env, host.delegate);
    }
    host.delegate = if delegate != nil {
        retain(env, delegate)
    } else {
        nil
    };
}

- (id)delegate {
    let host = env.objc.borrow::<UINavigationBarHostObject>(this);
    host.delegate
}

- (())pushNavigationItem:(id)item animated:(bool)_animated {
    let host = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    if item != nil {
        host.items.push(retain(env, item));
    }
}

- (id)popNavigationItemAnimated:(bool)_animated {
    let host = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    host.items.pop().unwrap_or(nil)
}

- (id)topItem {
    let host = env.objc.borrow::<UINavigationBarHostObject>(this);
    host.items.last().copied().unwrap_or(nil)
}

- (id)backItem {
    let host = env.objc.borrow::<UINavigationBarHostObject>(this);
    if host.items.len() >= 2 {
        host.items.get(host.items.len() - 2).copied().unwrap_or(nil)
    } else {
        nil
    }
}

- (())setItems:(id)items animated:(bool)_animated {
    // UIKit expects an NSArray, but we safely ignore contents for now
    let host = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    for item in host.items.drain(..) {
        release(env, item);
    }
    if items != nil {
        log!("UINavigationBar setItems:animated: (NSArray ignored)");
    }
}

- (())setBarTintColor:(id)color {
    let host = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    if host.bar_tint_color != nil {
        release(env, host.bar_tint_color);
    }
    host.bar_tint_color = if color != nil {
        retain(env, color)
    } else {
        nil
    };
}

- (())setTintColor:(id)color {
    let host = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    if host.tint_color != nil {
        release(env, host.tint_color);
    }
    host.tint_color = if color != nil {
        retain(env, color)
    } else {
        nil
    };
}

- (())setTranslucent:(bool)translucent {
    let host = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    host.translucent = translucent;
}

- (bool)isTranslucent {
    let host = env.objc.borrow::<UINavigationBarHostObject>(this);
    host.translucent
}

- (())dealloc {
    let host = env.objc.borrow::<UINavigationBarHostObject>(this);

    if host.delegate != nil {
        release(env, host.delegate);
    }
    if host.bar_tint_color != nil {
        release(env, host.bar_tint_color);
    }
    if host.tint_color != nil {
        release(env, host.tint_color);
    }
    for item in host.items.iter() {
        release(env, *item);
    }

    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};
