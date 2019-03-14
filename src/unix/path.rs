// src/unix/path.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::Path as StdPath;

// Third-party imports

// Local imports
use crate::path::Path;
use crate::path_asref_impl;

// ===========================================================================
// Path
// ===========================================================================

impl From<&Path> for Vec<u8> {
    fn from(p: &Path) -> Vec<u8> {
        p.as_bytes().to_vec()
    }
}

impl AsRef<[u8]> for Path {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

// ===========================================================================
// PathBuf
// ===========================================================================

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct PathBuf {
    inner: OsString,
}

impl PathBuf {
    pub fn new() -> PathBuf {
        Default::default()
    }

    pub fn from_bytes<P>(p: &P) -> PathBuf
    where
        P: AsRef<[u8]> + ?Sized,
    {
        let bytes = p.as_ref().to_vec();
        let inner = OsString::from_vec(bytes);
        PathBuf { inner }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.as_os_str().as_bytes()
    }

    pub fn as_os_str(&self) -> &OsStr {
        self.inner.as_ref()
    }
}

impl<P> From<&P> for PathBuf
where
    P: AsRef<OsStr> + ?Sized,
{
    fn from(p: &P) -> PathBuf {
        let inner = p.as_ref().to_os_string();
        PathBuf { inner }
    }
}

impl From<PathBuf> for Vec<u8> {
    fn from(p: PathBuf) -> Vec<u8> {
        p.as_bytes().to_vec()
    }
}

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

// ===========================================================================
// PathBuf AsRef implementations
// ===========================================================================

impl AsRef<OsStr> for PathBuf {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

path_asref_impl!(StdPath, PathBuf);

// ===========================================================================
//
// ===========================================================================
