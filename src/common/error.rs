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
use crate::common::string::as_osstr;
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

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorInfo<'path> {
    kind: ParseErrorKind,
    path: &'path [u8],
    start: usize,
    end: usize,
    pos: usize,
    msg: &'static str,
}

impl<'path> ErrorInfo<'path> {
    pub fn new(
        kind: ParseErrorKind,
        path: &'path [u8],
        start: usize,
        end: usize,
        pos: usize,
        msg: &'static str,
    ) -> ErrorInfo<'path> {
        ErrorInfo {
            kind,
            path,
            start,
            end,
            pos,
            msg,
        }
    }

    fn build_error(&self, msg: String) -> ParseError {
        let as_os_string = |path: &[u8]| OsString::from(as_osstr(path));

        let start = self.start;
        let end = self.end;
        ParseError::new(
            self.kind,
            as_os_string(&self.path[start..end]),
            as_os_string(&self.path[..]),
            start,
            end,
            msg,
        )
    }

    pub fn to_error(&self) -> ParseError {
        self.build_error(self.msg.to_owned())
    }

    pub fn with_errmsg<F>(&self, f: F) -> ParseError
    where
        F: Fn(&Self) -> String,
    {
        self.build_error(f(self))
    }
}

impl<'path> From<ErrorInfo<'path>> for ParseError {
    fn from(info: ErrorInfo<'path>) -> ParseError {
        info.to_error()
    }
}

// ===========================================================================
//
// ===========================================================================
