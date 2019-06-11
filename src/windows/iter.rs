// src/windows/iter.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

#[cfg(feature = "manual-iter")]
pub mod manual;

#[cfg(feature = "parser-iter")]
pub mod parser;

#[cfg(feature = "manual-iter")]
mod iter_imports {
    pub use super::manual::{Iter, Prefix};
}

#[cfg(all(feature = "parser-iter", not(feature = "manual-iter")))]
mod iter_imports {
    pub use super::parser::{Iter, Prefix};
}

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsStr;

// Third-party imports

// Local imports
use crate::common::{error::ErrorInfo, string::as_osstr};
use crate::path::SystemStr;

// ===========================================================================
// Re-exports
// ===========================================================================

pub use self::iter_imports::*;
pub use std::path::Prefix;

// ===========================================================================
// Iter
// ===========================================================================

#[derive(Debug, Eq, PartialEq)]
pub enum Component<'path> {
    Prefix(PrefixComponent<'path>),
    RootDir(&'path OsStr),
    CurDir,
    ParentDir,
    Normal(&'path OsStr),
    Error(ErrorInfo<'path>),
}

impl<'path> Component<'path> {
    pub fn as_os_str(&self) -> &'path OsStr {
        match self {
            Component::Prefix(prefix_str) => prefix_str.as_os_str(),
            Component::RootDir(rootdir) => rootdir,
            Component::CurDir => OsStr::new("."),
            Component::ParentDir => OsStr::new(".."),
            Component::Normal(comp) => comp,
            Component::Error(_) => unimplemented!(),
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

#[derive(Debug, Eq, PartialEq)]
pub struct PrefixComponent<'path> {
    raw: &'path OsStr,
    parsed: Prefix<'path>,
}

impl<'path> PrefixComponent<'path> {
    pub fn new(path: &'path [u8], prefix: Prefix<'path>) -> Self {
        PrefixComponent {
            raw: as_osstr(path),
            parsed: prefix,
        }
    }

    pub fn kind(&self) -> Prefix<'path> {
        self.parsed
    }

    pub fn as_os_str(&self) -> &'path OsStr {
        self.raw
    }
}

// ===========================================================================
//
// ===========================================================================
