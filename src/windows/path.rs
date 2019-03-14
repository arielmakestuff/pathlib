// src/windows/path.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path as StdPath;

// Third-party imports

// Local imports
use crate::common::string::{as_osstr, os_str_to_bytes};
use crate::path::Path;
use crate::path_asref_impl;

// ===========================================================================
// Path
// ===========================================================================

impl Path {
    pub fn as_bytes(&self) -> &[u8] {
        os_str_to_bytes(self.as_os_str())
    }

    pub fn to_utf16(&self) -> Vec<u16> {
        self.as_os_str().encode_wide().collect()
    }
}

impl From<&Path> for Vec<u16> {
    fn from(p: &Path) -> Vec<u16> {
        p.to_utf16()
    }
}

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
        let inner = as_osstr(p.as_ref()).to_os_string();
        PathBuf { inner }
    }

    pub fn from_utf16<P>(p: &P) -> PathBuf
    where
        P: AsRef<[u16]> + ?Sized,
    {
        let inner = OsString::from_wide(p.as_ref());
        PathBuf { inner }
    }

    pub fn to_utf16(&self) -> Vec<u16> {
        self.inner.as_os_str().encode_wide().collect()
    }

    pub fn as_os_str(&self) -> &OsStr {
        self.inner.as_os_str()
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

impl From<PathBuf> for Vec<u16> {
    fn from(p: PathBuf) -> Vec<u16> {
        p.to_utf16()
    }
}

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

// ===========================================================================
// AsRef implementations
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
