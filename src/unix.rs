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
use crate::path::{MemoryPath, MemoryPathBuf, PlatformPath, PlatformPathBuf};

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
// PlatformPath types
// ===========================================================================

// --------------------
// UnixPath
// --------------------

#[derive(Debug, PartialEq, Eq)]
pub struct UnixPath<'path> {
    path: &'path PlatformPath,
}

impl<'path> UnixPath<'path> {
    pub fn new<P: AsRef<OsStr> + ?Sized>(path: &P) -> UnixPath {
        UnixPath {
            path: PlatformPath::new(path),
        }
    }
}

impl<'path> Deref for UnixPath<'path> {
    type Target = PlatformPath;

    fn deref(&self) -> &PlatformPath {
        self.path
    }
}

impl<'path> MemoryPath<'path> for UnixPath<'path> {
    type Iter = Iter<'path>;

    fn iter(&self) -> Iter<'path> {
        Iter::new(self.path)
    }
}

// --------------------
// UnixPathBuf
// --------------------

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct UnixPathBuf {
    pathbuf: PlatformPathBuf,
}

impl UnixPathBuf {
    pub fn new() -> UnixPathBuf {
        Default::default()
    }
}

impl Deref for UnixPathBuf {
    type Target = PlatformPathBuf;

    fn deref(&self) -> &PlatformPathBuf {
        &self.pathbuf
    }
}

impl<P> From<&P> for UnixPathBuf
where
    P: AsRef<OsStr> + ?Sized,
{
    fn from(p: &P) -> UnixPathBuf {
        UnixPathBuf {
            pathbuf: PlatformPathBuf::from(p),
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
