/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CFSocket`

use super::cf_allocator::{kCFAllocatorDefault, CFAllocatorRef};
use super::CFTypeRef;
use crate::dyld::{export_c_func, FunctionExports};
use crate::mem::{MutVoidPtr, Ptr};
use crate::Environment;

fn CFSocketCreate(
    _env: &mut Environment,
    allocator: CFAllocatorRef,
    protocol_family: i32,
    type_: i32,
    protocol: i32,
    flags: u32,
    callout: MutVoidPtr,
    context: MutVoidPtr,
) -> CFTypeRef {
    assert_eq!(allocator, kCFAllocatorDefault); // unimplemented
    log!(
        "TODO: CFSocketCreate({}, {}, {}, {}, {:?}, {:?}) -> NULL",
        protocol_family,
        type_,
        protocol,
        flags,
        callout,
        context
    );
    Ptr::null()
}

fn CFSocketInvalidate(_env: &mut Environment, socket: CFTypeRef) {
    log!("TODO: CFSocketInvalidate({:?})", socket);
}

fn CFSocketIsValid(_env: &mut Environment, socket: CFTypeRef) -> bool {
    // NULL sockets are invalid; others are assumed valid for now
    !socket.is_null()
}

fn CFSocketGetNative(_env: &mut Environment, _socket: CFTypeRef) -> i32 {
    // No real socket backing yet
    -1
}

fn CFSocketSetAddress(
    _env: &mut Environment,
    socket: CFTypeRef,
    address: CFTypeRef,
) -> i32 {
    log!(
        "TODO: CFSocketSetAddress({:?}, {:?}) -> success",
        socket,
        address
    );
    0 // success
}

fn CFSocketSendData(
    _env: &mut Environment,
    socket: CFTypeRef,
    address: CFTypeRef,
    data: CFTypeRef,
    _timeout: f64,
) -> i32 {
    log!(
        "TODO: CFSocketSendData({:?}, {:?}, {:?}) -> success",
        socket,
        address,
        data
    );
    0 // success
}

fn CFSocketDisableCallBacks(
    _env: &mut Environment,
    socket: CFTypeRef,
    callback_types: u32,
) {
    log!(
        "TODO: CFSocketDisableCallBacks({:?}, {})",
        socket,
        callback_types
    );
}

fn CFSocketEnableCallBacks(
    _env: &mut Environment,
    socket: CFTypeRef,
    callback_types: u32,
) {
    log!(
        "TODO: CFSocketEnableCallBacks({:?}, {})",
        socket,
        callback_types
    );
}

fn CFSocketCopyAddress(_env: &mut Environment, socket: CFTypeRef) -> CFTypeRef {
    log!("TODO: CFSocketCopyAddress({:?}) -> NULL", socket);
    Ptr::null()
}

fn CFSocketCopyPeerAddress(_env: &mut Environment, socket: CFTypeRef) -> CFTypeRef {
    log!(
        "TODO: CFSocketCopyPeerAddress({:?}) -> NULL",
        socket
    );
    Ptr::null()
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFSocketCreate(_, _, _, _, _, _, _)),
    export_c_func!(CFSocketInvalidate(_)),
    export_c_func!(CFSocketIsValid(_)),
    export_c_func!(CFSocketGetNative(_)),
    export_c_func!(CFSocketSetAddress(_, _)),
    export_c_func!(CFSocketSendData(_, _, _, _)),
    export_c_func!(CFSocketDisableCallBacks(_, _)),
    export_c_func!(CFSocketEnableCallBacks(_, _)),
    export_c_func!(CFSocketCopyAddress(_)),
    export_c_func!(CFSocketCopyPeerAddress(_)),
];
