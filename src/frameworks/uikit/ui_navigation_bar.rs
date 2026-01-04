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
    // retain first
    let new_delegate = if delegate != nil {
        retain(env, delegate)
    } else {
        nil
    };

    // swap inside borrow
    let old_delegate = {
        let mut host = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
        let old = host.delegate;
        host.delegate = new_delegate;
        old
    };

    // release AFTER borrow ends
    if old_delegate != nil {
        release(env, old_delegate);
    }
}

- (id)delegate {
    let host = env.objc.borrow::<UINavigationBarHostObject>(this);
    host.delegate
}

- (())pushNavigationItem:(id)item animated:(bool)_animated {
    if item == nil {
        return;
    }

    let item = retain(env, item);

    let mut host = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    host.items.push(item);
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
    // Step 1: move old items out while borrowed
    let old_items = {
        let mut host = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
        host.items.drain(..).collect::<Vec<_>>()
    }; // ← borrow ends here

    // Step 2: now it's safe to release
    for item in old_items {
        release(env, item);
    }

    // UIKit expects NSArray; ignored for now
    if items != nil {
        log!("UINavigationBar setItems:animated: (NSArray ignored)");
    }
}

- (())setBarTintColor:(id)color {
    let new_color = if color != nil {
        retain(env, color)
    } else {
        nil
    };

    let old_color = {
        let mut host = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
        let old = host.bar_tint_color;
        host.bar_tint_color = new_color;
        old
    };

    if old_color != nil {
        release(env, old_color);
    }
}

- (())setTintColor:(id)color {
    let new_color = if color != nil {
        retain(env, color)
    } else {
        nil
    };

    let old_color = {
        let mut host = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
        let old = host.tint_color;
        host.tint_color = new_color;
        old
    };

    if old_color != nil {
        release(env, old_color);
    }
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
    let (delegate, bar_tint_color, tint_color, items) = {
        let host = env.objc.borrow::<UINavigationBarHostObject>(this);
        (
            host.delegate,
            host.bar_tint_color,
            host.tint_color,
            host.items.clone(),
        )
    };

    if delegate != nil {
        release(env, delegate);
    }
    if bar_tint_color != nil {
        release(env, bar_tint_color);
    }
    if tint_color != nil {
        release(env, tint_color);
    }
    for item in items {
        release(env, item);
    }

    env.objc.dealloc_object(this, &mut env.mem);
}

@end

};
