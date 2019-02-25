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
use std::path::{Component as StdComponent, Components, Path as StdPath};

// Third-party imports

// Local imports
use super::path_type::{Device, NonDevicePart};
use crate::common::error::ParseError;

use crate::path::Path;

use super::PathParseState;

// ===========================================================================
// Re-exports
// ===========================================================================

pub use std::path::{Prefix, PrefixComponent};

// ===========================================================================
// Error types
// ===========================================================================

use super::WindowsErrorKind;

// ===========================================================================
// Iter
// ===========================================================================

pub type PathComponent<'path> = Result<Component<'path>, ParseError>;

#[derive(Debug, Eq, PartialEq)]
pub enum Component<'path> {
    Prefix(PrefixComponent<'path>),
    RootDir(&'path OsStr),
    CurDir,
    ParentDir,
    Normal(&'path OsStr),
}

impl<'path> Component<'path> {
    pub fn as_os_str(&self) -> &'path OsStr {
        match self {
            Component::Prefix(prefix_str) => prefix_str.as_os_str(),
            Component::RootDir(rootdir) => rootdir,
            Component::CurDir => OsStr::new("."),
            Component::ParentDir => OsStr::new(".."),
            Component::Normal(comp) => comp,
        }
    }
}

impl<'path> From<StdComponent<'path>> for Component<'path> {
    fn from(c: StdComponent<'path>) -> Component<'path> {
        match c {
            StdComponent::Prefix(p) => Component::Prefix(p),
            StdComponent::RootDir => Component::RootDir(c.as_os_str()),
            StdComponent::CurDir => Component::CurDir,
            StdComponent::ParentDir => Component::ParentDir,
            StdComponent::Normal(p) => Component::Normal(p),
        }
    }
}

impl<'path> From<Component<'path>> for StdComponent<'path> {
    fn from(c: Component<'path>) -> StdComponent<'path> {
        match c {
            Component::Prefix(p) => StdComponent::Prefix(p),
            Component::RootDir(_) => StdComponent::RootDir,
            Component::CurDir => StdComponent::CurDir,
            Component::ParentDir => StdComponent::ParentDir,
            Component::Normal(p) => StdComponent::Normal(p),
        }
    }
}

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

            self.cur += prefix_comp.as_os_str().len();
            ret = Some(Ok(Component::Prefix(prefix_comp)));
        }

        self.parse_state = PathParseState::Prefix { verbatimdisk };

        if ret.is_some() {
            return ret;
        }

        self.root(verbatimdisk)
    }

    fn root(&mut self, verbatimdisk: bool) -> Option<PathComponent<'path>> {
        let comp = self.comp.expect("No component found");
        match comp {
            StdComponent::RootDir => {
                self.parse_state = PathParseState::Root;
                if verbatimdisk {
                    self.cur += comp.as_os_str().len();
                    Some(Ok(comp.into()))
                } else {
                    self.component()
                }
            }
            _ => self.component(),
        }
    }

    fn component(&mut self) -> Option<PathComponent<'path>> {
        let ret = match self.comp {
            None => {
                self.parse_state = PathParseState::Finish;
                return None;
            }
            Some(comp) => self.build_comp(comp),
        };

        match self.parse_state {
            PathParseState::Finish | PathParseState::PathComponent => {}
            _ => self.parse_state = PathParseState::PathComponent,
        }

        Some(ret)
    }

    fn build_comp(
        &mut self,
        comp: StdComponent<'path>,
    ) -> Result<Component<'path>, ParseError> {
        let part_str = match comp {
            StdComponent::Normal(p) => {
                let bytes: Vec<u16> = p.encode_wide().collect();
                String::from_utf16_lossy(&bytes[..])
            }
            _ => return Ok(comp.into()),
        };
        let part = part_str.as_bytes();
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
            // If haven't gotten to the end of the path string, add 1 to
            // represent a separator since any following component will only be
            // a path component
            if self.cur < self.source.as_os_str().len() {
                self.cur += 1;
            }
            Ok(comp.into())
        }
    }

    fn invalid_name(
        &mut self,
        comp: StdComponent<'path>,
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
        comp: StdComponent<'path>,
        start: usize,
        end: usize,
    ) -> Result<Component<'path>, ParseError> {
        // Return None for every call to next() after this
        self.parse_state = PathParseState::Finish;
        let msg = String::from("path component contains an invalid character");
        self.build_error(
            WindowsErrorKind::InvalidCharacter,
            comp,
            start,
            end,
            msg,
        )
    }

    fn build_error(
        &self,
        kind: WindowsErrorKind,
        comp: StdComponent<'path>,
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
    ($iter:expr) => {
        $iter.comp = $iter.iter.next();
        if $iter.comp.is_none() {
            $iter.parse_state = PathParseState::Finish;
            return None;
        }
    };
}

impl<'path> Iterator for Iter<'path> {
    type Item = PathComponent<'path>;

    fn next(&mut self) -> Option<PathComponent<'path>> {
        match self.parse_state {
            PathParseState::Start => {
                get_iter_item!(self);
                self.prefix()
            }
            PathParseState::Prefix { verbatimdisk } => {
                get_iter_item!(self);
                self.root(verbatimdisk)
            }
            PathParseState::Root | PathParseState::PathComponent => {
                get_iter_item!(self);
                self.component()
            }
            PathParseState::Finish => None,
        }
    }
}

impl<'path> AsRef<Path> for Iter<'path> {
    fn as_ref(&self) -> &Path {
        Path::new(self.iter.as_path())
    }
}

// ===========================================================================
//
// ===========================================================================
