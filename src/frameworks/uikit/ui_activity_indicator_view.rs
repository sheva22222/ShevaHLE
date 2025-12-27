/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIActivityIndicatorView`.

use crate::frameworks::foundation::NSInteger;
use crate::objc::{id, msg, ClassExports};
use crate::objc_classes;

type UIActivityIndicatorViewStyle = NSInteger;

/// Implementation notes:
/// - We don't have dedicated ivars here, so we pack a few small pieces of state into
///   the UIView `tag` integer:
///     bit 0: hidesWhenStopped (1 = true)
///     bit 1: isAnimating       (1 = true)
///     bits 8..16: activityIndicatorViewStyle (stored as a small integer)
/// This is a lightweight shim to provide predictable behavior for the common methods.
pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIActivityIndicatorView: UIView

- (id)initWithActivityIndicatorStyle:(UIActivityIndicatorViewStyle)_style {
    // Initialize via UIView init, then store defaults in the tag:
    // - hidesWhenStopped = true (bit 0)
    // - activityIndicatorViewStyle = _style (stored in bits 8..16)
    let obj: id = msg![env; this init];
    let mut tag: NSInteger = msg![env; obj tag];
    // clear relevant bits first (lower byte for flags, upper byte for style)
    tag &= !(0xff | (0xff << 8));
    tag |= 1; // hidesWhenStopped default true
    tag |= (_style as NSInteger) << 8;
    msg![env; obj setTag: tag];
    // Not animating by default
    msg![env; obj setHidden: false];
    obj
}

- (())startAnimating {
    // Set the isAnimating bit, ensure view is visible and request redraw.
    let mut tag: NSInteger = msg![env; this tag];
    tag |= 1 << 1; // set bit 1 = isAnimating
    msg![env; this setTag: tag];
    // Make sure the view is visible while animating
    msg![env; this setHidden: false];
    // Allow the view to redraw/update appearance
    msg![env; this setNeedsDisplay];
}

- (())stopAnimating {
    // Clear the isAnimating bit. If hidesWhenStopped is set, hide the view.
    let mut tag: NSInteger = msg![env; this tag];
    tag &= !(1 << 1); // clear bit 1 = isAnimating
    msg![env; this setTag: tag];

    let tag: NSInteger = msg![env; this tag];
    let hides: bool = (tag & 1) != 0;

    if hides {
        msg![env; this setHidden: true];
    } else {
        // ensure it's visible (but not animating)
        msg![env; this setHidden: false];
    }
    msg![env; this setNeedsDisplay];
}

- (())setHidesWhenStopped:(bool)_hides {
    // Update bit 0 of tag to reflect hidesWhenStopped
    let mut tag: NSInteger = msg![env; this tag];
    if _hides {
        tag |= 1; // set bit 0
    } else {
        tag &= !1; // clear bit 0
    }
    msg![env; this setTag: tag];
    // If we're not animating and hidesWhenStopped was set true, hide the view now.
    let is_animating: bool = (tag & (1 << 1)) != 0;
    if !is_animating && _hides {
        msg![env; this setHidden: true];
    } else {
        msg![env; this setHidden: false];
    }
}

- (bool)hidesWhenStopped {
    // Return bit 0
    let tag: NSInteger = msg![env; this tag];
    (tag & 1) != 0
}

- (bool)isAnimating {
    // Return bit 1
    let tag: NSInteger = msg![env; this tag];
    (tag & (1 << 1)) != 0
}

- (UIActivityIndicatorViewStyle)activityIndicatorViewStyle {
    // Read stored style from bits 8..16
    let tag: NSInteger = msg![env; this tag];
    ((tag >> 8) & 0xff) as UIActivityIndicatorViewStyle
}

- (())setActivityIndicatorViewStyle:(UIActivityIndicatorViewStyle)_style {
    // Write style into bits 8..16 without touching lower flag bits.
    let mut tag: NSInteger = msg![env; this tag];
    // clear current style bits
    tag &= !(0xff << 8);
    // write new style
    tag |= (_style as NSInteger) << 8;
    msg![env; this setTag: tag];
    // Style change may require redraw
    msg![env; this setNeedsDisplay];
}

@end

};
