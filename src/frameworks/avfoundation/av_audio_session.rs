/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! Minimal AVAudioSession stub for touchHLE.
//
//! This provides a stateful, no-op (but sensible) implementation of the
//! commonly-used AVAudioSession API so apps that query or set audio session
//! properties behave as expected.

use crate::frameworks::foundation::{NSInteger, NSTimeInterval};
use crate::mem::GuestUSize;
use crate::objc::{
    id, msg, msg_class, nil, release, retain, todo_objc_setter, ClassExports, HostObject, NSZonePtr,
};
use crate::objc_classes;
use crate::Environment;

/// Host object storing AVAudioSession state.
struct AVAudioSessionHostObject {
    category: Option<id>,
    mode: Option<id>,
    delegate: Option<id>,
    active: bool,
    preferred_sample_rate: f64,
    preferred_io_buffer_duration: NSTimeInterval,
    sample_rate: f64,
    io_buffer_duration: NSTimeInterval,
}
impl HostObject for AVAudioSessionHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation AVAudioSession: NSObject

// +sharedInstance
+ (id)sharedInstance {
    // For simplicity we create and return a new instance. Some apps expect the
    // same pointer back, but for the majority of use cases a fresh instance
    // that stores state works fine. If needed later this can be made a true
    // singleton stored in framework_state.
    let host_object = AVAudioSessionHostObject {
        category: None,
        mode: None,
        delegate: None,
        active: false,
        preferred_sample_rate: 44100.0,
        preferred_io_buffer_duration: 0.023219954, // ~1024/44100
        sample_rate: 44100.0,
        io_buffer_duration: 0.023219954,
    };
    let obj = env.objc.alloc_object(this, Box::new(host_object), &mut env.mem);
    // Historically sharedInstance returns an autoreleased object; retain so
    // callers can rely on it surviving until they explicitly release it.
    retain(env, obj);
    obj
}

// - (BOOL)setActive:(BOOL)active error:(NSError**)outError
- (bool)setActive:(bool)active error:(MutPtr<id>)outError {
    let host = env.objc.borrow_mut::<AVAudioSessionHostObject>(this);
    host.active = active;
    // No errors are produced by this stub implementation.
    true
}

// - (BOOL)setCategory:(NSString*)category error:(NSError**)outError
- (bool)setCategory:(id)category error:(MutPtr<id>)outError {
    // Store the category string (retain it).
    // Release previous if any.
    {
        let host = env.objc.borrow::<AVAudioSessionHostObject>(this);
        if let Some(prev) = host.category {
            release(env, prev);
        }
    }
    if !category.is_null() {
        retain(env, category);
        env.objc.borrow_mut::<AVAudioSessionHostObject>(this).category = Some(category);
    } else {
        env.objc.borrow_mut::<AVAudioSessionHostObject>(this).category = None;
    }
    true
}

- (id)category {
    if let Some(cat) = env.objc.borrow::<AVAudioSessionHostObject>(this).category {
        cat
    } else {
        nil
    }
}

// - (BOOL)setMode:(NSString*)mode error:(NSError**)outError
- (bool)setMode:(id)mode error:(MutPtr<id>)outError {
    {
        let host = env.objc.borrow::<AVAudioSessionHostObject>(this);
        if let Some(prev) = host.mode {
            release(env, prev);
        }
    }
    if !mode.is_null() {
        retain(env, mode);
        env.objc.borrow_mut::<AVAudioSessionHostObject>(this).mode = Some(mode);
    } else {
        env.objc.borrow_mut::<AVAudioSessionHostObject>(this).mode = None;
    }
    true
}

- (id)mode {
    if let Some(m) = env.objc.borrow::<AVAudioSessionHostObject>(this).mode {
        m
    } else {
        nil
    }
}

// - (void)setDelegate:(id)delegate
- (())setDelegate:(id)delegate {
    todo_objc_setter!(this, delegate);
}

// - (BOOL)setPreferredSampleRate:(double)sampleRate error:(NSError**)outError
- (bool)setPreferredSampleRate:(f64)sampleRate error:(MutPtr<id>)outError {
    env.objc.borrow_mut::<AVAudioSessionHostObject>(this).preferred_sample_rate = sampleRate;
    true
}

- (f64)preferredSampleRate {
    env.objc.borrow::<AVAudioSessionHostObject>(this).preferred_sample_rate
}

- (f64)sampleRate {
    env.objc.borrow::<AVAudioSessionHostObject>(this).sample_rate
}

// - (BOOL)setPreferredIOBufferDuration:(NSTimeInterval)duration error:(NSError**)outError
- (bool)setPreferredIOBufferDuration:(NSTimeInterval)duration error:(MutPtr<id>)outError {
    env.objc.borrow_mut::<AVAudioSessionHostObject>(this).preferred_io_buffer_duration = duration;
    true
}

- (NSTimeInterval)preferredIOBufferDuration {
    env.objc.borrow::<AVAudioSessionHostObject>(this).preferred_io_buffer_duration
}

- (NSTimeInterval)ioBufferDuration {
    env.objc.borrow::<AVAudioSessionHostObject>(this).io_buffer_duration
}

// - (BOOL)isOtherAudioPlaying
- (bool)isOtherAudioPlaying {
    // We do not simulate other apps playing audio.
    false
}

// - (BOOL)setActive:withOptions:error: (convenience)
- (bool)setActive:(bool)active withOptions:(NSUInteger)options error:(MutPtr<id>)outError {
    env.objc.borrow_mut::<AVAudioSessionHostObject>(this).active = active;
    true
}

// - dealloc
- (())dealloc {
    let &AVAudioSessionHostObject {
        category,
        mode,
        delegate: _,
        ..
    } = env.objc.borrow(this);
    if let Some(c) = category {
        release(env, c);
    }
    if let Some(m) = mode {
        release(env, m);
    }
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};
