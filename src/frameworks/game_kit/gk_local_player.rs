/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `GKLocalPlayer`.

use crate::dyld::{ConstantExports, HostConstant};
use crate::objc::{id, msg, nil, objc_classes, ClassExports};
use crate::Environment;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// TODO: proper inheritance chain
pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation GKLocalPlayer: NSObject

+ (id)localPlayer {
    // Singleton-style object
    static mut PLAYER: id = nil;

    unsafe {
        if PLAYER == nil {
            PLAYER = msg![env; this alloc];
            PLAYER = msg![env; PLAYER init];
        }
        PLAYER
    }
}

- (bool)isAuthenticated {
    // GameKit not implemented → always unauthenticated
    false
}

- (())authenticateWithCompletionHandler:(id)_handler {
    // Legacy API (iOS 5–6)
    log!("GKLocalPlayer authenticateWithCompletionHandler: (stub)");

    // If a block is provided, call it with nil error
    if _handler != nil {
        // handler(nil)
        msg![env; _handler callWithError:nil];
    }
}

- (())setAuthenticateHandler:(id)_handler {
    // Modern API (iOS 6+)
    log!("GKLocalPlayer setAuthenticateHandler: (stub)");

    // UIKit expects handler to be stored; we ignore and never call it
}

- (id)playerID {
    // Historically required; return a stable dummy string
    msg![env; NSString stringWithUTF8String:"local_player"]
}

- (id)alias {
    msg![env; NSString stringWithUTF8String:"Player"]
}

- (id)displayName {
    msg![env; NSString stringWithUTF8String:"Player"]
}

- (())loadFriendsWithCompletionHandler:(id)_handler {
    log!("GKLocalPlayer loadFriendsWithCompletionHandler: (stub)");

    if _handler != nil {
        // handler(emptyArray, nil)
        let empty = msg![env; NSArray array];
        msg![env; _handler callWithFriends:empty error:nil];
    }
}

@end

};

pub const GKPlayerAuthenticationDidChangeNotificationName: &str =
    "GKPlayerAuthenticationDidChangeNotificationName";

/// `NSNotificationName` values.
pub const CONSTANTS: ConstantExports = &[(
    "_GKPlayerAuthenticationDidChangeNotificationName",
    HostConstant::NSString(GKPlayerAuthenticationDidChangeNotificationName),
)];
