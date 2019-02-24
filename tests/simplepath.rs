// tests/unixpath.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsStr;

// Third-party imports
use pathlib::prelude::*;

#[cfg(unix)]
use pathlib::windows::{
    Component as WindowsComponent, Prefix, PrefixComponent, WindowsMemoryPath,
    WindowsMemoryPathBuf,
};

#[cfg(windows)]
use pathlib::unix::{
    Component as UnixComponent, UnixMemoryPath, UnixMemoryPathBuf,
};

use pathlib::Path;

// Local imports

// ===========================================================================
// Tests
// ===========================================================================

mod iter {
    use super::*;

    #[test]
    fn simple_unix_path() {
        let path = Path::new("/hello/world/greetings/planet");
        let comp: Vec<_> = UnixMemoryPath::iter(path).collect();

        let expected = vec![
            Ok(UnixComponent::RootDir),
            Ok(UnixComponent::Normal(OsStr::new("hello"))),
            Ok(UnixComponent::Normal(OsStr::new("world"))),
            Ok(UnixComponent::Normal(OsStr::new("greetings"))),
            Ok(UnixComponent::Normal(OsStr::new("planet"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn simple_windows_path() {
        let path = Path::new("C:/hello/world/greetings/planet");
        let comp: Vec<_> = WindowsMemoryPath::iter(path).collect();

        let expected = vec![
            Ok(WindowsComponent::Prefix(PrefixComponent::new(
                b"C:",
                Prefix::Disk(b'C'),
            ))),
            Ok(WindowsComponent::RootDir(OsStr::new(r"/"))),
            Ok(WindowsComponent::Normal(OsStr::new("hello"))),
            Ok(WindowsComponent::Normal(OsStr::new("world"))),
            Ok(WindowsComponent::Normal(OsStr::new("greetings"))),
            Ok(WindowsComponent::Normal(OsStr::new("planet"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn simple_unix_pathbuf() {
        let path = PathBuf::from("/hello/world/greetings/planet");
        let comp: Vec<_> = UnixMemoryPathBuf::iter(&path).collect();

        let expected = vec![
            Ok(UnixComponent::RootDir),
            Ok(UnixComponent::Normal(OsStr::new("hello"))),
            Ok(UnixComponent::Normal(OsStr::new("world"))),
            Ok(UnixComponent::Normal(OsStr::new("greetings"))),
            Ok(UnixComponent::Normal(OsStr::new("planet"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn simple_windows_pathbuf() {
        let path = PathBuf::from("C:/hello/world/greetings/planet");
        let comp: Vec<_> = WindowsMemoryPathBuf::iter(&path).collect();

        let expected = vec![
            Ok(WindowsComponent::Prefix(PrefixComponent::new(
                b"C:",
                Prefix::Disk(b'C'),
            ))),
            Ok(WindowsComponent::RootDir(OsStr::new(r"/"))),
            Ok(WindowsComponent::Normal(OsStr::new("hello"))),
            Ok(WindowsComponent::Normal(OsStr::new("world"))),
            Ok(WindowsComponent::Normal(OsStr::new("greetings"))),
            Ok(WindowsComponent::Normal(OsStr::new("planet"))),
        ];

        assert_eq!(comp, expected);
    }
}

// ===========================================================================
//
// ===========================================================================
