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

// ===========================================================================
//
// ===========================================================================
