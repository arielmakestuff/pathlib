// src/windows/iter/manual.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::{OsStr, OsString};

// Third-party imports

// Local imports
use super::{Component, PrefixComponent};
use crate::common::error::ParseError;
use crate::common::string::{as_osstr, as_str};
use crate::path::{PathIterator, SystemStr};
use crate::windows::{
    match_prefix::match_prefix,
    path_type::{Device, NonDevicePart},
    WindowsErrorKind, SEPARATOR,
};

// ===========================================================================
// Re-exports
// ===========================================================================

pub use std::path::Prefix;

// ===========================================================================
// Types
// ===========================================================================

#[derive(Debug, Eq, PartialEq)]
enum PathParseState {
    Start,
    Prefix { verbatimdisk: bool },
    Root,
    PathComponent,
    Finish,
}

pub type PathComponent<'path> = Result<Component<'path>, ParseError>;

// ===========================================================================
// Iter
// ===========================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Iter<'path> {
    path: &'path [u8],
    parse_state: PathParseState,
    cur: usize,
}

impl<'path> PathIterator<'path> for Iter<'path> {
    fn new(path: &'path SystemStr) -> Iter {
        Iter {
            path: path.as_ref(),
            parse_state: PathParseState::Start,
            cur: 0,
        }
    }
}

impl<'path> Iter<'path> {
    fn parse_prefix(&mut self) -> Option<PathComponent<'path>> {
        let mut verbatimdisk = false;
        let mut ret = None;
        if let Some((end, prefix)) = match_prefix(self.path) {
            if let Prefix::VerbatimDisk(_) = prefix {
                verbatimdisk = true;
            }
            let prefix_comp = PrefixComponent::new(&self.path[..end], prefix);
            self.cur = end;

            ret = Some(Ok(Component::Prefix(prefix_comp)));
        }

        self.parse_state = PathParseState::Prefix { verbatimdisk };

        if ret.is_some() {
            return ret;
        }

        self.parse_root(verbatimdisk)
    }

    fn parse_root(
        &mut self,
        verbatimdisk: bool,
    ) -> Option<PathComponent<'path>> {
        let path_len = self.path.len();
        let cur = self.cur;
        if path_len == 0 {
            self.parse_state = PathParseState::PathComponent;
            return Some(Ok(Component::CurDir));
        } else if cur == path_len {
            self.parse_state = PathParseState::Finish;
            return None;
        }

        self.parse_state = PathParseState::Root;

        let is_root = SEPARATOR.contains(&self.path[self.cur]);
        if is_root {
            self.cur += 1;
        }

        if verbatimdisk || is_root {
            let end = self.cur;
            let start = end - 1;
            let ret = Component::RootDir(as_osstr(&self.path[start..end]));
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
        for i in cur..end {
            let cur_char = &self.path[i];
            if SEPARATOR.contains(cur_char) {
                let part = &self.path[cur..i];
                let comp = if part.is_empty() {
                    Ok(Component::CurDir)
                } else {
                    self.build_comp(cur, i)
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
                let comp = self.build_comp(cur, end);
                self.cur = end;
                Some(comp)
            }
        }
    }

    fn build_comp(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<Component<'path>, ParseError> {
        let part = &self.path[start..end];
        if part != NonDevicePart {
            if part == Device {
                self.invalid_name(start, end)
            } else {
                self.invalid_char(start, end)
            }
        } else {
            let comp_str = as_str(part);
            let ret = match comp_str {
                "." => Component::CurDir,
                ".." => Component::ParentDir,
                _ => Component::Normal(OsStr::new(comp_str)),
            };
            Ok(ret)
        }
    }

    fn invalid_name(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<Component<'path>, ParseError> {
        // Return None for every call to next() after this
        self.parse_state = PathParseState::Finish;

        let msg = String::from("component uses a restricted name");
        self.build_error(WindowsErrorKind::RestrictedName, start, end, msg)
    }

    fn invalid_char(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<Component<'path>, ParseError> {
        // Return None for every call to next() after this
        self.parse_state = PathParseState::Finish;
        let msg = String::from("path component contains an invalid character");
        self.build_error(WindowsErrorKind::InvalidCharacter, start, end, msg)
    }

    fn build_error(
        &self,
        kind: WindowsErrorKind,
        start: usize,
        end: usize,
        msg: String,
    ) -> Result<Component<'path>, ParseError> {
        let part = as_str(&self.path[start..end]);
        let err = ParseError::new(
            kind.into(),
            OsString::from(part),
            OsString::from(as_str(self.path)),
            self.cur,
            self.cur + part.len(),
            msg,
        );

        Err(err)
    }

    #[cfg(test)]
    pub fn current_index(&self) -> usize {
        self.cur
    }
}

impl<'path> Iterator for Iter<'path> {
    type Item = PathComponent<'path>;

    fn next(&mut self) -> Option<PathComponent<'path>> {
        match self.parse_state {
            PathParseState::Start => self.parse_prefix(),
            PathParseState::Prefix { verbatimdisk } => {
                self.parse_root(verbatimdisk)
            }
            PathParseState::Root | PathParseState::PathComponent => {
                self.parse_component()
            }
            PathParseState::Finish => None,
        }
    }
}

impl<'path> AsRef<SystemStr> for Iter<'path> {
    fn as_ref(&self) -> &SystemStr {
        SystemStr::from_bytes(&self.path[self.cur..])
    }
}

// ===========================================================================
//
// ===========================================================================
