/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0.
 */

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
  otherButtonTitles:(id)other
{
    let this: id = msg![env; this init];

    let host = env.objc.borrow_mut::<UIActionSheetHostObject>(this);

    if title != nil {
        host.title = retain(env, title);
    }

    if delegate != nil {
        host.delegate = retain(env, delegate);
    }

    if destructive != nil {
        host.destructive_button_index = host.buttons.len() as i32;
        host.buttons.push(retain(env, destructive));
    }

    if cancel != nil {
        host.cancel_button_index = host.buttons.len() as i32;
        host.buttons.push(retain(env, cancel));
    }

    // NOTE: `otherButtonTitles` is variadic in ObjC.
    // touchHLE cannot support varargs here, so we ignore it safely.

    this
}

- (i32)addButtonWithTitle:(id)title {
    let host = env.objc.borrow_mut::<UIActionSheetHostObject>(this);
    let index = host.buttons.len() as i32;
    if title != nil {
        host.buttons.push(retain(env, title));
    }
    index
}

- (())showInView:(id)_view {
    log!("UIActionSheet showInView: (stub, no UI shown)");
}

- (())dismissWithClickedButtonIndex:(i32)index animated:(bool)_animated {
    log!(
        "UIActionSheet dismissed with button index {} (stub)",
        index
    );
}

- (())dealloc {
    let host = env.objc.borrow::<UIActionSheetHostObject>(this);

    if host.title != nil {
        release(env, host.title);
    }

    if host.delegate != nil {
        release(env, host.delegate);
    }

    for button in host.buttons.iter() {
        release(env, *button);
    }

    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};
