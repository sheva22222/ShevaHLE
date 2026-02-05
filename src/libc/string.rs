/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `string.h`

use crate::dyld::{export_c_func, FunctionExports};
use crate::mem::{ConstPtr, ConstVoidPtr, GuestUSize, MutPtr, MutVoidPtr, Ptr};
use crate::Environment;
use std::cmp::Ordering;

use super::generic_char::GenericChar;

#[derive(Default)]
pub struct State {
    strtok: Option<MutPtr<u8>>,
}

fn ffs_impl(x: i32) -> i32 {
    if x == 0 {
        0
    } else {
        x.trailing_zeros() as i32 + 1
    }
}

fn ffsl_impl(x: i64) -> i32 {
    if x == 0 {
        0
    } else {
        x.trailing_zeros() as i32 + 1
    }
}

fn ffsll_impl(x: i64) -> i32 {
    ffsl_impl(x)
}

fn strtok(env: &mut Environment, s: MutPtr<u8>, sep: ConstPtr<u8>) -> MutPtr<u8> {
    let s = if s.is_null() {
        let state = env.libc_state.string.strtok.unwrap();
        if state.is_null() {
            env.libc_state.string.strtok = None;
            return Ptr::null();
        }
        state
    } else {
        s
    };

    let sep = env.mem.cstr_at(sep);

    let mut token_start = s;
    loop {
        let c = env.mem.read(token_start);
        if c == b'\0' {
            env.libc_state.string.strtok = None;
            return Ptr::null();
        } else if sep.contains(&c) {
            token_start += 1;
        } else {
            break;
        }
    }

    let mut token_end = token_start;
    let next_token = loop {
        let c = env.mem.read(token_end);
        if sep.contains(&c) {
            env.mem.write(token_end, b'\0');
            break token_end + 1;
        } else if c == b'\0' {
            break Ptr::null();
        } else {
            token_end += 1;
        }
    };

    env.libc_state.string.strtok = Some(next_token);

    token_start
}

// Functions shared with wchar.rs

