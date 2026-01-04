/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0.
 */

use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, nil, msg, objc_classes, retain, release,
    ClassExports, HostObject, NSZonePtr,
};
use crate::Environment;

pub struct UIActionSheetHostObject {
    title: id,
    delegate: id,
    cancel_button_index: i32,
    destructive_button_index: i32,
    buttons: Vec<id>,
}

impl HostObject for UIActionSheetHostObject {}

impl UIActionSheetHostObject {
    fn new() -> Self {
        Self {
            title: nil,
            delegate: nil,
            cancel_button_index: -1,
            destructive_button_index: -1,
            buttons: Vec::new(),
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIActionSheet : NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host = UIActionSheetHostObject::new();
    env.objc.alloc_object(this, Box::new(host), &mut env.mem)
}

- (id)initWithTitle:(id)title
           delegate:(id)delegate
  cancelButtonTitle:(id)cancel
destructiveButtonTitle:(id)destructive
  otherButtonTitles:(id)_others
{
    // retain first (NO host borrow yet)
    let title = if title != nil { retain(env, title) } else { nil };
    let delegate = if delegate != nil { retain(env, delegate) } else { nil };
    let cancel = if cancel != nil { retain(env, cancel) } else { nil };
    let destructive = if destructive != nil { retain(env, destructive) } else { nil };

    let mut host = env.objc.borrow_mut::<UIActionSheetHostObject>(this);

    host.title = title;
    host.delegate = delegate;
    host.buttons.clear();
    host.cancel_button_index = -1;
    host.destructive_button_index = -1;

    if destructive != nil {
        host.destructive_button_index = host.buttons.len() as i32;
        host.buttons.push(destructive);
    }

    if cancel != nil {
        host.cancel_button_index = host.buttons.len() as i32;
        host.buttons.push(cancel);
    }

    this
}

- (NSInteger)addButtonWithTitle:(id)title {
    if title == nil {
        return -1;
    }

    let title = retain(env, title);

    let mut host = env.objc.borrow_mut::<UIActionSheetHostObject>(this);
    let index = host.buttons.len() as i32;
    host.buttons.push(title);
    index
}

- (id)buttonTitleAtIndex:(NSInteger)index {
    let host = env.objc.borrow::<UIActionSheetHostObject>(this);
    host.buttons.get(index as usize).copied().unwrap_or(nil)
}

- (NSInteger)numberOfButtons {
    let host = env.objc.borrow::<UIActionSheetHostObject>(this);
    host.buttons.len() as i32
}

- (())showInView:(id)_view {
    log!("UIActionSheet showInView: (stub)");
}

- (())dismissWithClickedButtonIndex:(i32)index animated:(bool)_animated {
    log!(
        "UIActionSheet dismissed with button index {} (stub)",
        index
    );
}

- (())dealloc {
    // extract only
    let (title, delegate, buttons) = {
        let host = env.objc.borrow::<UIActionSheetHostObject>(this);
        (host.title, host.delegate, host.buttons.clone())
    };

    // release AFTER borrow ends
    if title != nil {
        release(env, title);
    }
    if delegate != nil {
        release(env, delegate);
    }
    for b in buttons {
        release(env, b);
    }

    env.objc.dealloc_object(this, &mut env.mem);
}

@end

};
