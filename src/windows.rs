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
use self::path_type::NonDevicePart;

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
// PathIterator
// ===========================================================================

// The unsafe is safe since we're not modifying the slice at all, and we will
// only be checking for ascii characters
fn as_str<'path>(path: &'path [u8]) -> &'path str {
    unsafe { str::from_utf8_unchecked(path) }
}

fn as_osstr<'path>(path: &'path [u8]) -> &'path OsStr {
    OsStr::new(as_str(path))
}

#[derive(Debug, Eq, PartialEq)]
pub enum Component<'path> {
    Prefix(PrefixComponent<'path>),
    RootDir(&'path OsStr),
    CurDir,
    ParentDir,
    Normal(&'path OsStr),
    Error {
        component: &'path OsStr,
        path: &'path OsStr,
        start: usize,
        end: usize,
        msg: String,
    },
}

impl<'path> Component<'path> {
    pub fn as_os_str(self) -> &'path OsStr {
        match self {
            Component::Prefix(prefix_str) => prefix_str.as_os_str(),
            Component::RootDir(rootdir) => rootdir,
            Component::CurDir => OsStr::new("."),
            Component::ParentDir => OsStr::new(".."),
            Component::Normal(comp) => comp,
            Component::Error { component, .. } => component,
        }
    }
}

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
pub struct PathIterator<'path> {
    path: &'path [u8],
    parse_state: PathParseState,
    cur: usize,
}

impl<'path> PathIterator<'path> {
    pub fn new(path: &[u8]) -> PathIterator {
        PathIterator {
            path: path,
            parse_state: PathParseState::Start,
            cur: 0,
        }
    }

    fn parse_prefix(&mut self) -> Option<Component<'path>> {
        let mut verbatimdisk = false;
        let mut ret = None;
        if let Some((end, prefix)) = match_prefix(self.path) {
            if let Prefix::VerbatimDisk(_) = prefix {
                verbatimdisk = true;
            }
            let prefix_comp = PrefixComponent::new(&self.path[..end], prefix);
            self.cur = end;

            ret = Some(Component::Prefix(prefix_comp));
        }

        self.parse_state = PathParseState::Prefix { verbatimdisk };

        if ret.is_some() {
            return ret;
        }

        self.parse_root(verbatimdisk)
    }

    fn parse_root(&mut self, verbatimdisk: bool) -> Option<Component<'path>> {
        if self.cur == self.path.len() {
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
            return Some(Component::RootDir(as_osstr(&self.path[start..end])));
        }

        self.parse_component()
    }

    fn parse_component(&mut self) -> Option<Component<'path>> {
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
                self.cur = i + 1;
                let part = &self.path[cur..i];
                let comp = if part.len() == 0 {
                    Component::CurDir
                } else {
                    self.to_comp(part)
                };
                ret = Some(comp);
                break;
            }
        }

        self.parse_state = PathParseState::PathComponent;
        match ret {
            Some(_) => ret,
            None => {
                self.cur = end;
                let comp = self.to_comp(&self.path[cur..end]);
                Some(comp)
            }
        }
    }

    fn to_comp(&mut self, part: &'path [u8]) -> Component<'path> {
        let comp_str = as_str(part);
        if part != NonDevicePart {
            self.parse_error(comp_str)
        } else {
            match comp_str {
                "." => Component::CurDir,
                ".." => Component::ParentDir,
                _ => Component::Normal(OsStr::new(comp_str)),
            }
        }
    }

    fn parse_error(&mut self, part: &'path str) -> Component<'path> {
        // Return None for every call to next() after this
        self.parse_state = PathParseState::Finish;

        let msg = String::from("Path component includes invalid character");
        Component::Error {
            component: OsStr::new(part),
            path: as_osstr(self.path),
            start: self.cur,
            end: self.cur + part.len(),
            msg: msg,
        }
    }
}

impl<'path> Iterator for PathIterator<'path> {
    type Item = Component<'path>;

    fn next(&mut self) -> Option<Component<'path>> {
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

// ===========================================================================
//
// ===========================================================================