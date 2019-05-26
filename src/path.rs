// src/path.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::{OsStr, OsString};
use std::marker::PhantomData;
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

pub trait AsSystemStr {
    fn as_sys_str(&self) -> &SystemStr;
}

pub trait SystemSeq {
    fn as_bytes(&self) -> &[u8];
    fn as_os_str(&self) -> &OsStr;
}

pub trait SystemSeqBuf: SystemSeq {}

pub trait PathIterator<'path>: Iterator {
    fn new(path: &'path SystemStr) -> Self
    where
        Self: Sized;
}

pub trait Path<'path> {
    type Iter: Iterator + 'path;

    fn iter(&'path self) -> Self::Iter;

    // --------------------
    // Properties
    // --------------------
    fn parts(&'path self) -> PathParts<Self::Iter> {
        PathParts::new(self.iter())
    }
}

pub trait PathBuf<'path>: Path<'path> {}

// ===========================================================================
// PathParts
// ===========================================================================

pub trait PathPartsExt<I>
where
    I: Iterator,
{
    fn stored_item(&mut self) -> &mut Option<OsString>;
    fn path_iter(&mut self) -> &mut I;
}

pub struct PathParts<'path, I>
where
    I: Iterator + 'path,
{
    iter: I,
    cur: Option<OsString>,
    _phantom: PhantomData<&'path ()>,
}

impl<'path, I> PathParts<'path, I>
where
    I: Iterator + 'path,
{
    fn new(iter: I) -> Self {
        PathParts {
            iter,
            cur: None,
            _phantom: PhantomData,
        }
    }
}

impl<'path, I> PathPartsExt<I> for PathParts<'path, I>
where
    I: Iterator + 'path,
{
    fn stored_item(&mut self) -> &mut Option<OsString> {
        &mut self.cur
    }

    fn path_iter(&mut self) -> &mut I {
        &mut self.iter
    }
}

// ===========================================================================
// SystemStr
// ===========================================================================

#[derive(Debug, PartialEq, Eq)]
pub struct SystemStr {
    inner: OsStr,
}

impl SystemStr {
    pub fn new<P: AsRef<OsStr> + ?Sized>(path: &P) -> &SystemStr {
        unsafe { &*(path.as_ref() as *const OsStr as *const SystemStr) }
    }

    pub fn from_bytes<T>(s: &T) -> &SystemStr
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let s = as_osstr(s.as_ref());
        SystemStr::new(s)
    }
}

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
impl SystemStr {
    pub fn to_utf16(&self) -> Vec<u16> {
        self.as_os_str().encode_wide().collect()
    }
}

impl SystemSeq for SystemStr {
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

impl From<&SystemStr> for Vec<u8> {
    fn from(p: &SystemStr) -> Vec<u8> {
        p.as_bytes().to_vec()
    }
}

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
impl From<&SystemStr> for Vec<u16> {
    fn from(p: &SystemStr) -> Vec<u16> {
        p.to_utf16()
    }
}

impl AsRef<[u8]> for SystemStr {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

unsafe impl Send for SystemStr {}

unsafe impl Sync for SystemStr {}

impl AsRef<OsStr> for SystemStr {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

path_asref_impl!(SystemStr, SystemStr);
path_asref_impl!(SystemStr, OsStr);
path_asref_impl!(SystemStr, StdPath);
path_asref_impl!(StdPath, SystemStr);

// ===========================================================================
// SystemString
// ===========================================================================

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SystemString {
    inner: OsString,
}

impl SystemString {
    pub fn new() -> SystemString {
        Default::default()
    }

    pub fn from_bytes<P>(p: &P) -> SystemString
    where
        P: AsRef<[u8]> + ?Sized,
    {
        let inner = as_osstr(p.as_ref()).to_os_string();
        SystemString { inner }
    }
}

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
impl SystemString {
    pub fn from_utf16<P>(p: &P) -> SystemString
    where
        P: AsRef<[u16]> + ?Sized,
    {
        let inner = OsString::from_wide(p.as_ref());
        SystemString { inner }
    }

    pub fn to_utf16(&self) -> Vec<u16> {
        self.inner.as_os_str().encode_wide().collect()
    }
}

impl SystemSeq for SystemString {
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

impl SystemSeqBuf for SystemString {}

impl<P> From<&P> for SystemString
where
    P: AsRef<OsStr> + ?Sized,
{
    fn from(p: &P) -> SystemString {
        let inner = p.as_ref().to_os_string();
        SystemString { inner }
    }
}

impl From<SystemString> for Vec<u8> {
    fn from(p: SystemString) -> Vec<u8> {
        p.as_bytes().to_vec()
    }
}

#[cfg(windows)]
#[cfg_attr(tarpaulin, skip)]
impl From<SystemString> for Vec<u16> {
    fn from(p: SystemString) -> Vec<u16> {
        p.to_utf16()
    }
}

impl AsRef<SystemStr> for SystemString {
    fn as_ref(&self) -> &SystemStr {
        SystemStr::new(self)
    }
}

impl AsRef<OsStr> for SystemString {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

path_asref_impl!(StdPath, SystemString);

// ===========================================================================
//
// ===========================================================================
