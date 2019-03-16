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
// Traits
// ===========================================================================

pub trait Path {
    fn as_bytes(&self) -> &[u8];
    fn as_os_str(&self) -> &OsStr;
}

pub trait PathBuf: Path {}

pub trait MemoryPath<'path> {
    type Iter: Iterator + 'path;

    fn iter(&'path self) -> Self::Iter;
}

pub trait MemoryPathBuf<'path>: MemoryPath<'path> {}

// ===========================================================================
// PlatformPath
// ===========================================================================

#[derive(Debug, PartialEq, Eq)]
pub struct PlatformPath {
    inner: OsStr,
}

impl PlatformPath {
    pub fn new<P: AsRef<OsStr> + ?Sized>(path: &P) -> &PlatformPath {
        unsafe { &*(path.as_ref() as *const OsStr as *const PlatformPath) }
    }

    pub fn from_bytes<T>(s: &T) -> &PlatformPath
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let s = as_osstr(s.as_ref());
        PlatformPath::new(s)
    }
}

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
impl PlatformPath {
    pub fn to_utf16(&self) -> Vec<u16> {
        self.as_os_str().encode_wide().collect()
    }
}

impl Path for PlatformPath {
    #[cfg(unix)]
    fn as_bytes(&self) -> &[u8] {
        (&self.inner).as_bytes()
    }

    #[cfg(windows)]
    #[cfg_attr(tarpaulin, skip)]
    fn as_bytes(&self) -> &[u8] {
        os_str_as_bytes(&self.inner)
    }

    fn as_os_str(&self) -> &OsStr {
        &self.inner
    }
}

impl From<&PlatformPath> for Vec<u8> {
    fn from(p: &PlatformPath) -> Vec<u8> {
        p.as_bytes().to_vec()
    }
}

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
impl From<&PlatformPath> for Vec<u16> {
    fn from(p: &PlatformPath) -> Vec<u16> {
        p.to_utf16()
    }
}

impl AsRef<[u8]> for PlatformPath {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

unsafe impl Send for PlatformPath {}

unsafe impl Sync for PlatformPath {}

impl AsRef<OsStr> for PlatformPath {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

path_asref_impl!(PlatformPath, PlatformPath);
path_asref_impl!(PlatformPath, OsStr);
path_asref_impl!(PlatformPath, StdPath);
path_asref_impl!(StdPath, PlatformPath);

// ===========================================================================
// PlatformPathBuf
// ===========================================================================

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct PlatformPathBuf {
    inner: OsString,
}

impl PlatformPathBuf {
    pub fn new() -> PlatformPathBuf {
        Default::default()
    }

    pub fn from_bytes<P>(p: &P) -> PlatformPathBuf
    where
        P: AsRef<[u8]> + ?Sized,
    {
        let inner = as_osstr(p.as_ref()).to_os_string();
        PlatformPathBuf { inner }
    }
}

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
impl PlatformPathBuf {
    pub fn from_utf16<P>(p: &P) -> PlatformPathBuf
    where
        P: AsRef<[u16]> + ?Sized,
    {
        let inner = OsString::from_wide(p.as_ref());
        PlatformPathBuf { inner }
    }

    pub fn to_utf16(&self) -> Vec<u16> {
        self.inner.as_os_str().encode_wide().collect()
    }
}

impl Path for PlatformPathBuf {
    #[cfg(unix)]
    fn as_bytes(&self) -> &[u8] {
        self.as_os_str().as_bytes()
    }

    #[cfg(windows)]
    #[cfg_attr(tarpaulin, skip)]
    fn as_bytes(&self) -> &[u8] {
        os_str_as_bytes(self.inner.as_os_str())
    }

    fn as_os_str(&self) -> &OsStr {
        self.inner.as_os_str()
    }
}

impl PathBuf for PlatformPathBuf {}

impl<P> From<&P> for PlatformPathBuf
where
    P: AsRef<OsStr> + ?Sized,
{
    fn from(p: &P) -> PlatformPathBuf {
        let inner = p.as_ref().to_os_string();
        PlatformPathBuf { inner }
    }
}

impl From<PlatformPathBuf> for Vec<u8> {
    fn from(p: PlatformPathBuf) -> Vec<u8> {
        p.as_bytes().to_vec()
    }
}

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
impl From<PlatformPathBuf> for Vec<u16> {
    fn from(p: PlatformPathBuf) -> Vec<u16> {
        p.to_utf16()
    }
}

impl AsRef<PlatformPath> for PlatformPathBuf {
    fn as_ref(&self) -> &PlatformPath {
        PlatformPath::new(self)
    }
}

impl AsRef<OsStr> for PlatformPathBuf {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

path_asref_impl!(StdPath, PlatformPathBuf);

// ===========================================================================
//
// ===========================================================================
