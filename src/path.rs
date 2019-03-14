// src/path.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsStr;
use std::path::Path as StdPath;

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

// Third-party imports

// Local imports
use crate::common::string::as_osstr;

#[cfg(unix)]
pub use crate::unix::path::PathBuf;

#[cfg(windows)]
pub use crate::{common::string::os_str_as_bytes, windows::path::PathBuf};

// ===========================================================================
// Macros
// ===========================================================================

#[macro_export]
macro_rules! path_asref_impl {
    ($dest:ident, $base:ident) => {
        impl AsRef<$dest> for $base {
            fn as_ref(&self) -> &$dest {
                $dest::new(self)
            }
        }
    };
}

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

    #[cfg(unix)]
    pub fn as_bytes(&self) -> &[u8] {
        (&self.inner).as_bytes()
    }

    #[cfg(windows)]
    pub fn as_bytes(&self) -> &[u8] {
        os_str_as_bytes(&self.inner)
    }

    pub fn as_os_str(&self) -> &OsStr {
        &self.inner
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
