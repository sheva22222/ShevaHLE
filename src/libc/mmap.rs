/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::dyld::FunctionExports;
use crate::environment::Environment;
use crate::export_c_func;
use crate::libc::errno::{set_errno, EINVAL};
use crate::libc::posix_io;
use crate::libc::posix_io::{off_t, FileDescriptor, SEEK_SET};
use crate::mem::{GuestUSize, MutVoidPtr};
use std::collections::HashMap;

#[allow(dead_code)]
const MAP_FILE: i32 = 0x0000;
const MAP_ANON: i32 = 0x1000;
const MAP_PRIVATE: i32 = 0x0002;
const MAP_SHARED: i32 = 0x0001;


#[derive(Default)]
pub struct State {
    /// Keeping track of `mmap` allocations
    allocations: HashMap<MutVoidPtr, GuestUSize>,
}

struct MmapRegion {
    ptr: MutVoidPtr,
    len: GuestUSize,
    fd: Option<FileDescriptor>,
    offset: off_t,
    shared: bool,
}

/// Our implementation of mmap is really simple: it's just load entirety of
/// file in memory!
fn mmap(
    env: &mut Environment,
    addr: MutVoidPtr,
    len: GuestUSize,
    _prot: i32,
    flags: i32,
    fd: FileDescriptor,
    offset: off_t,
) -> MutVoidPtr {
    set_errno(env, 0);

    // POSIX: zero-length mapping is invalid
    if len == 0 {
        set_errno(env, EINVAL);
        return MutVoidPtr::null();
    }

    // addr is a hint → ignore
    if !addr.is_null() {
        log_dbg!("mmap: ignoring addr hint {:?}", addr);
    }

    // Offset must be page-aligned
    if offset % crate::mem::PAGE_SIZE as off_t != 0 {
        set_errno(env, EINVAL);
        return MutVoidPtr::null();
    }

    let is_anon = (flags & MAP_ANON) != 0;
    let is_shared = (flags & MAP_SHARED) != 0;
    let is_private = (flags & MAP_PRIVATE) != 0;

    // Exactly one of SHARED / PRIVATE must be set
    if is_shared == is_private {
        set_errno(env, EINVAL);
        return MutVoidPtr::null();
    }

    // Allocate memory
    let ptr = env.mem.alloc(len);

    // Track allocation
    env.libc_state
        .mmap
        .allocations
        .insert(ptr, len);

    // Always operate on bytes
    let byte_ptr = ptr.cast::<u8>();

    // Anonymous mapping → zero-fill
    if is_anon {
        for i in 0..len {
            env.mem.write(byte_ptr + i, 0u8);
        }
        return ptr;
    }

    // File-backed mapping
    if fd < 0 {
        set_errno(env, EINVAL);
        env.mem.free(ptr);
        env.libc_state.mmap.allocations.remove(&ptr);
        return MutVoidPtr::null();
    }

    // Save current offset
    let old_off = posix_io::lseek(env, fd, 0, SEEK_SET);

    // Seek to requested offset
    let new_off = posix_io::lseek(env, fd, offset, SEEK_SET);
    assert_eq!(new_off, offset);

    // Read file contents
    let read = posix_io::read(env, fd, ptr, len);

    // Zero-fill remainder
    let read = read as GuestUSize;
    for i in read..len {
        env.mem.write(byte_ptr + i, 0u8);
    }

    // Restore fd offset
    posix_io::lseek(env, fd, old_off, SEEK_SET);

    // MAP_SHARED currently behaves like MAP_PRIVATE
    // (writes affect memory only, msync is a no-op)

    ptr
}

fn munmap(env: &mut Environment, addr: MutVoidPtr, len: GuestUSize) -> i32 {
    set_errno(env, 0);

    if len == 0 {
        set_errno(env, EINVAL);
        return -1;
    }

    let Some(&alloc_len) = env.libc_state.mmap.allocations.get(&addr) else {
        set_errno(env, EINVAL);
        return -1;
    };

    if len != alloc_len {
        log!("Warning: partial munmap ignored ({}/{})", len, alloc_len);
    }

    env.mem.free(addr);
    env.libc_state.mmap.allocations.remove(&addr);
    0
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(mmap(_, _, _, _, _, _)),
    export_c_func!(munmap(_, _)),
];
