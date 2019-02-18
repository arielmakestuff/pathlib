// src/windows.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

mod match_prefix;
mod path_type;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
// use std::cmp::PartialEq;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::Prefix;
use std::str;

// Third-party imports
use lazy_static::lazy_static;

// Local imports
use self::match_prefix::match_prefix;
use self::path_type::{Device, NonDevicePart};
use crate::common::error::ParseError;
use crate::common::string::{as_osstr, as_str};
use crate::common::{AsPath, PathData};
use crate::path::Path;
use crate::{component_asref_impl, pathiter_trait_impl};

// ===========================================================================
// Constants
// ===========================================================================

lazy_static! {
    static ref SEPARATOR: HashSet<u8> = {
        let sep_chars = r#"\/"#;
        let mut all_sep = HashSet::with_capacity(sep_chars.len());
        for s in sep_chars.chars() {
            all_sep.insert(s as u8);
        }
        all_sep
    };
    static ref DRIVE_LETTERS: HashSet<char> = {
        let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut all_letters = HashSet::with_capacity(letters.len());
        for l in letters.chars() {
            all_letters.insert(l);
        }
        all_letters
    };
    static ref RESERVED_NAMES: HashSet<String> = {
        let base = ["CON", "PRN", "AUX", "NUL"];
        let numbered_base = ["COM", "LPT"];
        let mut reserved = HashSet::with_capacity(22);
        for b in base.iter() {
            reserved.insert(b.to_string());
        }

        for b in numbered_base.iter() {
            for i in 1..=9 {
                reserved.insert(format!("{}{}", b, i));
            }
        }
        reserved
    };
    static ref RESTRICTED_CHARS: HashSet<u8> = {
        let chars = r#"<>:"/\|?*"#;
        let mut all_chars = HashSet::with_capacity(chars.len());
        for c in chars.chars() {
            all_chars.insert(c as u8);
        }

        // These are ascii chars code 0 - 31
        for i in 0..=31 {
            all_chars.insert(i);
        }
        all_chars
    };
}

// ===========================================================================
// Error types
// ===========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowsErrorKind {
    InvalidCharacter,
    RestrictedName,
}

// ===========================================================================
// Iter
// ===========================================================================

pub type PathComponent<'path> = Result<Component<'path>, ParseError<'path>>;

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

// Implement AsRef<OsStr> and AsRef<Path> for Component
component_asref_impl!(Component, 'path);

#[derive(Debug, Eq, PartialEq)]
pub struct PrefixComponent<'path> {
    raw: &'path OsStr,
    parsed: Prefix<'path>,
}

impl<'path> PrefixComponent<'path> {
    pub fn new(path: &'path [u8], prefix: Prefix<'path>) -> Self {
        PrefixComponent {
            raw: as_osstr(path),
            parsed: prefix,
        }
    }

    pub fn kind(&self) -> Prefix<'path> {
        self.parsed
    }

    pub fn as_os_str(&self) -> &'path OsStr {
        self.raw
    }
}

#[derive(Debug, Eq, PartialEq)]
enum PathParseState {
    Start,
    Prefix { verbatimdisk: bool },
    Root,
    PathComponent,
    Finish,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Iter<'path> {
    path: &'path [u8],
    parse_state: PathParseState,
    cur: usize,
}

impl<'path> Iter<'path> {
    pub fn new(path: &[u8]) -> Iter {
        Iter {
            path: path,
            parse_state: PathParseState::Start,
            cur: 0,
        }
    }

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
                let comp = if part.len() == 0 {
                    Ok(Component::CurDir)
                } else {
                    self.to_comp(part)
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
                let comp = self.to_comp(&self.path[cur..end]);
                self.cur = end;
                Some(comp)
            }
        }
    }

    fn to_comp(
        &mut self,
        part: &'path [u8],
    ) -> Result<Component<'path>, ParseError<'path>> {
        let comp_str = as_str(part);
        if part != NonDevicePart {
            if part == Device {
                self.invalid_name(comp_str)
            } else {
                self.invalid_char(comp_str)
            }
        } else {
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
        part: &'path str,
    ) -> Result<Component<'path>, ParseError<'path>> {
        // Return None for every call to next() after this
        self.parse_state = PathParseState::Finish;

        let msg = String::from("component uses a restricted name");
        self.build_error(WindowsErrorKind::RestrictedName, part, msg)
    }

    fn invalid_char(
        &mut self,
        part: &'path str,
    ) -> Result<Component<'path>, ParseError<'path>> {
        // Return None for every call to next() after this
        self.parse_state = PathParseState::Finish;
        let msg = String::from("path component contains an invalid character");
        self.build_error(WindowsErrorKind::InvalidCharacter, part, msg)
    }

    fn build_error(
        &self,
        kind: WindowsErrorKind,
        part: &'path str,
        msg: String,
    ) -> Result<Component<'path>, ParseError<'path>> {
        let err = ParseError::new(
            kind.into(),
            OsStr::new(part),
            as_osstr(self.path),
            self.cur,
            self.cur + part.len(),
            msg,
        );

        Err(err)
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

// Implement PathData and AsPath traits for Iter
pathiter_trait_impl!(Iter, 'path);

// ===========================================================================
//
// ===========================================================================
