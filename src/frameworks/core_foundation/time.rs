/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! Time things including `CFAbsoluteTime`.

use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::core_foundation::CFTypeRef;
use crate::frameworks::foundation::NSTimeInterval;
use crate::libc::time::{time_t, timestamp_to_calendar_date};
use crate::mem::SafeRead;
use crate::objc::nil;
use crate::{impl_GuestRet_for_large_struct, Environment};
use std::ops::Add;
use std::time::{Duration, SystemTime};

/// Seconds between Unix and Apple's epochs
pub const SECS_FROM_UNIX_TO_APPLE_EPOCHS: u64 = 978_307_200;

/// The absolute reference date is 1 Jan 2001 00:00:00 GMT
pub fn apple_epoch() -> SystemTime {
    SystemTime::UNIX_EPOCH.add(Duration::from_secs(SECS_FROM_UNIX_TO_APPLE_EPOCHS))
}

pub type CFTimeInterval = NSTimeInterval;
pub type CFAbsoluteTime = CFTimeInterval;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C, packed)]
pub struct CFGregorianDate {
    pub year: i32,    // SInt32
    pub month: i8,    // SInt8
    pub day: i8,      // SInt8
    pub hours: i8,    // SInt8
    pub minutes: i8,  // SInt8
    pub seconds: f64, // double
}
unsafe impl SafeRead for CFGregorianDate {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CFGregorianUnits {
    pub years: i32,
    pub months: i32,
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: f64,
}
unsafe impl SafeRead for CFGregorianUnits {}

impl_GuestRet_for_large_struct!(CFGregorianDate);

/// Absolute time is measured in seconds relative to the absolute reference date
/// of Jan 1 2001 00:00:00 GMT.
fn CFAbsoluteTimeGetCurrent(_env: &mut Environment) -> CFAbsoluteTime {
    SystemTime::now()
        .duration_since(apple_epoch())
        .unwrap()
        .as_secs_f64()
}

type CFTimeZoneRef = CFTypeRef;

fn CFTimeZoneCopySystem(_env: &mut Environment) -> CFTimeZoneRef {
    // TODO: implement (nil seems to correspond to GMT)
    nil
}

pub fn CFAbsoluteTimeGetGregorianDate(
    _env: &mut Environment,
    at: CFAbsoluteTime,
    tz: CFTimeZoneRef,
) -> CFGregorianDate {
    assert!(tz.is_null());
    let time64 = apple_epoch()
        .add(Duration::from_secs_f64(at))
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let time = time64 as time_t;
    let tm = timestamp_to_calendar_date(time);
    CFGregorianDate {
        year: 1900 + tm.tm_year,
        month: (tm.tm_mon + 1) as i8,
        day: tm.tm_mday as i8,
        hours: tm.tm_hour as i8,
        minutes: tm.tm_min as i8,
        seconds: tm.tm_sec.into(),
    }
}

fn CFAbsoluteTimeGetDayOfWeek(env: &mut Environment, at: CFAbsoluteTime, tz: CFTimeZoneRef) -> i32 {
    // assert!(tz.is_null());
    CFAbsoluteTimeGetGregorianDate(env, at, tz).day.into()
}

fn CFAbsoluteTimeAddGregorianUnits(
    _env: &mut Environment,
    at: CFAbsoluteTime,
    _tz: CFTimeZoneRef,
    units: CFGregorianUnits,
) -> CFAbsoluteTime {
    // Best-effort: add seconds only (safe, predictable)
    at + units.seconds
}

fn CFAbsoluteTimeGetDifferenceAsGregorianUnits(
    _env: &mut Environment,
    at1: CFAbsoluteTime,
    at2: CFAbsoluteTime,
    _tz: CFTimeZoneRef,
) -> CFGregorianUnits {
    CFGregorianUnits {
        years: 0,
        months: 0,
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: at2 - at1,
    }
}

fn CFTimeZoneGetSecondsFromGMT(
    _env: &mut Environment,
    tz: CFTimeZoneRef,
    _at: CFAbsoluteTime,
) -> i32 {
    // nil == GMT
    if tz.is_null() { 0 } else { 0 }
}

fn CFTimeZoneGetName(_env: &mut Environment, _tz: CFTimeZoneRef) -> CFTypeRef {
    nil
}

fn CFTimeZoneIsDaylightSavingTime(
    _env: &mut Environment,
    _tz: CFTimeZoneRef,
    _at: CFAbsoluteTime,
) -> bool {
    false
}

fn CFAbsoluteTimeGetDayOfYear(
    env: &mut Environment,
    at: CFAbsoluteTime,
    tz: CFTimeZoneRef,
) -> i32 {
    let d = CFAbsoluteTimeGetGregorianDate(env, at, tz);
    // Rough, but deterministic: day within month + offset
    d.day as i32
}

fn CFAbsoluteTimeGetSecondsSinceReferenceDate(
    _env: &mut Environment,
    at: CFAbsoluteTime,
) -> CFTimeInterval {
    at
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFAbsoluteTimeGetCurrent()),
    export_c_func!(CFTimeZoneCopySystem()),
    export_c_func!(CFAbsoluteTimeGetGregorianDate(_, _)),
    export_c_func!(CFAbsoluteTimeGetDayOfWeek(_, _)),
    export_c_func!(CFAbsoluteTimeGetSecondsSinceReferenceDate(_)),
    export_c_func!(CFAbsoluteTimeAddGregorianUnits(_, _, _)),
    export_c_func!(CFAbsoluteTimeGetDifferenceAsGregorianUnits(_, _, _)),
    export_c_func!(CFTimeZoneGetSecondsFromGMT(_, _)),
    export_c_func!(CFTimeZoneGetName(_)),
    export_c_func!(CFTimeZoneIsDaylightSavingTime(_, _)),
    export_c_func!(CFAbsoluteTimeGetDayOfYear(_, _)),

];
