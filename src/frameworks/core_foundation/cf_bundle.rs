/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CFBundle`.
//!
//! This is not even toll-free bridged to `NSBundle` in Apple's implementation,
//! but here it is the same type.

use super::cf_array::CFArrayRef;
use super::cf_string::CFStringRef;
use super::cf_url::CFURLRef;
use super::CFTypeRef;
use crate::dyld::{export_c_func, ConstantExports, FunctionExports, HostConstant};
use crate::frameworks::foundation::ns_bundle::NSBundleHostObject;
use crate::frameworks::foundation::{ns_array, ns_string, NSUInteger};
use crate::objc::{id, msg, msg_class, retain};
use crate::Environment;

const kCFBundleVersionKey: &str = "CFBundleVersion";

pub const CONSTANTS: ConstantExports = &[(
    "_kCFBundleVersionKey",
    HostConstant::NSString(kCFBundleVersionKey),
)];

pub type CFBundleRef = CFTypeRef;

fn CFBundleGetMainBundle(env: &mut Environment) -> CFBundleRef {
    msg_class![env; NSBundle mainBundle]
}

fn CFBundleGetValueForInfoDictionaryKey(
    env: &mut Environment,
    bundle: CFBundleRef,
    key: CFStringRef,
) -> CFTypeRef {
    let dict: id = msg![env; bundle infoDictionary];
    msg![env; dict objectForKey:key]
}

fn CFBundleGetVersionNumber(env: &mut Environment, bundle: CFBundleRef) -> u32 {
    let version_key: id = ns_string::get_static_str(env, kCFBundleVersionKey);
    let vers: id = CFBundleGetValueForInfoDictionaryKey(env, bundle, version_key);
    let vers_str = ns_string::to_rust_string(env, vers);
    log_dbg!("CFBundleGetVersionNumber {}", vers_str);

    let parts: Vec<&str> = vers_str.split('.').collect();
    assert!(parts.len() <= 3);

    let mut result: u32 = 1 << 15;
    let major: u32 = parts[0].parse().unwrap();
    assert!(major <= 99);
    result |= (major / 10) << 28;
    result |= (major % 10) << 24;
    if parts.len() >= 2 {
        let minor: u32 = parts[1].parse().unwrap();
        assert!(minor <= 9);
        result |= minor << 20;
    }
    if parts.len() == 3 {
        let bug_fix: u32 = parts[2].parse().unwrap();
        assert!(bug_fix <= 9);
        result |= bug_fix << 16;
    }
    result
}

fn CFBundleCopyBundleURL(env: &mut Environment, bundle: CFBundleRef) -> CFURLRef {
    let url: CFURLRef = msg![env; bundle bundleURL];
    msg![env; url copy]
}

fn CFBundleCopyResourcesDirectoryURL(env: &mut Environment, bundle: CFBundleRef) -> CFURLRef {
    let url: CFURLRef = msg![env; bundle resourceURL];
    msg![env; url copy]
}

fn CFBundleCopyResourceURL(
    env: &mut Environment,
    bundle: CFBundleRef,
    resource_name: CFStringRef,
    resource_type: CFStringRef,
    sub_dir_name: CFStringRef,
) -> CFURLRef {
    let url: CFURLRef = msg![env; bundle URLForResource:resource_name
                                          withExtension:resource_type
                                           subdirectory:sub_dir_name];
    msg![env; url copy]
}

pub fn CFBundleCopyBundleLocalizations(env: &mut Environment, bundle: CFBundleRef) -> CFArrayRef {
    let bundle_localizations = env
        .objc
        .borrow_mut::<NSBundleHostObject>(bundle)
        .bundle
        .as_ref()
        .unwrap_or(&env.bundle)
        .bundle_localizations()
        .iter()
        .map(|value| value.as_string().unwrap().to_string())
        .collect::<Vec<String>>();
    let guest_bundle_localizations = bundle_localizations
        .iter()
        .map(|loc| ns_string::from_rust_string(env, loc.to_owned()))
        .collect::<Vec<id>>();
    let loc_array = ns_array::from_vec(env, guest_bundle_localizations);
    log_dbg!(
        "CFBundleCopyBundleLocalizations({:?}) => {:?} ({})",
        bundle,
        loc_array,
        bundle_localizations.join(", ")
    );
    loc_array
}

pub fn CFBundleCopyPreferredLocalizationsFromArray(
    env: &mut Environment,
    loc_array: CFArrayRef,
) -> CFArrayRef {
    let mut result = Vec::new();

    let preferred_languages: id = msg_class![env; NSLocale preferredLanguages];

    // Check if the user's preferred languages are in loc_array
    let loc_count: NSUInteger = msg![env; loc_array count];
    let pref_loc_count: NSUInteger = msg![env; preferred_languages count];
    for loc_index in 0..loc_count {
        let loc: id = msg![env; loc_array objectAtIndex:loc_index];
        for pref_loc_index in 0..pref_loc_count {
            let pref_loc: id = msg![env; preferred_languages objectAtIndex:pref_loc_index];
            let equal: bool = msg![env; loc isEqualToString:pref_loc];
            if equal {
                // If one of them is, add it to the array
                result.push(loc);
                retain(env, loc);
                break;
            }
        }
    }

    if loc_count > 0 {
        // Add the first element as fallback
        let first_loc: id = msg![env; loc_array objectAtIndex:(0 as NSUInteger)];
        result.push(first_loc);
        retain(env, first_loc);
    } else {
        // Behaviour was verified on macOS
        let en_loc = ns_string::get_static_str(env, "en");
        result.push(en_loc);
    };

    let result = ns_array::from_vec(env, result);
    log_dbg!(
        "CFBundleCopyPreferredLocalizationsFromArray({:?}) => {:?}",
        loc_array,
        result
    );
    result
}

fn CFBundleCopyLocalizedString(
    env: &mut Environment,
    bundle: CFBundleRef,
    key: CFStringRef,
    value: CFStringRef,
    table_name: CFStringRef,
) -> CFStringRef {
    let res = msg![env; bundle localizedStringForKey:key value:value table:table_name];
    msg![env; res copy]
}


fn CFBundleGetIdentifier(env: &mut Environment, bundle: CFBundleRef) -> CFStringRef {
    let ident: id = msg![env; bundle bundleIdentifier];
    if ident.is_null() {
        return std::ptr::null_mut();
    }
    msg![env; ident copy]
}

fn CFBundleGetInfoDictionary(env: &mut Environment, bundle: CFBundleRef) -> CFTypeRef {
    let dict: id = msg![env; bundle infoDictionary];
    if dict.is_null() {
        return std::ptr::null_mut();
    }
    retain(env, dict);
    dict
}

fn CFBundleGetLocalInfoDictionary(env: &mut Environment, bundle: CFBundleRef) -> CFTypeRef {
    let dict: id = msg![env; bundle localizedInfoDictionary];
    if dict.is_null() {
        return std::ptr::null_mut();
    }
    retain(env, dict);
    dict
}

fn CFBundleGetExecutableURL(env: &mut Environment, bundle: CFBundleRef) -> CFURLRef {
    let url: id = msg![env; bundle executableURL];
    if url.is_null() {
        return std::ptr::null_mut();
    }
    msg![env; url copy]
}

fn CFBundleGetDevelopmentRegion(env: &mut Environment, bundle: CFBundleRef) -> CFStringRef {
    let key = ns_string::get_static_str(env, "CFBundleDevelopmentRegion");
    let val = CFBundleGetValueForInfoDictionaryKey(env, bundle, key);
    if val.is_null() {
        return std::ptr::null_mut();
    }
    msg![env; val copy]
}

fn CFBundleIsExecutableLoaded(_env: &mut Environment, _bundle: CFBundleRef) -> bool {
    // Matches observed macOS behavior for main bundle
    true
}

fn CFBundleLoadExecutable(_env: &mut Environment, _bundle: CFBundleRef) -> bool {
    true
}

fn CFBundleUnloadExecutable(_env: &mut Environment, _bundle: CFBundleRef) -> bool {
    true
}

fn CFBundleCopyInfoDictionaryInDirectory(
    env: &mut Environment,
    bundle_url: CFURLRef,
) -> CFTypeRef {
    if bundle_url.is_null() {
        return std::ptr::null_mut();
    }

    let bundle: id = msg_class![env; NSBundle bundleWithURL:bundle_url];
    if bundle.is_null() {
        return std::ptr::null_mut();
    }

    let dict: id = msg![env; bundle infoDictionary];
    if dict.is_null() {
        return std::ptr::null_mut();
    }

    retain(env, dict);
    dict
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFBundleGetMainBundle()),
    export_c_func!(CFBundleGetValueForInfoDictionaryKey(_, _)),
    export_c_func!(CFBundleGetVersionNumber(_)),
    export_c_func!(CFBundleCopyBundleURL(_)),
    export_c_func!(CFBundleCopyResourcesDirectoryURL(_)),
    export_c_func!(CFBundleCopyResourceURL(_, _, _, _)),
    export_c_func!(CFBundleCopyBundleLocalizations(_)),
    export_c_func!(CFBundleCopyPreferredLocalizationsFromArray(_)),
    export_c_func!(CFBundleCopyLocalizedString(_, _, _, _)),
    export_c_func!(CFBundleGetIdentifier(_)),
    export_c_func!(CFBundleGetInfoDictionary(_)),
    export_c_func!(CFBundleGetLocalInfoDictionary(_)),
    export_c_func!(CFBundleGetExecutableURL(_)),
    export_c_func!(CFBundleGetDevelopmentRegion(_)),
    export_c_func!(CFBundleIsExecutableLoaded(_)),
    export_c_func!(CFBundleLoadExecutable(_)),
    export_c_func!(CFBundleUnloadExecutable(_)),
    export_c_func!(CFBundleCopyInfoDictionaryInDirectory(_)),
];
