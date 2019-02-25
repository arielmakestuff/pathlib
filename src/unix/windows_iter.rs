// src/unix/windows_iter.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::OsStringExt;

// Third-party imports

// Local imports
use super::path_type::{Null, Separator};
use crate::common::error::ParseError;
use crate::path::{Path, PathBuf};

use super::{as_os_string, PathParseState, UnixErrorKind};
use crate::{unix_iter_body, unix_iter_iterator_body};

// ===========================================================================
// Component
// ===========================================================================

pub type PathComponent = Result<Component, ParseError>;

#[derive(Debug, Eq, PartialEq)]
pub enum Component {
    RootDir,
    CurDir,
    ParentDir,
    Normal(OsString),
}

impl Component {
    pub fn as_os_str(&self) -> &OsStr {
        match self {
            Component::RootDir => OsStr::new("/"),
            Component::CurDir => OsStr::new("."),
            Component::ParentDir => OsStr::new(".."),
            Component::Normal(comp) => comp.as_os_str(),
        }
    }
}

impl From<&[u16]> for Component {
    fn from(s: &[u16]) -> Component {
        let s_len = s.len();
        if s_len == 2 && s[0] == u16::from(b'.') && s[0] == s[1] {
            // Check if s is ".."
            Component::ParentDir
        } else if s_len != 1 {
            Component::Normal(OsString::from_wide(s))
        } else if s[0] == u16::from(b'/') {
            Component::RootDir
        } else if s[0] == u16::from(b'.') {
            Component::CurDir
        } else {
            Component::Normal(OsString::from_wide(s))
        }
    }
}

// Implement AsRef<OsStr> and AsRef<Path> for Component
impl AsRef<OsStr> for Component {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl AsRef<Path> for Component {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

// ===========================================================================
// Iter
// ===========================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Iter<'path> {
    source: &'path Path,
    path: Vec<u16>,
    parse_state: PathParseState,
    cur: usize,
}

impl<'path> Iter<'path> {
    pub fn new(path: &'path Path) -> Iter {
        let source = path;
        let path = path.to_utf16();
        Iter {
            source,
            path,
            parse_state: PathParseState::Start,
            cur: 0,
        }
    }

    unix_iter_body!(PathComponent, Component);

    #[cfg(test)]
    pub fn current_index(&self) -> usize {
        self.cur
    }
}

impl<'path> Iterator for Iter<'path> {
    unix_iter_iterator_body!(PathComponent);
}

impl<'path> From<&Iter<'path>> for PathBuf {
    fn from(i: &Iter<'path>) -> PathBuf {
        PathBuf::from_utf16(&i.path[i.cur..])
    }
}

// ===========================================================================
//
// ===========================================================================
