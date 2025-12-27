/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `GKLocalPlayer`.

use crate::dyld::{ConstantExports, HostConstant};
use crate::Environment;
use crate::objc::{id, msg, objc_classes};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation GKLocalPlayer: NSObject

+ (id)localPlayer {
    let obj: id = msg![env; super alloc];
    msg![env; obj init]
}

- (bool)isAuthenticated {
    false
}

- (id)playerID {
    msg![env; NSString stringWithUTF8String:"LocalPlayer"]
}

- (id)displayName {
    msg![env; NSString stringWithUTF8String:"Player"]
}

- (id)alias {
    msg![env; NSString stringWithUTF8String:"Player"]
}

@end

};


};

pub const GKPlayerAuthenticationDidChangeNotificationName: &str =
    "GKPlayerAuthenticationDidChangeNotificationName";

/// `NSNotificationName` values.
pub const CONSTANTS: ConstantExports = &[(
    "_GKPlayerAuthenticationDidChangeNotificationName",
    HostConstant::NSString(GKPlayerAuthenticationDidChangeNotificationName),
)];
