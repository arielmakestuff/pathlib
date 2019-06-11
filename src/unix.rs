// src/unix.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

pub mod iter;
mod path_type;

#[cfg(feature = "parser-iter")]
mod parser;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::{OsStr, OsString};
use std::ops::Deref;

// Third-party imports

// Local imports
use crate::path::{
    AsSystemStr, Path, PathBuf, PathParts, PathPartsExt as _, SystemStr,
    SystemString,
};

// ===========================================================================
// Re-exports
// ===========================================================================

pub use self::iter::{Component, Iter};

// ===========================================================================
// Types needed for Iter
// ===========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnixErrorKind {
    InvalidCharacter,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum PathParseState {
    Start,
    Root,
    PathComponent,
    Finish,
}

// ===========================================================================
// SystemStr types
// ===========================================================================

// --------------------
// UnixPath
// --------------------

#[derive(Debug, PartialEq, Eq)]
pub struct UnixPath {
    path: SystemStr,
}

impl UnixPath {
    pub fn new<P: AsRef<OsStr> + ?Sized>(path: &P) -> &UnixPath {
        // This is safe for 2 reasons:
        //
        // 1. SystemStr is essentially just an OsStr and UnixPath is essentially
        //    just a SystemStr so the type casting is valid wrt memory layout
        // 2. this is strictly returning an immutable reference
        unsafe { &*(path.as_ref() as *const OsStr as *const UnixPath) }
    }
}

impl Deref for UnixPath {
    type Target = SystemStr;

    fn deref(&self) -> &SystemStr {
        &self.path
    }
}

impl AsSystemStr for &UnixPath {
    fn as_sys_str(&self) -> &SystemStr {
        &*self
    }
}

impl<'path> Path<'path, Iter<'path>> for &'path UnixPath {}

impl<'path> Iterator for PathParts<'path, Iter<'path>> {
    type Item = OsString;

    fn next(&mut self) -> Option<OsString> {
        match self.path_iter().next() {
            Some(c) => Some(c.as_os_str().to_os_string()),
            _ => None,
        }
    }
}

// --------------------
// UnixPathBuf
// --------------------

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct UnixPathBuf {
    pathbuf: SystemString,
}

impl UnixPathBuf {
    pub fn new() -> UnixPathBuf {
        Default::default()
    }
}

impl Deref for UnixPathBuf {
    type Target = SystemString;

    fn deref(&self) -> &SystemString {
        &self.pathbuf
    }
}

impl AsSystemStr for UnixPathBuf {
    fn as_sys_str(&self) -> &SystemStr {
        self.pathbuf.as_ref()
    }
}

impl<P> From<&P> for UnixPathBuf
where
    P: AsRef<OsStr> + ?Sized,
{
    fn from(p: &P) -> UnixPathBuf {
        UnixPathBuf {
            pathbuf: SystemString::from(p),
        }
    }
}

impl<'path> Path<'path, Iter<'path>> for UnixPathBuf {}

impl<'path> PathBuf<'path, Iter<'path>> for UnixPathBuf {}

// ===========================================================================
//
// ===========================================================================
