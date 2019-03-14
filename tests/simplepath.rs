// tests/simplepath.rs
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
use pathlib::unix::Component as UnixComponent;
use pathlib::windows::{Component as WindowsComponent, Prefix};
use pathlib::Path;

// Local imports

// ===========================================================================
// Tests
// ===========================================================================

mod path_type {
    use super::*;

    #[test]
    fn new_unixpathbuf() {
        let path = UnixPathBuf::new();
        let comp: Vec<_> = path.iter().collect();
        let expected = vec![Ok(UnixComponent::CurDir)];

        assert_eq!(comp, expected);
    }

    #[test]
    fn unixpath_derefs_to_path() {
        let path = UnixPath::new("hello");
        let expected = Path::new(OsStr::new("hello"));

        assert_eq!(&*path, expected);
    }

    #[test]
    fn new_windowspathbuf() {
        let path = WindowsPathBuf::new();
        let comp: Vec<_> = path.iter().collect();
        let expected = vec![Ok(WindowsComponent::CurDir)];

        assert_eq!(comp, expected);
    }

    #[test]
    fn windowspath_derefs_to_path() {
        let path = WindowsPath::new("hello");
        let expected = Path::new(OsStr::new("hello"));

        assert_eq!(&*path, expected);
    }
}

mod iter {
    use super::*;
    use pathlib::windows::PrefixComponent;

    #[test]
    fn simple_unix_path() {
        let path = UnixPath::new("/hello/world/greetings/planet");
        let comp: Vec<_> = path.iter().collect();

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
        let path = WindowsPath::new("C:/hello/world/greetings/planet");
        let comp: Vec<_> = path.iter().collect();

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
        let path = UnixPathBuf::from("/hello/world/greetings/planet");
        let comp: Vec<_> = path.iter().collect();

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
        let path = WindowsPathBuf::from("C:/hello/world/greetings/planet");
        let comp: Vec<_> = path.iter().collect();

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
