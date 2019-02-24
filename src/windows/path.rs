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
use crate::common::string::as_osstr;
use crate::{path_asref_impl, path_struct};

// ===========================================================================
// Path
// ===========================================================================

// create path struct
//
// this macro invocation needs the following imports:
// * std::ffi::OsStr
// * crate::path_asref_impl
// * std::path::Path as StdPath
path_struct!();

impl Path {
    pub fn from_bytes<P>(p: P) -> PathBuf
    where
        P: AsRef<[u8]> + ?Sized,
    {
        Path::new(as_osstr(s.as_ref()))
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

// ===========================================================================
// PathBuf
// ===========================================================================

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PathBuf {
    inner: OsString,
}

impl PathBuf {
    pub fn new() -> PathBuf {
        let inner = OsString::new();
        PathBuf { inner }
    }

    pub fn from_bytes<P>(p: P) -> PathBuf
    where
        P: AsRef<[u8]> + ?Sized,
    {
        let inner = as_osstr(s.as_ref()).to_os_string();
        PathBuf { inner }
    }

    pub fn from_utf16<P>(p: P) -> PathBuf
    where
        P: AsRef<[u16]> + ?Sized,
    {
        let inner = OsString::from_wide(p.as_ref());
        PathBuf { inner }
    }

    pub fn to_utf16(&self) -> Vec<u16> {
        Path::from(self).to_utf16()
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
        let inner = path.as_ref().to_os_string();
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
