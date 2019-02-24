// src/windows/iter.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::{Components, Path as StdPath};

// Third-party imports

// Local imports
use super::path_type::{Device, NonDevicePart};
use crate::common::error::ParseError;

use crate::path::{Path, PathBuf};

use super::PathParseState;

// ===========================================================================
// Re-exports
// ===========================================================================

pub use std::path::{
    Component, Prefix, PrefixComponent,
};

// ===========================================================================
// Constants
// ===========================================================================

use super::SEPARATOR;

// ===========================================================================
// Error types
// ===========================================================================

use super::WindowsErrorKind;

// ===========================================================================
// Iter
// ===========================================================================

pub type PathComponent<'path> = Result<Component<'path>, ParseError>;

// Implement AsRef<OsStr> and AsRef<Path> for Component
impl<'path> AsRef<OsStr> for Component<'path> {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl<'path> AsRef<Path> for Component<'path> {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Iter<'path> {
    source: &'path StdPath,
    iter: Components<'path>,
    parse_state: PathParseState,
    comp: Option<StdComponent<'path>>,
    cur: usize,
}

impl<'path> Iter<'path> {
    pub fn new(path: &'path Path) -> Iter<'path> {
        let stdpath = StdPath::new(path);
        let iter = stdpath.components();
        Iter {
            source: stdpath,
            iter,
            parse_state: PathParseState::Start,
            comp: None,
            cur: 0,
        }
    }

    // self.cur must always be set to something when entering this method
    fn prefix(&mut self) -> Option<PathComponent<'path>> {
        let mut verbatimdisk = false;
        let mut ret = None;
        let comp = self.comp.expect("No component found");
        if let StdComponent::Prefix(prefix_comp) = comp {
            if let Prefix::VerbatimDisk(_) = prefix_comp.kind() {
                verbatimdisk = true;
            }

            self.cur += prefix_comp.len();
            ret = Some(Ok(Component::Prefix(prefix_comp)));
        }

        self.parse_state = PathParseState::Prefix { verbatimdisk };

        if ret.is_some() {
            return ret;
        }

        self.root(verbatimdisk)
    }

    fn root(
        &mut self,
        verbatimdisk: bool,
    ) -> Option<PathComponent<'path>> {
        let comp = self.comp.expect("No component found");
        match comp {
            StdComponent::RootDir => {
                self.parse_state = PathParseState::Root;
                if verbatimdisk {
                    self.cur += comp.as_os_str().len();
                    Some(Ok(comp))
                }
            }
            _ => {
                self.component()
            }
        }
    }

    fn component(&mut self) -> Option<PathComponent<'path>> {
        let ret = match self.comp {
            None => {
                self.parse_state = PathParseState::Finish;
                return None;
            }
            Some(comp) => self.to_comp(comp),
        };

        match self.parse_state {
            PathParseState::Finish | PathParseState::PathComponent => {}
            _ => self.parse_state = PathParseState::PathComponent,
        }

        ret
    }

    fn to_comp(
        &mut self,
        comp: Component<'path>
    ) -> Result<Component<'path>, ParseError> {
        let part_str = match comp {
            Component::Normal(p) => {
                let bytes: Vec<u16> = p.encode_wide().collect();
                String::from_utf16_lossy(&bytes[..])
            }
            _ => return Ok(comp);
        };
        let part = &part_str[..];
        let comp_len = comp.as_os_str().len();
        if part != NonDevicePart {
            let start = self.cur;
            let end = start + comp_len;
            if part == Device {
                self.invalid_name(comp, start, end)
            } else {
                self.invalid_char(comp, start, end)
            }
        } else {
            self.cur += comp_len;
            Ok(comp)
        }
    }

    fn invalid_name(
        &mut self,
        comp: Component<'path>,
        start: usize,
        end: usize,
    ) -> Result<Component<'path>, ParseError> {
        // Return None for every call to next() after this
        self.parse_state = PathParseState::Finish;

        let msg = String::from("component uses a restricted name");
        let errkind = WindowsErrorKind::RestrictedName;
        self.build_error(errkind, comp, start, end, msg)
    }

    fn invalid_char(
        &mut self,
        comp: Component<'path>,
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
        comp: Component<'path>,
        start: usize,
        end: usize,
        msg: String,
    ) -> Result<Component<'path>, ParseError> {
        let err = ParseError::new(
            kind.into(),
            comp.as_os_str().to_os_string(),
            self.source.as_os_str().to_os_string(),
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

macro_rules! get_iter_item {
    () => {
        self.cur = self.iter.next();
        if self.cur.is_none() {
            self.parse_state = PathParseState::Finish;
            return None;
        }
    };
}

impl<'path> Iterator for Iter<'path> {
    type Item = PathComponent<'path>

    fn next(&mut self) -> Option<PathComponent<'path>> {
        match self.parse_state {
            PathParseState::Start => {
                get_iter_item!();
                self.prefix()
            },
            PathParseState::Prefix { verbatimdisk } => {
                get_iter_item!();
                self.root(verbatimdisk)
            }
            PathParseState::Root | PathParseState::PathComponent => {
                get_iter_item!();
                self.component()
            }
            PathParseState::Finish => None,
        }
    }
}

impl<'path> AsRef<Path> for Iter<'path> {
    fn as_ref(&self) -> &Path {
        self.iter.as_ref()
    }
}

// ===========================================================================
//
// ===========================================================================
