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
pub(crate) fn as_str<'path>(path: &'path [u8]) -> &'path str {
    unsafe { str::from_utf8_unchecked(path) }
}

pub(crate) fn as_osstr<'path>(path: &'path [u8]) -> &'path OsStr {
    OsStr::new(as_str(path))
}

pub(crate) fn is_char(path: &str, index: usize) -> bool {
    let cur_is_boundary = path.is_char_boundary(index);
    let ret = if index == path.len() - 1 {
        cur_is_boundary && path.is_char_boundary(index - 1)
    } else if index == 0 {
        cur_is_boundary && path.is_char_boundary(index + 1)
    } else {
        cur_is_boundary
            && path.is_char_boundary(index + 1)
            && path.is_char_boundary(index - 1)
    };

    return ret;
}

// ===========================================================================
//
// ===========================================================================
