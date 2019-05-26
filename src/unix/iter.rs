// src/unix/iter.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

#[cfg(feature = "manual-iter")]
mod manual;

#[cfg(feature = "parser-iter")]
mod parser;

#[cfg(feature = "manual-iter")]
mod iter_imports {
    pub use super::manual::{Iter, PathComponent};
}

#[cfg(all(feature = "parser-iter", not(feature = "manual-iter")))]
mod iter_imports {
    pub use super::parser::{Iter, PathComponent};
}

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsStr;

// Third-party imports

// Local imports
use crate::common::string::as_str;
use crate::path::SystemStr;

// ===========================================================================
// Re-exports
// ===========================================================================

pub use self::iter_imports::*;

// ===========================================================================
// Component
// ===========================================================================

#[derive(Debug, Eq, PartialEq)]
pub enum Component<'path> {
    RootDir,
    CurDir,
    ParentDir,
    Normal(&'path OsStr),
}

impl<'path> Component<'path> {
    pub fn as_os_str(&self) -> &'path OsStr {
        match self {
            Component::RootDir => OsStr::new("/"),
            Component::CurDir => OsStr::new("."),
            Component::ParentDir => OsStr::new(".."),
            Component::Normal(comp) => comp,
        }
    }
}

impl<'path> From<&'path [u8]> for Component<'path> {
    fn from(s: &'path [u8]) -> Component<'path> {
        let s = as_str(s);
        match s {
            "/" => Component::RootDir,
            "." => Component::CurDir,
            ".." => Component::ParentDir,
            _ => Component::Normal(OsStr::new(s)),
        }
    }
}

// Implement AsRef<OsStr> and AsRef<SystemStr> for Component
impl<'path> AsRef<OsStr> for Component<'path> {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl<'path> AsRef<SystemStr> for Component<'path> {
    fn as_ref(&self) -> &SystemStr {
        SystemStr::new(self)
    }
}

// ===========================================================================
//
// ===========================================================================
