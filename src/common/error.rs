// src/common/error.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
pub use std::error::Error;
use std::ffi::OsString;

// Third-party imports

// Local imports
pub use crate::unix::UnixErrorKind;
pub use crate::windows::WindowsErrorKind;

// ===========================================================================
// Error types
// ===========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    Unix(UnixErrorKind),
    Windows(WindowsErrorKind),
}

impl From<UnixErrorKind> for ParseErrorKind {
    fn from(error: UnixErrorKind) -> Self {
        ParseErrorKind::Unix(error)
    }
}

impl From<WindowsErrorKind> for ParseErrorKind {
    fn from(error: WindowsErrorKind) -> Self {
        ParseErrorKind::Windows(error)
    }
}

#[derive(Debug, Display, PartialEq, Eq)]
#[display(
    fmt = "{:?}: unable to parse component {:?} range {}..{}: {}",
    path,
    component,
    start,
    end,
    msg
)]
pub struct ParseError {
    _kind: ParseErrorKind,
    component: OsString,
    path: OsString,
    start: usize,
    end: usize,
    msg: String,
}

impl ParseError {
    pub(crate) fn new(
        kind: ParseErrorKind,
        component: OsString,
        path: OsString,
        start: usize,
        end: usize,
        msg: String,
    ) -> ParseError {
        ParseError {
            _kind: kind,
            component,
            path,
            start,
            end,
            msg,
        }
    }

    pub fn kind(&self) -> ParseErrorKind {
        self._kind
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// ===========================================================================
//
// ===========================================================================
