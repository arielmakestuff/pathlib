// src/windows/iter/parser.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports
use combine::{
    easy::{self, Errors},
    stream::{self, PointerOffset},
    Parser,
};

// Local imports
use crate::common::error;
use crate::path::{PathIterator, SystemStr};
use crate::windows::iter::Component;
use crate::windows::parser::{
    component, prefix, root, valid_part_char, RESTRICTED_NAME_ERRMSG,
};
use crate::windows::WindowsErrorKind;

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
    Prefix,
    Root,
    PathComponent,
    Finish,
}

// ===========================================================================
// Helpers
// ===========================================================================

fn make_error<'path, I, R>(
    path: &'path [u8],
    start: usize,
    parse_error: Errors<I, R, PointerOffset>,
) -> error::ErrorInfo {
    let path_comp = &path[start..];

    let err = parse_error.map_position(|p| p.translate_position(path_comp));
    let err_position = err.position;
    let pos = start + err_position;
    let mut msg = "";

    let errkind = {
        use easy::Error::*;
        use stream::easy::Info::*;

        let mut ret = WindowsErrorKind::InvalidCharacter;
        for e in err.errors {
            match e {
                Message(Borrowed(errmsg))
                    if errmsg == RESTRICTED_NAME_ERRMSG =>
                {
                    msg = errmsg;
                    ret = WindowsErrorKind::RestrictedName;
                    break;
                }
                Message(Borrowed(errmsg)) | Unexpected(Borrowed(errmsg)) => {
                    msg = errmsg;
                    break;
                }
                _ => {}
            }
        }

        ret
    };

    let kind = error::ParseErrorKind::Windows(errkind);

    // the returned tuple is (found, rest) where found is the part of the input
    // that matches and the rest is the remaining part of the input that's
    // unparsed
    let rest = valid_part_char()
        .parse(path_comp)
        .expect("should not fail")
        .0;
    let end = start + rest.len();

    error::ErrorInfo::new(kind, path, start, end, pos, msg)
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
        let p = path.as_ref();
        Iter {
            path: p,
            parse_state: PathParseState::Start,
            cur: 0,
        }
    }
}

impl<'path> Iter<'path> {
    fn parse_prefix(&mut self) -> Option<Component<'path>> {
        // This case will only happen if the input path is empty
        if self.path.is_empty() {
            self.parse_state = PathParseState::PathComponent;
            return Some(Component::CurDir);
        }

        let mut ret = None;
        if let Ok((found, _)) = prefix().easy_parse(self.path) {
            if let (Component::Prefix(_), end) = found {
                self.cur = end;
                ret = Some(found.0);
            }
        }

        self.parse_state = PathParseState::Prefix;

        match ret {
            Some(_) => ret,
            None => self.parse_root(),
        }
    }

    fn parse_root(&mut self) -> Option<Component<'path>> {
        self.parse_state = PathParseState::Root;
        let path = &self.path[self.cur..];

        if let Ok(((comp, len), _)) = root().easy_parse(path) {
            self.cur += len;
            Some(comp)
        } else {
            self.parse_component()
        }
    }

    fn parse_component(&mut self) -> Option<Component<'path>> {
        let end = self.path.len();
        let cur = self.cur;

        if cur == end {
            self.parse_state = PathParseState::Finish;
            return None;
        }

        let path = &self.path[self.cur..];
        let ret = match component().easy_parse(path) {
            Ok(((comp, len), _)) => {
                // Add an additional 1 to account for the separator
                let inc = if cur + len < end { len + 1 } else { len };
                self.cur += inc;
                Some(comp)
            }
            Err(err) => {
                self.parse_state = PathParseState::Finish;
                let err = make_error(self.path, self.cur, err);
                Some(Component::Error(err))
            }
        };

        match self.parse_state {
            PathParseState::Finish | PathParseState::PathComponent => {}
            _ => self.parse_state = PathParseState::PathComponent,
        }

        ret
    }

    #[allow(dead_code)]
    #[cfg(test)]
    pub fn current_index(&self) -> usize {
        self.cur
    }
}

impl<'path> Iterator for Iter<'path> {
    type Item = Component<'path>;

    fn next(&mut self) -> Option<Component<'path>> {
        match self.parse_state {
            PathParseState::Start => self.parse_prefix(),
            PathParseState::Prefix => self.parse_root(),
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
