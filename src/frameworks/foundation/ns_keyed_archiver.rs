/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSKeyedArchiver` - Currently just a fake implementation.

use crate::objc::{
    autorelease, id, msg, msg_class, nil, objc_classes, ClassExports, HostObject,
    NSZonePtr,
};
use crate::Environment;
use plist::Dictionary;

struct NSKeyedArchiverHostObject {
    data: id,
    plist: Dictionary,
    /// Something responding to NSKeyedUnarchiverDelegate
    delegate: id,
}
impl HostObject for NSKeyedArchiverHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSKeyedArchiver: NSCoder

+ (id)allocWithZone:(NSZonePtr)_zone { // struct _NSZone*
    let archiver = Box::new(NSKeyedArchiverHostObject {
        delegate: nil,
        data: nil,
        plist: Dictionary::new(),
    });
    env.objc.alloc_object(this, archiver, &mut env.mem)
}

+ (id)archivedDataWithRootObject:(id)_rootObject { // NSData *
    let data: id = msg_class![env; NSMutableData new];
    autorelease(env, data)
}

// TODO: other init methods.

- (id)initForWritingWithMutableData:(id)data { // NSData *
    if data == nil {
        return nil;
    }

    let host_obj = env.objc.borrow_mut::<NSKeyedArchiverHostObject>(this);
    // assert!(host_obj.data.is_null());

    host_obj.data = data;
    host_obj.plist = Dictionary::new();

    this
}

- (())dealloc {
    env.objc.dealloc_object(this, &mut env.mem)
}

// TODO: implement calls to delegate methods
// weak/non-retaining
- (())setDelegate:(id)delegate { // id<NSKeyedUnarchiverDelegate>
    let host_object = env.objc.borrow_mut::<NSKeyedArchiverHostObject>(this);
    host_object.delegate = delegate;
}
- (id)delegate {
    env.objc.borrow::<NSKeyedArchiverHostObject>(this).delegate
}

- (bool)containsValueForKey:(id)key { // NSString*
    // assert!(key != nil);
    return false;
}

- (())encodeObject:(id)obj forKey:(id)key {
    log!(
        "TODO: [(NSKeyedArchiver*){:?} encodeObject:{:?} forKey:{:?}]",
        this,
        obj,
        key
    );

    if key == nil {
        return;
    }

    let host = env.objc.borrow_mut::<NSKeyedArchiverHostObject>(this);
    // Placeholder: store NSNull-like marker
    host.plist.insert(
        format!("{:?}", key),
        plist::Value::String(format!("{:?}", obj)),
    );
}

- (())encodeBool:(bool)value forKey:(id)key {
    log!(
        "TODO: [(NSKeyedArchiver*){:?} encodeBool:{} forKey:{:?}]",
        this,
        value,
        key
    );

    if key == nil {
        return;
    }

    let host = env.objc.borrow_mut::<NSKeyedArchiverHostObject>(this);
    host.plist.insert(
        format!("{:?}", key),
        plist::Value::Boolean(value),
    );
}

- (())encodeInt:(i32)value forKey:(id)key {
    log!(
        "TODO: [(NSKeyedArchiver*){:?} encodeInt:{} forKey:{:?}]",
        this,
        value,
        key
    );

    if key == nil {
        return;
    }

    let host = env.objc.borrow_mut::<NSKeyedArchiverHostObject>(this);
    host.plist.insert(
        format!("{:?}", key),
        plist::Value::Integer(value.into()),
    );
}

- (())encodeInteger:(i32)value forKey:(id)key {
    log!(
        "TODO: [(NSKeyedArchiver*){:?} encodeInteger:{} forKey:{:?}]",
        this,
        value,
        key
    );

    if key == nil {
        return;
    }

    let host = env.objc.borrow_mut::<NSKeyedArchiverHostObject>(this);
    host.plist.insert(
        format!("{:?}", key),
        plist::Value::Integer((value as i64).into()),
    );
}

- (())encodeFloat:(f32)value forKey:(id)key {
    log!(
        "TODO: [(NSKeyedArchiver*){:?} encodeFloat:{} forKey:{:?}]",
        this,
        value,
        key
    );

    if key == nil {
        return;
    }

    let host = env.objc.borrow_mut::<NSKeyedArchiverHostObject>(this);
    host.plist.insert(
        format!("{:?}", key),
        plist::Value::Real(value as f64),
    );
}

- (())encodeDouble:(f64)value forKey:(id)key {
    log!(
        "TODO: [(NSKeyedArchiver*){:?} encodeDouble:{} forKey:{:?}]",
        this,
        value,
        key
    );

    if key == nil {
        return;
    }

    let host = env.objc.borrow_mut::<NSKeyedArchiverHostObject>(this);
    host.plist.insert(
        format!("{:?}", key),
        plist::Value::Real(value),
    );
}

- (())encodeConditionalObject:(id)obj forKey:(id)key {
    // Behaves the same as encodeObject:forKey: for now
    msg![env; this encodeObject:obj forKey:key];
}

- (())encodeObject:(id)obj {
    log!(
        "TODO: [(NSKeyedArchiver*){:?} encodeObject:{:?}]",
        this,
        obj
    );
}

- (())encodeRootObject:(id)obj {
    log!(
        "TODO: [(NSKeyedArchiver*){:?} encodeRootObject:{:?}]",
        this,
        obj
    );
}

- (bool)requiresSecureCoding {
    false
}

- (bool)allowsKeyedCoding {
    true
}

- (())finishEncoding {
    log!(
        "TODO: [(NSKeyedArchiver*){:?} finishEncoding]",
        this
    );
}

- (())encodeBytes:(id)bytes length:(u32)length forKey:(id)key {
    log!(
        "TODO: [(NSKeyedArchiver*){:?} encodeBytes:{:?} length:{} forKey:{:?}]",
        this,
        bytes,
        length,
        key
    );
}

// TODO: add more decode methods

@end

};
