// src/unix/iter/manual.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Externs
// ===========================================================================

// Stdlib externs

// Third-party externs

// Local externs

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsString;

// Third-party imports

// Local imports
use super::Component;
use crate::common::error::ParseError;
use crate::common::string::as_osstr;
use crate::path::{PathIterator, SystemStr};
use crate::unix::{
    path_type::{Null, Separator},
    PathParseState, UnixErrorKind,
};

// ===========================================================================
// Types
// ===========================================================================

pub type PathComponent<'path> = Result<Component<'path>, ParseError>;

// ===========================================================================
// Helpers
// ===========================================================================

fn as_os_string(path: &[u8]) -> OsString {
    OsString::from(as_osstr(path))
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

impl<'path> PathIterator<'path> for Iter<'path> {
    fn new(path: &SystemStr) -> Iter {
        Iter {
            path: path.as_ref(),
            parse_state: PathParseState::Start,
            cur: 0,
        }
    }
}

impl<'path> Iter<'path> {
    // unix_iter_body!(PathComponent<'path>, Component<'path>);
    fn parse_root(&mut self) -> Option<PathComponent<'path>> {
        // This case will only happen if the input path is empty
        if self.cur == self.path.len() {
            self.parse_state = PathParseState::PathComponent;
            return Some(Ok(Component::CurDir));
        }

        self.parse_state = PathParseState::Root;

        // Check for root
        if Separator == self.path[self.cur] {
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
        let mut has_invalid_char = false;
        for i in cur..end {
            let cur_char = self.path[i];
            if Null == cur_char {
                // The null character is not allowed in unix filenames
                has_invalid_char = true;
            } else if Separator == cur_char {
                let part_len = i - cur;
                let comp = if part_len == 0 {
                    Ok(Component::CurDir)
                } else {
                    self.build_comp(cur, i, has_invalid_char)
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
                let comp = self.build_comp(cur, end, has_invalid_char);
                self.cur = end;
                Some(comp)
            }
        }
    }

    fn build_comp(
        &mut self,
        start: usize,
        end: usize,
        found_err: bool,
    ) -> Result<Component<'path>, ParseError> {
        if found_err {
            self.invalid_char(start, end)
        } else {
            Ok(Component::from(&self.path[start..end]))
        }
    }

    fn invalid_char(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<Component<'path>, ParseError> {
        // Return None for every call to next() after this
        self.parse_state = PathParseState::Finish;

        let msg = String::from("path component contains an invalid character");
        let err = ParseError::new(
            UnixErrorKind::InvalidCharacter.into(),
            as_os_string(&self.path[start..end]),
            as_os_string(&self.path[..]),
            start,
            end,
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
            PathParseState::Start => self.parse_root(),
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