fn bzero(env: &mut Environment, dest: MutVoidPtr, count: GuestUSize) {
    memset(env, dest, 0, count);
}
fn memset(env: &mut Environment, dest: MutVoidPtr, ch: i32, count: GuestUSize) -> MutVoidPtr {
    GenericChar::<u8>::memset(env, dest.cast(), ch as u8, count, GuestUSize::MAX).cast()
}
fn __memset_chk(
    env: &mut Environment,
    dest: MutVoidPtr,
    ch: i32,
    count: GuestUSize,
    dest_count: GuestUSize,
) -> MutVoidPtr {
    GenericChar::<u8>::memset(env, dest.cast(), ch as u8, count, dest_count).cast()
}
fn memset_pattern4(env: &mut Environment, b: MutVoidPtr, pattern4: ConstVoidPtr, len: GuestUSize) {
    memset_pattern_inner(env, b, pattern4, len, 4)
}
fn memset_pattern8(env: &mut Environment, b: MutVoidPtr, pattern8: ConstVoidPtr, len: GuestUSize) {
    memset_pattern_inner(env, b, pattern8, len, 8)
}
fn memset_pattern16(
    env: &mut Environment,
    b: MutVoidPtr,
    pattern16: ConstVoidPtr,
    len: GuestUSize,
) {
    memset_pattern_inner(env, b, pattern16, len, 16)
}
fn memset_pattern_inner(
    env: &mut Environment,
    b: MutVoidPtr,
    pattern: ConstVoidPtr,
    len: GuestUSize,
    pattern_len: GuestUSize,
) {
    assert!(matches!(pattern_len, 4 | 8 | 16));
    let mut tmp = [0; 16];
    tmp[..pattern_len as usize].copy_from_slice(env.mem.bytes_at(pattern.cast(), pattern_len));
    let mut target: MutPtr<u8> = b.cast();
    for _ in 0..(len / pattern_len) {
        env.mem
            .bytes_at_mut(target, pattern_len)
            .copy_from_slice(&tmp[..pattern_len as usize]);
        target += pattern_len;
    }
    for i in 0..(len % pattern_len) {
        env.mem.write(target, env.mem.read(pattern.cast() + i));
        target += 1;
    }
}
fn memcpy(
    env: &mut Environment,
    dest: MutVoidPtr,
    src: ConstVoidPtr,
    size: GuestUSize,
) -> MutVoidPtr {
    GenericChar::<u8>::memcpy(env, dest.cast(), src.cast(), size, GuestUSize::MAX).cast()
}
fn __memcpy_chk(
    env: &mut Environment,
    dest: MutVoidPtr,
    src: ConstVoidPtr,
    size: GuestUSize,
    dest_size: GuestUSize,
) -> MutVoidPtr {
    GenericChar::<u8>::memcpy(env, dest.cast(), src.cast(), size, dest_size).cast()
}
fn memmove(
    env: &mut Environment,
    dest: MutVoidPtr,
    src: ConstVoidPtr,
    size: GuestUSize,
) -> MutVoidPtr {
    GenericChar::<u8>::memmove(env, dest.cast(), src.cast(), size, GuestUSize::MAX).cast()
}
fn __memmove_chk(
    env: &mut Environment,
    dest: MutVoidPtr,
    src: ConstVoidPtr,
    size: GuestUSize,
    dest_size: GuestUSize,
) -> MutVoidPtr {
    GenericChar::<u8>::memmove(env, dest.cast(), src.cast(), size, dest_size).cast()
}
fn memchr(env: &mut Environment, string: ConstVoidPtr, c: i32, size: GuestUSize) -> ConstVoidPtr {
    GenericChar::<u8>::memchr(env, string.cast(), c as u8, size).cast()
}
fn memcmp(env: &mut Environment, a: ConstVoidPtr, b: ConstVoidPtr, size: GuestUSize) -> i32 {
    GenericChar::<u8>::memcmp(env, a.cast(), b.cast(), size)
}
pub(super) fn strlen(env: &mut Environment, s: ConstPtr<u8>) -> GuestUSize {
    GenericChar::<u8>::strlen(env, s)
}
pub(super) fn strcpy(env: &mut Environment, dest: MutPtr<u8>, src: ConstPtr<u8>) -> MutPtr<u8> {
    GenericChar::<u8>::strcpy(env, dest, src, GuestUSize::MAX)
}
fn __strcpy_chk(
    env: &mut Environment,
    dest: MutPtr<u8>,
    src: ConstPtr<u8>,
    size: GuestUSize,
) -> MutPtr<u8> {
    GenericChar::<u8>::strcpy(env, dest, src, size)
}
fn strcat(env: &mut Environment, dest: MutPtr<u8>, src: ConstPtr<u8>) -> MutPtr<u8> {
    GenericChar::<u8>::strcat(env, dest, src, GuestUSize::MAX)
}
fn __strcat_chk(
    env: &mut Environment,
    dest: MutPtr<u8>,
    src: ConstPtr<u8>,
    size: GuestUSize,
) -> MutPtr<u8> {
    GenericChar::<u8>::strcat(env, dest, src, size)
}
fn strcspn(env: &mut Environment, s: ConstPtr<u8>, charset: ConstPtr<u8>) -> GuestUSize {
    GenericChar::<u8>::strcspn(env, s, charset)
}
pub(crate) fn strncpy(
    env: &mut Environment,
    dest: MutPtr<u8>,
    src: ConstPtr<u8>,
    size: GuestUSize,
) -> MutPtr<u8> {
    GenericChar::<u8>::strncpy(env, dest, src, size)
}
fn strsep(env: &mut Environment, stringp: MutPtr<MutPtr<u8>>, delim: ConstPtr<u8>) -> MutPtr<u8> {
    let orig = env.mem.read(stringp);
    if orig.is_null() {
        return Ptr::null();
    }
    let tmp = orig;
    let mut i = 0;
    loop {
        let c = env.mem.read(tmp + i);
        if c == b'\0' {
            env.mem.write(stringp, Ptr::null());
            break;
        }
        let mut j = 0;
        loop {
            let cc = env.mem.read(delim + j);
            if c == cc {
                env.mem.write(tmp + i, b'\0');
                env.mem.write(stringp, tmp + i + 1);
                return orig;
            }
            if cc == b'\0' {
                break;
            }
            j += 1;
        }
        i += 1;
    }
    orig
}
pub(super) fn strdup(env: &mut Environment, src: ConstPtr<u8>) -> MutPtr<u8> {
    GenericChar::<u8>::strdup(env, src)
}
pub fn strcmp(env: &mut Environment, a: ConstPtr<u8>, b: ConstPtr<u8>) -> i32 {
    GenericChar::<u8>::strcmp(env, a, b)
}
fn strncmp(env: &mut Environment, a: ConstPtr<u8>, b: ConstPtr<u8>, n: GuestUSize) -> i32 {
    GenericChar::<u8>::strncmp(env, a, b, n)
}
fn strcasecmp(env: &mut Environment, a: ConstPtr<u8>, b: ConstPtr<u8>) -> i32 {
    // TODO: generalize to wide chars
    let mut offset = 0;
    loop {
        let char_a = env.mem.read(a + offset).to_ascii_lowercase();
        let char_b = env.mem.read(b + offset).to_ascii_lowercase();
        offset += 1;

        match char_a.cmp(&char_b) {
            Ordering::Less => return -1,
            Ordering::Greater => return 1,
            Ordering::Equal => {
                if char_a == u8::default() {
                    return 0;
                } else {
                    continue;
                }
            }
        }
    }
}
fn strncasecmp(env: &mut Environment, a: ConstPtr<u8>, b: ConstPtr<u8>, n: GuestUSize) -> i32 {
    // TODO: generalize to wide chars
    if n == 0 {
        return 0;
    }

    let mut offset = 0;
    loop {
        let char_a = env.mem.read(a + offset).to_ascii_lowercase();
        let char_b = env.mem.read(b + offset).to_ascii_lowercase();
        offset += 1;

        match char_a.cmp(&char_b) {
            Ordering::Less => return -1,
            Ordering::Greater => return 1,
            Ordering::Equal => {
                if offset == n || char_a == u8::default() {
                    return 0;
                } else {
                    continue;
                }
            }
        }
    }
}
fn strncat(env: &mut Environment, s1: MutPtr<u8>, s2: ConstPtr<u8>, n: GuestUSize) -> MutPtr<u8> {
    GenericChar::<u8>::strncat(env, s1, s2, n)
}
fn strstr(env: &mut Environment, string: ConstPtr<u8>, substring: ConstPtr<u8>) -> ConstPtr<u8> {
    GenericChar::<u8>::strstr(env, string, substring)
}
fn strchr(env: &mut Environment, path: ConstPtr<u8>, c: u8) -> ConstPtr<u8> {
    GenericChar::<u8>::strchr(env, path, c)
}
fn strrchr(env: &mut Environment, path: ConstPtr<u8>, c: u8) -> ConstPtr<u8> {
    GenericChar::<u8>::strrchr(env, path, c)
}
fn strlcpy(
    env: &mut Environment,
    dst: MutPtr<u8>,
    src: ConstPtr<u8>,
    size: GuestUSize,
) -> GuestUSize {
    GenericChar::<u8>::strlcpy(env, dst, src, size)
}

