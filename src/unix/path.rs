// src/unix/path.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::path::Path as StdPath;

// Third-party imports

// Local imports
use crate::{path_asref_impl, path_struct};

// ===========================================================================
// Path
// ===========================================================================

// Create path struct
//
// This macro invocation needs the following imports:
// * std::ffi::osstr
// * crate::path_asref_impl
// * std::path::Path as StdPath
path_struct!();

impl Path {
    pub fn as_bytes<'path>(&'path self) -> &[u8] {
        self.inner.as_bytes()
    }
}

// ===========================================================================
// PathBuf
// ===========================================================================

#[derive(Debug, PartialEq, Eq)]
pub struct PathBuf {
    inner: OsString,
}

impl PathBuf {
    pub fn new<P: AsRef<OsStr> + ?Sized>(path: &P) -> PathBuf {
        PathBuf {
            inner: OsString::from(path),
        }
    }

    pub fn as_bytes<'path>(&'path self) -> &[u8] {
        self.as_os_str().as_bytes()
    }

    pub fn as_os_str(&self) -> &OsStr {
        self.inner.as_ref()
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
