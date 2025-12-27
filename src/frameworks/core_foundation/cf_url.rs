/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CFURL`.
//!
//! This is toll-free bridged to `NSURL` in Apple's implementation. Here it is
//! the same type.

use super::cf_allocator::{kCFAllocatorDefault, CFAllocatorRef};
use super::CFIndex;
use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::core_foundation::cf_string::{
    kCFStringEncodingASCII, CFStringConvertEncodingToNSStringEncoding, CFStringEncoding,
    CFStringRef,
};
use crate::frameworks::foundation::ns_string::{
    get_static_str, to_rust_string, NSUTF8StringEncoding,
};
use crate::frameworks::foundation::NSUInteger;
use crate::mem::{ConstPtr, MutPtr, Ptr};
use crate::objc::{id, msg, msg_class, release};
use crate::Environment;

pub type CFURLRef = super::CFTypeRef;

type CFURLPathStyle = CFIndex;
const kCFURLPOSIXPathStyle: CFURLPathStyle = 0;
#[allow(dead_code)]
const kCFURLHFSPathStyle: CFURLPathStyle = 1;
#[allow(dead_code)]
const kCFURLWindowsPathStyle: CFURLPathStyle = 2;

pub fn CFURLGetFileSystemRepresentation(
    env: &mut Environment,
    url: CFURLRef,
    resolve_against_base: bool,
    buffer: MutPtr<u8>,
    buffer_size: CFIndex,
) -> bool {
    if resolve_against_base {
        // this function usually called to resolve resources from the main
        // bundle
        // thus, the url should already be an absolute path name
        // TODO: use absoluteURL instead once implemented
        let path = msg![env; url path];
        // TODO: avoid copy
        assert!(to_rust_string(env, path).starts_with('/'));
    }
    let buffer_size: NSUInteger = buffer_size.try_into().unwrap();

    msg![env; url getFileSystemRepresentation:buffer
                                    maxLength:buffer_size]
}

pub fn CFURLCreateFromFileSystemRepresentation(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    buffer: ConstPtr<u8>,
    buffer_size: CFIndex,
    is_directory: bool,
) -> CFURLRef {
    assert!(allocator == kCFAllocatorDefault); // unimplemented

    let buffer_size: NSUInteger = buffer_size.try_into().unwrap();

    let string: id = msg_class![env; NSString alloc];
    let string: id = msg![env; string initWithBytes:buffer
                                             length:buffer_size
                                           encoding:NSUTF8StringEncoding];

    let url: id = msg_class![env; NSURL alloc];
    let res = msg![env; url initFileURLWithPath:string isDirectory:is_directory];
    release(env, string);
    res
}

fn CFURLCreateWithBytes(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    url_bytes: ConstPtr<u8>,
    length: CFIndex,
    encoding: CFStringEncoding,
    base_url: CFURLRef,
) -> CFURLRef {
    assert_eq!(allocator, kCFAllocatorDefault); // unimplemented
    assert_eq!(encoding, kCFStringEncodingASCII); // TODO
    assert!(base_url.is_null()); // TODO

    // TODO: interpret percent escape sequences using encoding as well
    let encoding = CFStringConvertEncodingToNSStringEncoding(env, encoding);
    let length: NSUInteger = length.try_into().unwrap();

    if length == 0 {
        return Ptr::null();
    }

    let string: id = msg_class![env; NSString alloc];
    let string: id = msg![env; string initWithBytes:url_bytes
                                             length:length
                                           encoding:encoding];

    assert!(!to_rust_string(env, string).contains("://")); // TODO

    // Assume file URL case here
    let url: id = msg_class![env; NSURL alloc];
    let res = msg![env; url initFileURLWithPath:string];
    release(env, string);
    res
}

fn CFURLCreateWithFileSystemPath(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    file_path: CFStringRef,
    style: CFURLPathStyle,
    is_directory: bool,
) -> CFURLRef {
    assert!(allocator == kCFAllocatorDefault); // unimplemented
    assert_eq!(style, kCFURLPOSIXPathStyle);
    let url: id = msg_class![env; NSURL alloc];
    msg![env; url initFileURLWithPath:file_path isDirectory:is_directory]
}

pub fn CFURLCopyPathExtension(env: &mut Environment, url: CFURLRef) -> CFStringRef {
    let path = msg![env; url path];
    let ext = msg![env; path pathExtension];
    msg![env; ext copy]
}

fn CFURLCopyFileSystemPath(
    env: &mut Environment,
    url: CFURLRef,
    style: CFURLPathStyle,
) -> CFStringRef {
    assert_eq!(style, kCFURLPOSIXPathStyle);
    let path: CFStringRef = msg![env; url path];
    msg![env; path copy]
}

fn CFURLCreateCopyAppendingPathComponent(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    url: CFURLRef,
    path_component: CFStringRef,
    is_directory: bool,
) -> CFURLRef {
    assert!(allocator.is_null());
    let new_url =
        msg![env; url URLByAppendingPathComponent:path_component isDirectory:is_directory];
    msg![env; new_url copy]
}

fn CFURLCreateCopyDeletingLastPathComponent(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    url: CFURLRef,
) -> CFURLRef {
    assert!(allocator.is_null());
    let new_url = msg![env; url URLByDeletingLastPathComponent];
    msg![env; new_url copy]
}

fn CFURLHasDirectoryPath(env: &mut Environment, url: CFURLRef) -> bool {
    assert!(!url.is_null());

    let path = msg![env; url path];
    if msg![env; path isEqual:(get_static_str(env, "//"))] {
        // Special case
        return false;
    }
    // Note: cannot use `lastPathComponent` here!
    let components: id = msg![env; path pathComponents];
    let count: NSUInteger = msg![env; components count];
    if count == 0 {
        return false;
    }
    let last: id = msg![env; components objectAtIndex:(count - 1)];
    msg![env; last isEqual:(get_static_str(env, "/"))]
        || msg![env; last isEqual:(get_static_str(env, "."))]
        || msg![env; last isEqual:(get_static_str(env, ".."))]
}

pub fn CFURLCopyLastPathComponent(env: &mut Environment, url: CFURLRef) -> CFStringRef {
    if url.is_null() {
        return Ptr::null();
    }
    let path = msg![env; url path];
    let last = msg![env; path lastPathComponent];
    msg![env; last copy]
}

pub fn CFURLCopyPath(env: &mut Environment, url: CFURLRef) -> CFStringRef {
    if url.is_null() {
        return Ptr::null();
    }
    let path: CFStringRef = msg![env; url path];
    msg![env; path copy]
}

pub fn CFURLGetString(env: &mut Environment, url: CFURLRef) -> CFStringRef {
    if url.is_null() {
        return Ptr::null();
    }
    let s: CFStringRef = msg![env; url absoluteString];
    s
}

pub fn CFURLCopyAbsoluteURL(env: &mut Environment, url: CFURLRef) -> CFURLRef {
    if url.is_null() {
        return Ptr::null();
    }
    let abs: id = msg![env; url absoluteURL];
    msg![env; abs copy]
}

pub fn CFURLIsFileReferenceURL(_env: &mut Environment, _url: CFURLRef) -> bool {
    // File reference URLs not currently distinguished
    false
}

pub fn CFURLCreateCopyStandardizingPath(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    url: CFURLRef,
) -> CFURLRef {
    if url.is_null() {
        return Ptr::null();
    }
    assert!(allocator.is_null() || allocator == kCFAllocatorDefault);

    let std: id = msg![env; url URLByStandardizingPath];
    msg![env; std copy]
}

pub fn CFURLCreateCopyResolvingSymlinksInPath(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    url: CFURLRef,
) -> CFURLRef {
    if url.is_null() {
        return Ptr::null();
    }
    assert!(allocator.is_null() || allocator == kCFAllocatorDefault);

    let resolved: id = msg![env; url URLByResolvingSymlinksInPath];
    msg![env; resolved copy]
}

pub fn CFURLCreateStringByAddingPercentEscapes(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    original_string: CFStringRef,
    characters_to_leave_unescaped: CFStringRef,
    legal_url_characters_to_be_escaped: CFStringRef,
    encoding: CFStringEncoding,
) -> CFStringRef {
    // Allocator handling (only default supported)
    assert!(allocator.is_null() || allocator == kCFAllocatorDefault);

    if original_string.is_null() {
        return Ptr::null();
    }

    // Convert CFStringEncoding → NSStringEncoding
    let ns_encoding =
        CFStringConvertEncodingToNSStringEncoding(env, encoding);

    // Bridge to NSString
    let ns_string: id = original_string;

    // Use NSString percent-encoding API
    let escaped: id = msg![env;
        ns_string stringByAddingPercentEscapesUsingEncoding:ns_encoding
    ];

    if escaped.is_null() {
        return Ptr::null();
    }

    msg![env; escaped copy]
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFURLGetFileSystemRepresentation(_, _, _, _)),
    export_c_func!(CFURLCreateFromFileSystemRepresentation(_, _, _, _)),
    export_c_func!(CFURLCreateWithBytes(_, _, _, _, _)),
    export_c_func!(CFURLCreateWithFileSystemPath(_, _, _, _)),
    export_c_func!(CFURLCopyPathExtension(_)),
    export_c_func!(CFURLCopyFileSystemPath(_, _)),
    export_c_func!(CFURLCreateCopyAppendingPathComponent(_, _, _, _)),
    export_c_func!(CFURLCreateCopyDeletingLastPathComponent(_, _)),
    export_c_func!(CFURLHasDirectoryPath(_)),
    export_c_func!(CFURLCopyLastPathComponent(_)),
    export_c_func!(CFURLCopyPath(_)),
    export_c_func!(CFURLGetString(_)),
    export_c_func!(CFURLCopyAbsoluteURL(_)),
    export_c_func!(CFURLIsFileReferenceURL(_)),
    export_c_func!(CFURLCreateCopyStandardizingPath(_, _)),
    export_c_func!(CFURLCreateCopyResolvingSymlinksInPath(_, _)),
    export_c_func!(CFURLCreateStringByAddingPercentEscapes(_, _, _, _, _)),
];
