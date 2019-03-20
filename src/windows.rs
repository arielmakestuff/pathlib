// src/windows.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

mod iter;
mod match_prefix;
mod path_type;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
// use std::cmp::PartialEq;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::ops::Deref;

// Third-party imports
use lazy_static::lazy_static;

// Local imports
use crate::path::{MemoryPath, MemoryPathBuf, SystemStr, SystemString};

// ===========================================================================
// Re-exports
// ===========================================================================

pub use self::iter::{Component, Iter, PathComponent, Prefix, PrefixComponent};

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
// SystemStr types
// ===========================================================================

// --------------------
// WindowsPath
// --------------------

#[derive(Debug, PartialEq, Eq)]
pub struct WindowsPath<'path> {
    path: &'path SystemStr,
}

impl<'path> WindowsPath<'path> {
    pub fn new<P: AsRef<OsStr> + ?Sized>(path: &P) -> WindowsPath {
        WindowsPath {
            path: SystemStr::new(path),
        }
    }
}

impl<'path> Deref for WindowsPath<'path> {
    type Target = SystemStr;

    fn deref(&self) -> &SystemStr {
        self.path
    }
}

impl<'path> MemoryPath<'path> for WindowsPath<'path> {
    type Iter = Iter<'path>;

    fn iter(&self) -> Iter<'path> {
        Iter::new(self.path)
    }
}

// --------------------
// WindowsPathBuf
// --------------------

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct WindowsPathBuf {
    pathbuf: SystemString,
}

impl WindowsPathBuf {
    pub fn new() -> WindowsPathBuf {
        Default::default()
    }
}

impl Deref for WindowsPathBuf {
    type Target = SystemString;

    fn deref(&self) -> &SystemString {
        &self.pathbuf
    }
}

impl<P> From<&P> for WindowsPathBuf
where
    P: AsRef<OsStr> + ?Sized,
{
    fn from(p: &P) -> WindowsPathBuf {
        WindowsPathBuf {
            pathbuf: SystemString::from(p),
        }
    }
}

impl<'path> MemoryPath<'path> for WindowsPathBuf {
    type Iter = Iter<'path>;

    fn iter(&'path self) -> Iter<'path> {
        Iter::new(self.as_ref())
    }
}

impl<'path> MemoryPathBuf<'path> for WindowsPathBuf {}

// ===========================================================================
//
// ===========================================================================
