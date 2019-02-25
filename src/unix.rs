// src/unix.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

#[cfg(unix)]
pub mod path;

#[cfg(unix)]
mod unix_iter;

#[cfg(windows)]
mod windows_iter;

mod path_type;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsString;

// Third-party imports

// Local imports
use crate::path::{Path, PathBuf};

// Platform imports

#[cfg(unix)]
use crate::common::string::as_osstr;

#[cfg(windows)]
use std::os::windows::ffi::OsStringExt;

// ===========================================================================
// Re-exports
// ===========================================================================

#[cfg(unix)]
pub use self::unix_iter::{Component, Iter, PathComponent};

#[cfg(windows)]
pub use self::windows_iter::{Component, Iter, PathComponent};

// ===========================================================================
// Types needed for Iter
// ===========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnixErrorKind {
    InvalidCharacter,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum PathParseState {
    Start,
    Root,
    PathComponent,
    Finish,
}

// ===========================================================================
// Helpers
// ===========================================================================

#[cfg(unix)]
pub(crate) fn as_os_string(path: &[u8]) -> OsString {
    OsString::from(as_osstr(path))
}

#[cfg(windows)]
pub(crate) fn as_os_string(path: &[u16]) -> OsString {
    OsString::from_wide(path)
}

// ===========================================================================
// Macros
// ===========================================================================

#[macro_export]
macro_rules! unix_iter_body {
    ($pathcomp:ty, $comp:ty) => {


    fn parse_root(&mut self) -> Option<$pathcomp> {
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

    fn parse_component(&mut self) -> Option<$pathcomp> {
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
    ) -> Result<$comp, ParseError> {
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
    ) -> Result<$comp, ParseError> {
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


    };
}

#[macro_export]
macro_rules! unix_iter_iterator_body {
    ($pathcomp:ty) => {


    type Item = $pathcomp;

    fn next(&mut self) -> Option<$pathcomp> {
        match self.parse_state {
            PathParseState::Start => self.parse_root(),
            PathParseState::Root | PathParseState::PathComponent => {
                self.parse_component()
            }
            PathParseState::Finish => None,
        }
    }


    };
}

// ===========================================================================
// Traits
// ===========================================================================

pub trait UnixMemoryPath<'path> {
    fn iter(&'path self) -> Iter<'path>;
}

pub trait UnixMemoryPathBuf<'path>: UnixMemoryPath<'path> {}

impl<'path> UnixMemoryPath<'path> for Path {
    fn iter(&'path self) -> Iter<'path> {
        Iter::new(self)
    }
}

impl<'path> UnixMemoryPath<'path> for PathBuf {
    fn iter(&'path self) -> Iter<'path> {
        Iter::new(self.as_ref())
    }
}

impl<'path> UnixMemoryPathBuf<'path> for PathBuf {}

// ===========================================================================
//
// ===========================================================================
