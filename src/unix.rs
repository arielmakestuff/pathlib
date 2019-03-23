// src/unix.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

mod iter;
mod path_type;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::{OsStr, OsString};
use std::ops::Deref;

// Third-party imports

// Local imports
use crate::common::string::as_osstr;
use crate::path::{
    MemoryPath, MemoryPathBuf, MemoryPathParts, MemoryPathPartsExt as _,
    SystemStr, SystemString,
};

// ===========================================================================
// Re-exports
// ===========================================================================

pub use self::iter::{Component, Iter, PathComponent};

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
// Helpers
// ===========================================================================

pub(crate) fn as_os_string(path: &[u8]) -> OsString {
    OsString::from(as_osstr(path))
}

// ===========================================================================
// SystemStr types
// ===========================================================================

// --------------------
// UnixPath
// --------------------

#[derive(Debug, PartialEq, Eq)]
pub struct UnixPath<'path> {
    path: &'path SystemStr,
}

impl<'path> UnixPath<'path> {
    pub fn new<P: AsRef<OsStr> + ?Sized>(path: &P) -> UnixPath {
        UnixPath {
            path: SystemStr::new(path),
        }
    }
}

impl<'path> Deref for UnixPath<'path> {
    type Target = SystemStr;

    fn deref(&self) -> &SystemStr {
        self.path
    }
}

impl<'path> MemoryPath<'path> for UnixPath<'path> {
    type Iter = Iter<'path>;

    fn iter(&self) -> Iter<'path> {
        Iter::new(self.path)
    }
}

impl<'path> Iterator for MemoryPathParts<'path, Iter<'path>> {
    type Item = OsString;

    fn next(&mut self) -> Option<OsString> {
        match self.path_iter().next() {
            Some(Ok(c)) => Some(c.as_os_str().to_os_string()),
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

impl<'path> MemoryPath<'path> for UnixPathBuf {
    type Iter = Iter<'path>;

    fn iter(&'path self) -> Iter<'path> {
        Iter::new(self.as_ref())
    }
}

impl<'path> MemoryPathBuf<'path> for UnixPathBuf {}

// ===========================================================================
//
// ===========================================================================
