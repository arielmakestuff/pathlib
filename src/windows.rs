// src/windows.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

mod match_prefix;
mod path_type;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
// use std::cmp::PartialEq;
use std::collections::HashSet;
use std::path::Component;

// Third-party imports
use lazy_static::lazy_static;

// Local imports

// ===========================================================================
// Constants
// ===========================================================================

lazy_static! {
    static ref SEPARATOR: HashSet<u8> = {
        let sep_chars = r#"\/"#;
        let mut all_sep = HashSet::with_capacity(sep_chars.len());
        for s in sep_chars.chars() {
            all_sep.insert(s as u8);
        }
        all_sep
    };
    static ref DRIVE_LETTERS: HashSet<char> = {
        let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut all_letters = HashSet::with_capacity(letters.len());
        for l in letters.chars() {
            all_letters.insert(l);
        }
        all_letters
    };
    static ref RESERVED_NAMES: HashSet<String> = {
        let base = ["CON", "PRN", "AUX", "NUL"];
        let numbered_base = ["COM", "LPT"];
        let mut reserved = HashSet::with_capacity(22);
        for b in base.iter() {
            reserved.insert(b.to_string());
        }

        for b in numbered_base.iter() {
            for i in 1..=9 {
                reserved.insert(format!("{}{}", b, i));
            }
        }
        reserved
    };
    static ref RESTRICTED_CHARS: HashSet<u8> = {
        let chars = r#"<>:"/\|?*"#;
        let mut all_chars = HashSet::with_capacity(chars.len());
        for c in chars.chars() {
            all_chars.insert(c as u8);
        }

        // These are ascii chars code 0 - 31
        for i in 0..=31 {
            all_chars.insert(i);
        }
        all_chars
    };
}

// ===========================================================================
// PathIterator
// ===========================================================================

struct PathIterator<'path> {
    path: &'path [u8],
    cur: usize,
}

impl<'path> PathIterator<'path> {
    fn new(path: &[u8]) -> PathIterator {
        PathIterator { path: path, cur: 0 }
    }
}

impl<'path> Iterator for PathIterator<'path> {
    type Item = Component<'path>;

    fn next(&mut self) -> Option<Component<'path>> {
        unimplemented!()
    }
}

// ===========================================================================
//
// ===========================================================================
