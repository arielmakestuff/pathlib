// src/unix/iter/parser.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports
use combine::Parser;

// Local imports
use crate::path::{PathIterator, SystemStr};
use crate::unix::{
    iter::Component,
    parser::{component, into_error, root},
    PathParseState,
};

// ===========================================================================
// Re-exports
// ===========================================================================

pub use crate::unix::parser::PathComponent;

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
    fn parse_root(&mut self) -> Option<PathComponent<'path>> {
        // This case will only happen if the input path is empty
        if self.path.is_empty() {
            self.parse_state = PathParseState::PathComponent;
            return Some(Ok(Component::CurDir));
        }

        self.parse_state = PathParseState::Root;

        let result = root().parse(self.path);
        match result {
            Err(_) => self.parse_component(),
            Ok(((comp, len), _)) => {
                self.cur = len;
                Some(comp)
            }
        }
    }

    fn parse_component(&mut self) -> Option<PathComponent<'path>> {
        let path_len = self.path.len();
        if self.cur == path_len {
            self.parse_state = PathParseState::Finish;
            return None;
        }

        match self.parse_state {
            PathParseState::Finish | PathParseState::PathComponent => {}
            _ => self.parse_state = PathParseState::PathComponent,
        }

        let remaining = &self.path[self.cur..];
        let result = component().easy_parse(remaining);
        match result {
            Err(err) => Some(Err(into_error(self.path, self.cur, err))),
            Ok(((comp, len), _)) => {
                self.cur += len;
                if self.cur < path_len {
                    // At this point, the very next byte is the separator, so it
                    // is safe to increment self.cur by 1 to make sure the next
                    // iteration does not include the separator
                    self.cur += 1;
                }
                Some(comp)
            }
        }
    }

    #[allow(dead_code)]
    #[cfg(test)]
    pub fn current_index(&self) -> usize {
        self.cur
    }
}

impl<'path> Iterator for Iter<'path> {
    // unix_iter_iterator_body!(PathComponent<'path>);
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
