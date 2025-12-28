/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIImagePickerController`

use crate::frameworks::foundation::NSInteger;
use crate::objc::{id, nil, objc_classes, ClassExports};

type UIImagePickerControllerSourceType = NSInteger;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// TODO: should extend UINavigationController, which extends
//       UIViewController.
@implementation UIImagePickerController: UIViewController

+ (bool)isSourceTypeAvailable:(UIImagePickerControllerSourceType)_type {
    // For now, simply claim no sources are available.
    // TODO: support some sources.
    false
}

- (())setDelegate:(id)_delegate {
    // TODO
}

- (UIImagePickerControllerSourceType)sourceType {
    0
}

- (())setSourceType:(UIImagePickerControllerSourceType)_type {
    // ignored
}

- (id)mediaTypes {
    nil
}

- (())setMediaTypes:(id)_mediaTypes {
    // ignored
}

- (bool)allowsEditing {
    false
}

- (())setAllowsEditing:(bool)_allowsEditing {
    // ignored
}

- (NSInteger)cameraDevice {
    0
}

- (())setCameraDevice:(NSInteger)_device {
    // ignored
}

- (NSInteger)cameraCaptureMode {
    0
}

- (())setCameraCaptureMode:(NSInteger)_mode {
    // ignored
}

- (NSInteger)videoQuality {
    0
}

- (())setVideoQuality:(NSInteger)_quality {
    // ignored
}

- (double)videoMaximumDuration {
    0.0
}

- (())setVideoMaximumDuration:(double)_duration {
    // ignored
}

@end

};
