// src/windows/path.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsStr;
use std::path::Path as StdPath;
// use std::os::windows::ffi::OsStrExt;

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
// * std::ffi::osstr
// * crate::path_asref_impl
// * std::path::path as stdpath
path_struct!();

impl Path {
    pub fn as_bytes<'path>(&'path self) -> Vec<u8> {
        unimplemented!()
        // let result: vec<u16> = self.as_ref().encode_wide().collect();
        // let s = string::from_utf16_lossy(&result[..]);
        // s.bytes().collect()
    }
}

// ===========================================================================
// Path AsRef implementations
// ===========================================================================

// impl AsRef<OsStr> for Path {
//     fn as_ref(&self) -> &OsStr {
//         self.as_os_str()
//     }
// }

// path_asref_impl!(Path, Path);
// path_asref_impl!(Path, OsStr);
// path_asref_impl!(Path, StdPath);
// path_asref_impl!(StdPath, Path);

// ===========================================================================
// PathBuf
// ===========================================================================

#[derive(Debug, PartialEq, Eq)]
pub struct PathBuf {
    inner: Vec<u8>,
}

impl PathBuf {
    pub fn new<P: AsRef<OsStr> + ?Sized>(_path: &P) -> PathBuf {
        unimplemented!()
        // let result: Vec<u16> = path.as_ref().encode_wide().collect();
        // let s = String::from_utf16_lossy(&result[..]);
        // PathBuf {
        //     inner: s.bytes().collect(),
        // }
    }

    pub fn as_bytes<'path>(&'path self) -> &'path [u8] {
        &self.inner[..]
    }

    pub fn as_os_str(&self) -> &OsStr {
        as_osstr(&self.inner[..])
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
