/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `clocale.h`

use std::collections::hash_map::Entry;

use crate::dyld::FunctionExports;
use crate::environment::Environment;
use crate::export_c_func;
use crate::mem::{ConstPtr, MutPtr, SafeRead};

pub type LocaleCategory = i32;
pub const LC_ALL: LocaleCategory = 0;
pub const LC_COLLATE: LocaleCategory = 1;
pub const LC_CTYPE: LocaleCategory = 2;
pub const LC_MONETARY: LocaleCategory = 3;
pub const LC_NUMERIC: LocaleCategory = 4;
pub const LC_TIME: LocaleCategory = 5;
pub const LC_MESSAGES: LocaleCategory = 6;

#[derive(Default)]
pub struct State {
    locale: std::collections::HashMap<LocaleCategory, MutPtr<u8>>,
}

unsafe impl SafeRead for lconv {}

#[repr(C)]
pub struct lconv {
    pub decimal_point: MutPtr<u8>,
    pub thousands_sep: MutPtr<u8>,
    pub grouping: MutPtr<u8>,

    pub int_curr_symbol: MutPtr<u8>,
    pub currency_symbol: MutPtr<u8>,
    pub mon_decimal_point: MutPtr<u8>,
    pub mon_thousands_sep: MutPtr<u8>,
    pub mon_grouping: MutPtr<u8>,
    pub positive_sign: MutPtr<u8>,
    pub negative_sign: MutPtr<u8>,

    pub int_frac_digits: i8,
    pub frac_digits: i8,
    pub p_cs_precedes: i8,
    pub p_sep_by_space: i8,
    pub n_cs_precedes: i8,
    pub n_sep_by_space: i8,
    pub p_sign_posn: i8,
    pub n_sign_posn: i8,
}

fn localeconv(env: &mut Environment) -> MutPtr<lconv> {
    // C locale defaults
    let conv = lconv {
        decimal_point: env.mem.alloc_and_write_cstr(b"."),
        thousands_sep: env.mem.alloc_and_write_cstr(b""),
        grouping: env.mem.alloc_and_write_cstr(b""),

        int_curr_symbol: env.mem.alloc_and_write_cstr(b""),
        currency_symbol: env.mem.alloc_and_write_cstr(b""),
        mon_decimal_point: env.mem.alloc_and_write_cstr(b""),
        mon_thousands_sep: env.mem.alloc_and_write_cstr(b""),
        mon_grouping: env.mem.alloc_and_write_cstr(b""),
        positive_sign: env.mem.alloc_and_write_cstr(b""),
        negative_sign: env.mem.alloc_and_write_cstr(b""),

        int_frac_digits: -1,
        frac_digits: -1,
        p_cs_precedes: -1,
        p_sep_by_space: -1,
        n_cs_precedes: -1,
        n_sep_by_space: -1,
        p_sign_posn: -1,
        n_sign_posn: -1,
    };

    // Allocate once per process
    env.mem.alloc_and_write(conv)
}

pub fn setlocale(
    env: &mut Environment,
    category: LocaleCategory,
    locale: ConstPtr<u8>,
) -> MutPtr<u8> {
    assert!(matches!(
        category,
        LC_ALL | LC_COLLATE | LC_CTYPE | LC_MONETARY | LC_NUMERIC | LC_TIME | LC_MESSAGES
    ));
    if !locale.is_null() {
        // TODO: Handle empty locale string and ensure the combination of
        // category and locale is valid.
        let locale_cstr = env.mem.cstr_at(locale).to_owned();
        assert_ne!(locale_cstr.len(), 0);
        let new_locale = env.mem.alloc_and_write_cstr(locale_cstr.as_slice());
        if let Some(old_locale) = env.libc_state.clocale.locale.insert(category, new_locale) {
            env.mem.free(old_locale.cast())
        };
    } else if let Entry::Vacant(entry) = env.libc_state.clocale.locale.entry(category) {
        let default_locale = env.mem.alloc_and_write_cstr(b"C");
        entry.insert(default_locale);
    }
    env.libc_state.clocale.locale.get(&category).unwrap().cast()
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(setlocale(_, _)),
    export_c_func!(localeconv()),
];
