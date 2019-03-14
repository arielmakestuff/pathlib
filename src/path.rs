// src/path.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::{OsStr, OsString};
use std::path::Path as StdPath;

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

#[cfg(windows)]
use std::os::windows::ffi::{OsStrExt, OsStringExt};

// Third-party imports

// Local imports
use crate::common::string::as_osstr;

#[cfg(windows)]
use crate::common::string::os_str_as_bytes;

// ===========================================================================
// Macros
// ===========================================================================

macro_rules! path_asref_impl {
    ($dest:ident, $base:ident) => {
        impl AsRef<$dest> for $base {
            fn as_ref(&self) -> &$dest {
                $dest::new(self)
            }
        }
    };
}

// ===========================================================================
// Path
// ===========================================================================

#[derive(Debug, PartialEq, Eq)]
pub struct Path {
    inner: OsStr,
}

impl Path {
    pub fn new<P: AsRef<OsStr> + ?Sized>(path: &P) -> &Path {
        unsafe { &*(path.as_ref() as *const OsStr as *const Path) }
    }

    pub fn from_bytes<T>(s: &T) -> &Path
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let s = as_osstr(s.as_ref());
        Path::new(s)
    }

    pub fn as_os_str(&self) -> &OsStr {
        &self.inner
    }
}

#[cfg(unix)]
impl Path {
    pub fn as_bytes(&self) -> &[u8] {
        (&self.inner).as_bytes()
    }
}

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
impl Path {
    pub fn as_bytes(&self) -> &[u8] {
        os_str_as_bytes(&self.inner)
    }

    pub fn to_utf16(&self) -> Vec<u16> {
        self.as_os_str().encode_wide().collect()
    }
}

impl From<&Path> for Vec<u8> {
    fn from(p: &Path) -> Vec<u8> {
        p.as_bytes().to_vec()
    }
}

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
impl From<&Path> for Vec<u16> {
    fn from(p: &Path) -> Vec<u16> {
        p.to_utf16()
    }
}

impl AsRef<[u8]> for Path {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

unsafe impl Send for Path {}

unsafe impl Sync for Path {}

impl AsRef<OsStr> for Path {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

path_asref_impl!(Path, Path);
path_asref_impl!(Path, OsStr);
path_asref_impl!(Path, StdPath);
path_asref_impl!(StdPath, Path);

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

    pub fn as_os_str(&self) -> &OsStr {
        self.inner.as_os_str()
    }
}

#[cfg(unix)]
impl PathBuf {
    pub fn as_bytes(&self) -> &[u8] {
        self.as_os_str().as_bytes()
    }
}

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
impl PathBuf {
    pub fn as_bytes(&self) -> &[u8] {
        os_str_as_bytes(self.inner.as_os_str())
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

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
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

impl AsRef<OsStr> for PathBuf {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

path_asref_impl!(StdPath, PathBuf);

// ===========================================================================
// Traits
// ===========================================================================

pub trait MemoryPath<'path> {
    type Iter: Iterator + 'path;

    fn iter(&'path self) -> Self::Iter;
}

pub trait MemoryPathBuf<'path>: MemoryPath<'path> {}

// ===========================================================================
//
// ===========================================================================
