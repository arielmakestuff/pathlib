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
#[display(fmt = "unable to parse path at position {}: {}", errpos, msg)]
pub struct ParseError {
    kind: ParseErrorKind,
    path: OsString,
    errpos: usize,
    msg: String,
}

impl ParseError {
    pub(crate) fn new(
        kind: ParseErrorKind,
        path: OsString,
        errpos: usize,
        msg: String,
    ) -> ParseError {
        ParseError {
            kind,
            path,
            errpos,
            msg,
        }
    }

    pub fn kind(&self) -> ParseErrorKind {
        self.kind
    }

    pub fn msg(&self) -> &str {
        &self.msg
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
    errpos: usize,
    msg: &'static str,
}

impl<'path> ErrorInfo<'path> {
    pub fn new(
        kind: ParseErrorKind,
        path: &'path [u8],
        errpos: usize,
        msg: &'static str,
    ) -> ErrorInfo<'path> {
        ErrorInfo {
            kind,
            path,
            errpos,
            msg,
        }
    }

    fn build_error(&self, msg: String) -> ParseError {
        ParseError::new(
            self.kind,
            OsString::from(as_osstr(&self.path[..])),
            self.errpos,
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

    pub fn kind(&self) -> &ParseErrorKind {
        &self.kind
    }

    pub fn path(&self) -> &'path [u8] {
        self.path
    }

    pub fn errpos(&self) -> usize {
        self.errpos
    }

    pub fn msg(&self) -> &'static str {
        self.msg
    }
}

impl<'path> From<&ErrorInfo<'path>> for ParseError {
    fn from(info: &ErrorInfo<'path>) -> ParseError {
        info.to_error()
    }
}

// ===========================================================================
//
// ===========================================================================
