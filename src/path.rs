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

// Third-party imports

// Local imports

// ===========================================================================
// Path
// ===========================================================================

pub trait ComponentIterator: Iterator {}

#[derive(Debug, PartialEq, Eq)]
pub struct Path {
    inner: OsStr,
}

impl Path {
    pub fn new<P: AsRef<OsStr> + ?Sized>(path: &P) -> &Path {
        unsafe { &*(path.as_ref() as *const OsStr as *const Path) }
    }

    pub fn as_os_str(&self) -> &OsStr {
        &self.inner
    }

pub trait MemoryPath<'path, I>
where
    I: ComponentIterator,
{
    fn iter(&'path self) -> I;
}

// ===========================================================================
// AsRef implementations
// ===========================================================================

impl AsRef<OsStr> for Path {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}


macro_rules! path_asref_impl {
    ($dest:ident, $base:ident) => {
        impl AsRef<$dest> for $base {
            fn as_ref(&self) -> &$dest {
                $dest::new(self)
            }
        }
    };
}

path_asref_impl!(Path, Path);
path_asref_impl!(Path, OsStr);
path_asref_impl!(Path, StdPath);
path_asref_impl!(StdPath, Path);

#[macro_export]
macro_rules! impl_memorypath {
    ($type:ty, $iter:ty) => {
        impl<'path> MemoryPath<'path, $iter> for $type {
            fn iter(&'path self) -> $iter {
                Iter::new(self.as_bytes())
            }
        }
    };
}

// ===========================================================================
//
// ===========================================================================