fn memccpy(
    env: &mut Environment,
    dest: MutVoidPtr,
    src: ConstVoidPtr,
    c: i32,
    n: GuestUSize,
) -> MutVoidPtr {
    let d: MutPtr<u8> = dest.cast();
    let s: ConstPtr<u8> = src.cast();

    for i in 0..n {
        let byte = env.mem.read(s + i);
        env.mem.write(d + i, byte);
        if byte == c as u8 {
            return (d + i + 1).cast();
        }
    }
    Ptr::null()
}

fn memrchr(
    env: &mut Environment,
    s: ConstVoidPtr,
    c: i32,
    n: GuestUSize,
) -> ConstVoidPtr {
    let p: ConstPtr<u8> = s.cast();
    let mut i = n;
    while i > 0 {
        i -= 1;
        if env.mem.read(p + i) == c as u8 {
            return (p + i).cast();
        }
    }
    Ptr::null()
}

fn stpcpy(env: &mut Environment, dest: MutPtr<u8>, src: ConstPtr<u8>) -> MutPtr<u8> {
    let len = GenericChar::<u8>::strlen(env, src);
    GenericChar::<u8>::strcpy(env, dest, src, GuestUSize::MAX);
    dest + len
}

fn stpncpy(
    env: &mut Environment,
    dest: MutPtr<u8>,
    src: ConstPtr<u8>,
    n: GuestUSize,
) -> MutPtr<u8> {
    let copied = GenericChar::<u8>::strncpy(env, dest, src, n);
    let len = GenericChar::<u8>::strlen(env, src);
    dest + core::cmp::min(len, n)
}

