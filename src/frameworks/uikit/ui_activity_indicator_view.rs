/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIActivityIndicatorView`.

use crate::frameworks::core_graphics::CGRect;
use crate::frameworks::foundation::NSInteger;
use crate::objc::{id, msg, ClassExports};
use crate::objc_classes;

type UIActivityIndicatorViewStyle = NSInteger;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIActivityIndicatorView: UIView

- (id)initWithFrame:(CGRect)_frame {
    msg![env; this init]
}

- (id)initWithActivityIndicatorStyle:(UIActivityIndicatorViewStyle)_style {
    msg![env; this init]
}

- (())startAnimating {
    log!("UIActivityIndicatorView startAnimating {:?}", this);
}

- (())stopAnimating {
    log!("UIActivityIndicatorView stopAnimating {:?}", this);
}

- (bool)isAnimating {
    // Always report "not animating"
    false
}

- (())setHidesWhenStopped:(bool)_hides {
    // ignored
}

- (bool)hidesWhenStopped {
    true
}

- (())setActivityIndicatorViewStyle:(UIActivityIndicatorViewStyle)_style {
    // ignored
}

- (UIActivityIndicatorViewStyle)activityIndicatorViewStyle {
    0
}

@end

};
