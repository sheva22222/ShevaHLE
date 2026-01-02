/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIAlertView`.

use crate::frameworks::foundation::ns_string;
use crate::objc::{id, msg_super, nil, objc_classes, ClassExports};
use std::borrow::Cow;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIAlertView: UIView

- (id)addButtonWithTitle:(id)title {
    log!(
        "TODO: [(UIAlertView*){:?} addButtonWithTitle:{:?}]",
        this,
        title
    );
    // UIKit returns the index of the new button
    nil
}

- (id)buttonTitleAtIndex:(id)index {
    log!(
        "TODO: [(UIAlertView*){:?} buttonTitleAtIndex:{:?}]",
        this,
        index
    );
    nil
}

- (id)numberOfButtons {
    log!(
        "TODO: [(UIAlertView*){:?} numberOfButtons]",
        this
    );
    nil
}

- (id)cancelButtonIndex {
    log!(
        "TODO: [(UIAlertView*){:?} cancelButtonIndex]",
        this
    );
    nil
}

- (())setCancelButtonIndex:(id)index {
    log!(
        "TODO: [(UIAlertView*){:?} setCancelButtonIndex:{:?}]",
        this,
        index
    );
}

- (id)delegate {
    log!(
        "TODO: [(UIAlertView*){:?} delegate]",
        this
    );
    nil
}

- (())setDelegate:(id)delegate {
    log!(
        "TODO: [(UIAlertView*){:?} setDelegate:{:?}]",
        this,
        delegate
    );
}

- (())dismissWithClickedButtonIndex:(id)index animated:(bool)animated {
    log!(
        "TODO: [(UIAlertView*){:?} dismissWithClickedButtonIndex:{:?} animated:{}]",
        this,
        index,
        animated
    );
}

- (id)initWithTitle:(id)title
                      message:(id)message
                     delegate:(id)delegate
            cancelButtonTitle:(id)cancelButtonTitle
            otherButtonTitles:(id)otherButtonTitles {

    log!("TODO: [(UIAlertView*){:?} initWithTitle:{:?} message:{:?} delegate:{:?} cancelButtonTitle:{:?} otherButtonTitles:{:?}]", this, title, message, delegate, cancelButtonTitle, otherButtonTitles);

    let msg = if message == nil { Cow::from("(nil)") } else { ns_string::to_rust_string(env, message) };
    let title = if title == nil { Cow::from("(nil)") } else { ns_string::to_rust_string(env, title) };
    log!("UIAlertView: title: {:?}, message: {:?}", title, msg);

    msg_super![env; this init]
}
- (())show {
    log!("TODO: [(UIAlertView*){:?} show]", this);
}
@end

};