fn strpbrk(env: &mut Environment, s: ConstPtr<u8>, accept: ConstPtr<u8>) -> ConstPtr<u8> {
    let mut i = 0;
    loop {
        let c = env.mem.read(s + i);
        if c == 0 {
            return Ptr::null();
        }
        let mut j = 0;
        loop {
            let a = env.mem.read(accept + j);
            if a == 0 {
                break;
            }
            if a == c {
                return s + i;
            }
            j += 1;
        }
        i += 1;
    }
}

fn strspn(env: &mut Environment, s: ConstPtr<u8>, accept: ConstPtr<u8>) -> GuestUSize {
    let mut i = 0;
    loop {
        let c = env.mem.read(s + i);
        if c == 0 {
            return i;
        }
        let mut j = 0;
        let mut found = false;
        loop {
            let a = env.mem.read(accept + j);
            if a == 0 {
                break;
            }
            if a == c {
                found = true;
                break;
            }
            j += 1;
        }
        if !found {
            return i;
        }
        i += 1;
    }
}

fn strtok_r(
    env: &mut Environment,
    s: MutPtr<u8>,
    sep: ConstPtr<u8>,
    saveptr: MutPtr<MutPtr<u8>>,
) -> MutPtr<u8> {
    let mut s = if s.is_null() {
        env.mem.read(saveptr)
    } else {
        s
    };

    if s.is_null() {
        return Ptr::null();
    }

    let sep_bytes = env.mem.cstr_at(sep);

    while sep_bytes.contains(&env.mem.read(s)) {
        s += 1;
    }

    if env.mem.read(s) == 0 {
        env.mem.write(saveptr, Ptr::null());
        return Ptr::null();
    }

    let token = s;

    loop {
        let c = env.mem.read(s);
        if c == 0 {
            env.mem.write(saveptr, Ptr::null());
            break;
        }
        if sep_bytes.contains(&c) {
            env.mem.write(s, 0);
            env.mem.write(saveptr, s + 1);
            break;
        }
        s += 1;
    }

    token
}

fn strcasestr(
    env: &mut Environment,
    haystack: ConstPtr<u8>,
    needle: ConstPtr<u8>,
) -> ConstPtr<u8> {
    let needle_len = GenericChar::<u8>::strlen(env, needle);
    if needle_len == 0 {
        return haystack;
    }

    let mut i = 0;
    loop {
        let h = env.mem.read(haystack + i);
        if h == 0 {
            return Ptr::null();
        }

        let mut matched = true;
        for j in 0..needle_len {
            let a = env.mem.read(haystack + i + j).to_ascii_lowercase();
            let b = env.mem.read(needle + j).to_ascii_lowercase();
            if a != b {
                matched = false;
                break;
            }
        }

        if matched {
            return haystack + i;
        }

        i += 1;
    }
}

