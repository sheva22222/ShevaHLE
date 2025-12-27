/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSDateFormatter`.
//!
//! Resources:
//! - Apple's [Introduction to Data Formatting Programming Guide For Cocoa](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/DataFormatting/DataFormatting.html)
//! - [Unicode Technical Standard #35](https://unicode.org/reports/tr35/tr35-10.html#Date_Format_Patterns)

use crate::frameworks::core_foundation::time::CFAbsoluteTimeGetGregorianDate;
use crate::frameworks::foundation::{ns_string, NSUInteger, NSTimeInterval};
use crate::objc::{autorelease, id, msg, nil, objc_classes, ClassExports, HostObject, NSZonePtr};

struct NSDateFormatterHostObject {
    date_format: Option<id>,
    locale: Option<id>,
    time_zone: Option<id>,
}
impl HostObject for NSDateFormatterHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSDateFormatter: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(NSDateFormatterHostObject {
        date_format: None,
        locale: None,
        time_zone: None,
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)dateFormat {
    let host = env.objc.borrow::<NSDateFormatterHostObject>(this);
    host.date_format.unwrap_or(nil)
}

- (())setDateFormat:(id)format { // NSString *
    let date_format: id = msg![env; format copy];
    env.objc.borrow_mut::<NSDateFormatterHostObject>(this).date_format = Some(date_format);
}

- (())setLocale:(id)locale {
    env.objc.borrow_mut::<NSDateFormatterHostObject>(this).locale = Some(locale);
}

- (id)locale {
    let host = env.objc.borrow::<NSDateFormatterHostObject>(this);
    host.locale.unwrap_or(nil)
}

- (())setTimeZone:(id)tz {
    env.objc.borrow_mut::<NSDateFormatterHostObject>(this).time_zone = Some(tz);
}

- (id)timeZone {
    let host = env.objc.borrow::<NSDateFormatterHostObject>(this);
    host.time_zone.unwrap_or(nil)
}

- (())setDateStyle:(NSUInteger)_style {}
- (())setTimeStyle:(NSUInteger)_style {}

- (id)stringForObjectValue:(id)obj {
    let cls: id = msg![env; obj class];
    let date_cls: id = msg_class![env; NSDate class];
    let is_date: bool = msg![env; cls isSubclassOfClass:date_cls];

    if is_date {
        let s: id = msg![env; this stringFromDate:obj];
        return autorelease(env, s);
    }

    nil
}

- (id)stringFromDate:(id)date {
    let &NSDateFormatterHostObject {
        date_format
    } = env.objc.borrow(this);
    let mut format = ns_string::to_rust_string(env, date_format.unwrap()).to_string().clone();
    log_dbg!("date_format before: {:?}", format);

    let ti: NSTimeInterval = msg![env; date timeIntervalSinceReferenceDate];
    let greg_date = CFAbsoluteTimeGetGregorianDate(env, ti, nil);
    let year = greg_date.year;
    let month = greg_date.month;
    let day = greg_date.day;
    let hour = greg_date.hours;
    let minute = greg_date.minutes;
    let second = greg_date.seconds;

    format = format.replace("yyyy", format!("{year:04}").as_str());
    format = format.replace("YYYY", format!("{year:04}").as_str());
    format = format.replace("MM", format!("{month:02}").as_str());
    format = format.replace("dd", format!("{day:02}").as_str());
    format = format.replace("HH", format!("{hour:02}").as_str());
    format = format.replace("mm", format!("{minute:02}").as_str());
    format = format.replace("ss", format!("{second:02}").as_str());

    for c in format.chars() {
        if let pattern @ ('A'..='Z' | 'a'..='z') = c {
            unimplemented!("date string contains unsubstituted format pattern: {pattern}");
        }
    }
    log_dbg!("date_format after: {:?}", format);

    let res = ns_string::from_rust_string(env, format);
    autorelease(env, res)
}

@end

};
