// src/unix.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

pub mod path;

mod path_type;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsStr;
use std::str;

// Third-party imports

// Local imports
use self::path_type::Separator;
use crate::common::error::ParseError;
use crate::common::string::{as_osstr, as_str, is_char};
use crate::common::{AsPath, PathData};
use crate::path::Path;
use crate::{component_asref_impl, pathiter_trait_impl};

// ===========================================================================
// Error types
// ===========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnixErrorKind {
    InvalidCharacter,
}

// ===========================================================================
// Iter
// ===========================================================================

pub type PathComponent<'path> = Result<Component<'path>, ParseError<'path>>;

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

// Implement AsRef<OsStr> and AsRef<Path> for Component
component_asref_impl!(Component, 'path);

#[derive(Debug, Eq, PartialEq)]
enum PathParseState {
    Start,
    Root,
    PathComponent,
    Finish,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Iter<'path> {
    #[cfg(unix)]
    path: &'path [u8],

    #[cfg(windows)]
    path: Vec<u8>,

    parse_state: PathParseState,
    cur: usize,
}

impl<'path> Iter<'path> {
    #[cfg(unix)]
    pub fn new(path: &[u8]) -> Iter {
        Iter {
            path: path,
            parse_state: PathParseState::Start,
            cur: 0,
        }
    }

    #[cfg(windows)]
    pub fn new(path: Vec<u8>) -> Iter {
        Iter {
            path: path,
            parse_state: PathParseState::Start,
            cur: 0,
        }
    }

    fn parse_root(&mut self) -> Option<PathComponent<'path>> {
        // This case will only happen if the input path is empty
        if self.cur == self.path.len() {
            self.parse_state = PathParseState::PathComponent;
            return Some(Ok(Component::CurDir));
        }

        self.parse_state = PathParseState::Root;

        let path_str = as_str(self.path.as_ref());

        // Check for root
        if Separator == self.path[self.cur] && is_char(path_str, self.cur) {
            self.cur += 1;
            let ret = Component::RootDir;
            return Some(Ok(ret));
        }

        self.parse_component()
    }

    fn parse_component(&mut self) -> Option<PathComponent<'path>> {
        let end = self.path.len();
        let cur = self.cur;

        if cur == end {
            self.parse_state = PathParseState::Finish;
            return None;
        }

        let mut ret = None;
        let path_str = as_str(self.path.as_ref());
        let mut has_invalid_char = false;
        for i in cur..end {
            if !is_char(path_str, i) {
                continue;
            } else if self.path[i] == b'\x00' {
                // The null character is not allowed in unix filenames
                has_invalid_char = true;
            } else if Separator == self.path[i] {
                let part = &self.path[cur..i];
                let comp = if part.len() == 0 {
                    Ok(Component::CurDir)
                } else {
                    self.to_comp(part, has_invalid_char)
                };
                ret = Some(comp);
                self.cur = i + 1;
                break;
            }
        }

        match self.parse_state {
            PathParseState::Finish | PathParseState::PathComponent => {}
            _ => self.parse_state = PathParseState::PathComponent,
        }

        match ret {
            Some(_) => ret,
            None => {
                let comp = self.to_comp(&self.path[cur..end], has_invalid_char);
                self.cur = end;
                Some(comp)
            }
        }
    }

    fn to_comp(
        &mut self,
        part: &'path [u8],
        found_err: bool,
    ) -> Result<Component<'path>, ParseError<'path>> {
        let comp_str = as_str(part);

        if found_err {
            self.invalid_char(comp_str)
        } else {
            let ret = match comp_str {
                "." => Component::CurDir,
                ".." => Component::ParentDir,
                _ => Component::Normal(OsStr::new(comp_str)),
            };
            Ok(ret)
        }
    }

    fn invalid_char(
        &mut self,
        part: &'path str,
    ) -> Result<Component<'path>, ParseError<'path>> {
        // Return None for every call to next() after this
        self.parse_state = PathParseState::Finish;

        let msg = String::from("path component contains an invalid character");
        let err = ParseError::new(
            UnixErrorKind::InvalidCharacter.into(),
            OsStr::new(part),
            as_osstr(self.path),
            self.cur,
            self.cur + part.len(),
            msg,
        );

        Err(err)
    }
}

impl<'path> Iterator for Iter<'path> {
    type Item = PathComponent<'path>;

    fn next(&mut self) -> Option<PathComponent<'path>> {
        match self.parse_state {
            PathParseState::Start => self.parse_root(),
            PathParseState::Root | PathParseState::PathComponent => {
                self.parse_component()
            }
            PathParseState::Finish => None,
        }
    }
}

// Implement PathData and AsPath traits for Iter
pathiter_trait_impl!(Iter, 'path);

// ===========================================================================
//
// ===========================================================================