fn explicit_bzero(env: &mut Environment, s: MutVoidPtr, n: GuestUSize) {
    let mut p: MutPtr<u8> = s.cast();
    for _ in 0..n {
        env.mem.write(p, 0);
        p += 1;
    }
}

fn memmem(
    env: &mut Environment,
    haystack: ConstVoidPtr,
    haystack_len: GuestUSize,
    needle: ConstVoidPtr,
    needle_len: GuestUSize,
) -> ConstVoidPtr {
    if needle_len == 0 {
        return haystack;
    }
    if needle_len > haystack_len {
        return Ptr::null();
    }

    let h: ConstPtr<u8> = haystack.cast();
    let n: ConstPtr<u8> = needle.cast();

    for i in 0..=(haystack_len - needle_len) {
        let mut match_all = true;
        for j in 0..needle_len {
            if env.mem.read(h + i + j) != env.mem.read(n + j) {
                match_all = false;
                break;
            }
        }
        if match_all {
            return (h + i).cast();
        }
    }

    Ptr::null()
}

fn ffs(_env: &mut Environment, x: i32) -> i32 {
    ffs_impl(x)
}

fn ffsl(_env: &mut Environment, x: i64) -> i32 {
    ffsl_impl(x)
}

fn ffsll(_env: &mut Environment, x: i64) -> i32 {
    ffsll_impl(x)
}


pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(strtok(_, _)),
    export_c_func!(bzero(_, _)),
    // Functions shared with wchar.rs
    export_c_func!(memset(_, _, _)),
    export_c_func!(__memset_chk(_, _, _, _)),
    export_c_func!(memset_pattern4(_, _, _)),
    export_c_func!(memset_pattern8(_, _, _)),
    export_c_func!(memset_pattern16(_, _, _)),
    export_c_func!(memcpy(_, _, _)),
    export_c_func!(__memcpy_chk(_, _, _, _)),
    export_c_func!(memmove(_, _, _)),
    export_c_func!(__memmove_chk(_, _, _, _)),
    export_c_func!(memchr(_, _, _)),
    export_c_func!(memcmp(_, _, _)),
    export_c_func!(strlen(_)),
    export_c_func!(strcpy(_, _)),
    export_c_func!(__strcpy_chk(_, _, _)),
    export_c_func!(strcat(_, _)),
    export_c_func!(strcspn(_, _)),
    export_c_func!(__strcat_chk(_, _, _)),
    export_c_func!(strncpy(_, _, _)),
    export_c_func!(strsep(_, _)),
    export_c_func!(strdup(_)),
    export_c_func!(strcmp(_, _)),
    export_c_func!(strncmp(_, _, _)),
    export_c_func!(strcasecmp(_, _)),
    export_c_func!(strncasecmp(_, _, _)),
    export_c_func!(strncat(_, _, _)),
    export_c_func!(strstr(_, _)),
    export_c_func!(strchr(_, _)),
    export_c_func!(strrchr(_, _)),
    export_c_func!(strlcpy(_, _, _)),
    export_c_func!(memccpy(_, _, _, _)),
    export_c_func!(memrchr(_, _, _)),
    export_c_func!(stpcpy(_, _)),
    export_c_func!(stpncpy(_, _, _)),
    export_c_func!(strpbrk(_, _)),
    export_c_func!(strspn(_, _)),
    export_c_func!(strtok_r(_, _, _)),
    export_c_func!(explicit_bzero(_, _)),
    export_c_func!(strcasestr(_, _)),
    export_c_func!(memmem(_, _, _, _)),
    export_c_func!(ffs(_)),
    export_c_func!(ffsl(_)),
    export_c_func!(ffsll(_)),
];
