/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSURLConnection`.

use crate::objc::{autorelease, id, msg, nil, objc_classes, release, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSURLConnection: NSObject

+ (id)connectionWithRequest:(id)request // NSURLRequest *
                   delegate:(id)delegate {
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initWithRequest:request delegate:delegate];
    autorelease(env, new)
}

+ (id)connectionWithRequest:(id)request
                   delegate:(id)delegate
           startImmediately:(bool)start_immediately {
    let new: id = msg![env; this alloc];
    let new: id = msg![
        env;
        new initWithRequest:request
                 delegate:delegate
         startImmediately:start_immediately
    ];
    autorelease(env, new)
}

+ (id)sendSynchronousRequest:(id)_request
           returningResponse:(id *)_response
                       error:(id *)_error {
    log!("TODO: NSURLConnection sendSynchronousRequest");
    nil
}

+ (bool)canHandleRequest:(id)_request {
    true
}

- (id)initWithRequest:(id)request // NSURLRequest *
             delegate:(id)delegate {
    msg![env; this initWithRequest:request delegate:delegate startImmediately:true]
}

- (id)initWithRequest:(id)request // NSURLRequest *
             delegate:(id)delegate
     startImmediately:(bool)start_immediately {
    log!(
        "TODO: [(NSURLConnection *){:?} initWithRequest:{:?} delegate:{:?} startImmediately:{}]",
        this,
        request,
        delegate,
        start_immediately,
    );
    release(env, this);
    nil
}

- (())start {
    log!("TODO: [(NSURLConnection *){:?} start]", this);
}

- (())cancel {
    log!("TODO: [(NSURLConnection *){:?} cancel]", this);
}

- (())scheduleInRunLoop:(id)_run_loop
               forMode:(id)_mode {
    // no-op
}

- (())unscheduleFromRunLoop:(id)_run_loop
                    forMode:(id)_mode {
    // no-op
}

- (id)description {
    msg![env; super description]
}

- (id)currentRequest {
    nil
}

@end

};
