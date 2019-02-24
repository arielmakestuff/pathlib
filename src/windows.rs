// src/windows.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

#[cfg(windows)]
pub mod path;

#[cfg(unix)]
mod unix_iter;

#[cfg(windows)]
mod windows_iter;

mod match_prefix;
mod path_type;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
// use std::cmp::PartialEq;
use std::collections::HashSet;

// Third-party imports
use lazy_static::lazy_static;

// Local imports
use crate::path::{Path, PathBuf};

// ===========================================================================
// Re-exports
// ===========================================================================

#[cfg(unix)]
pub use self::unix_iter::{
    Component, Iter, PathComponent, Prefix, PrefixComponent,
};

#[cfg(windows)]
pub use self::windows_iter::{
    Component, Iter, PathComponent, Prefix, PrefixComponent,
};

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
// Error types
// ===========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowsErrorKind {
    InvalidCharacter,
    RestrictedName,
}

// ===========================================================================
// Iter
// ===========================================================================

#[derive(Debug, Eq, PartialEq)]
enum PathParseState {
    Start,
    Prefix { verbatimdisk: bool },
    Root,
    PathComponent,
    Finish,
}

// ===========================================================================
// Traits
// ===========================================================================

pub trait WindowsMemoryPath<'path> {
    fn iter(&'path self) -> Iter<'path>;
}

pub trait WindowsMemoryPathBuf<'path>: WindowsMemoryPath<'path> {}

impl<'path> WindowsMemoryPath<'path> for Path {
    fn iter(&'path self) -> Iter<'path> {
        Iter::new(self)
    }
}

impl<'path> WindowsMemoryPath<'path> for PathBuf {
    fn iter(&'path self) -> Iter<'path> {
        Iter::new(self.as_ref())
    }
}

impl<'path> WindowsMemoryPathBuf<'path> for PathBuf {}

// ===========================================================================
//
// ===========================================================================
