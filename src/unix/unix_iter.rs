// src/unix.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsStr;

// Third-party imports

// Local imports
use super::path::Path;
use super::path_type::{Null, Separator};
use crate::common::error::ParseError;
use crate::common::string::as_str;

use super::{as_os_string, PathParseState, UnixErrorKind};
use crate::{unix_iter_body, unix_iter_iterator_body};

// ===========================================================================
// Component
// ===========================================================================

pub type PathComponent<'path> = Result<Component<'path>, ParseError>;

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

// Implement AsRef<OsStr> and AsRef<Path> for Component
impl<'path> AsRef<OsStr> for Component<'path> {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl<'path> AsRef<Path> for Component<'path> {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

// ===========================================================================
// Iter
// ===========================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Iter<'path> {
    path: &'path [u8],
    parse_state: PathParseState,
    cur: usize,
}

impl<'path> Iter<'path> {
    pub fn new(path: &Path) -> Iter {
        Iter {
            path: path.as_ref(),
            parse_state: PathParseState::Start,
            cur: 0,
        }
    }

    unix_iter_body!(PathComponent<'path>, Component<'path>);

    #[cfg(test)]
    pub fn current_index(&self) -> usize {
        self.cur
    }
}

impl<'path> Iterator for Iter<'path> {
    unix_iter_iterator_body!(PathComponent<'path>);
}

impl<'path> AsRef<Path> for Iter<'path> {
    fn as_ref(&self) -> &Path {
        Path::from_bytes(&self.path[self.cur..])
    }
}

// ===========================================================================
//
// ===========================================================================
