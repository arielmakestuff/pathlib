// src/common/string.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsStr;
use std::str;

// Third-party imports

// Local imports

// ===========================================================================
// String helpers
// ===========================================================================

// The unsafe is safe since we're not modifying the slice at all, and we will
// only be checking for ascii characters
pub(crate) fn as_str(path: &[u8]) -> &str {
    unsafe { str::from_utf8_unchecked(path) }
}

pub(crate) fn as_osstr(path: &[u8]) -> &OsStr {
    OsStr::new(as_str(path))
}

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
pub(crate) fn os_str_as_bytes(s: &OsStr) -> &[u8] {
    unsafe { &*(s as *const OsStr as *const [u8]) }
}

pub(crate) fn ascii_uppercase(letter: u8) -> u8 {
    (letter as char).to_ascii_uppercase() as u8
}

// ===========================================================================
//
// ===========================================================================
