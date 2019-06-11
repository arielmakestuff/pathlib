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
use pathlib::SystemStr;

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
        let expected = vec![UnixComponent::CurDir];

        assert_eq!(comp, expected);
    }

    #[test]
    fn unixpath_derefs_to_path() {
        let path = UnixPath::new("hello");
        let expected = SystemStr::new(OsStr::new("hello"));

        assert_eq!(&**path, expected);
    }

    #[test]
    fn new_windowspathbuf() {
        let path = WindowsPathBuf::new();
        let comp: Vec<_> = path.iter().collect();
        let expected = vec![WindowsComponent::CurDir];

        assert_eq!(comp, expected);
    }

    #[test]
    fn windowspath_derefs_to_path() {
        let path = WindowsPath::new("hello");
        let expected = SystemStr::new(OsStr::new("hello"));

        assert_eq!(&**path, expected);
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
            UnixComponent::RootDir,
            UnixComponent::Normal(OsStr::new("hello")),
            UnixComponent::Normal(OsStr::new("world")),
            UnixComponent::Normal(OsStr::new("greetings")),
            UnixComponent::Normal(OsStr::new("planet")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn simple_windows_path() {
        let path = WindowsPath::new("C:/hello/world/greetings/planet");
        let comp: Vec<_> = path.iter().collect();

        let expected = vec![
            WindowsComponent::Prefix(PrefixComponent::new(
                b"C:",
                Prefix::Disk(b'C'),
            )),
            WindowsComponent::RootDir(OsStr::new(r"/")),
            WindowsComponent::Normal(OsStr::new("hello")),
            WindowsComponent::Normal(OsStr::new("world")),
            WindowsComponent::Normal(OsStr::new("greetings")),
            WindowsComponent::Normal(OsStr::new("planet")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn simple_unix_pathbuf() {
        let path = UnixPathBuf::from("/hello/world/greetings/planet");
        let comp: Vec<_> = path.iter().collect();

        let expected = vec![
            UnixComponent::RootDir,
            UnixComponent::Normal(OsStr::new("hello")),
            UnixComponent::Normal(OsStr::new("world")),
            UnixComponent::Normal(OsStr::new("greetings")),
            UnixComponent::Normal(OsStr::new("planet")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn simple_windows_pathbuf() {
        let path = WindowsPathBuf::from("C:/hello/world/greetings/planet");
        let comp: Vec<_> = path.iter().collect();

        let expected = vec![
            WindowsComponent::Prefix(PrefixComponent::new(
                b"C:",
                Prefix::Disk(b'C'),
            )),
            WindowsComponent::RootDir(OsStr::new(r"/")),
            WindowsComponent::Normal(OsStr::new("hello")),
            WindowsComponent::Normal(OsStr::new("world")),
            WindowsComponent::Normal(OsStr::new("greetings")),
            WindowsComponent::Normal(OsStr::new("planet")),
        ];

        assert_eq!(comp, expected);
    }
}

mod deref {
    use super::*;
    use pathlib::windows::WindowsPath;

    #[test]
    fn unixpath_as_bytes() {
        // --------------------
        // GIVEN
        // --------------------
        // a UnixPath
        let path = UnixPath::new("hello");

        // --------------------
        // WHEN
        // --------------------
        // Path::as_bytes() is called on UnixPath
        let result = path.as_bytes();
        let bytes_ref: &[u8] = path.as_ref();

        // --------------------
        // THEN
        // --------------------
        // the UnixPath is auto-dereferenced to SystemStr::as_bytes() and
        // called
        let expected = b"hello";

        assert_eq!(result, expected);
        assert_eq!(bytes_ref, expected);
    }

    #[test]
    fn unixpath_as_os_str() {
        // --------------------
        // GIVEN
        // --------------------
        // a UnixPath
        let path = UnixPath::new("world");

        // --------------------
        // WHEN
        // --------------------
        // Path::as_os_str() is called on UnixPath
        let result = path.as_os_str();
        let os_str_ref: &OsStr = path.as_ref();

        // --------------------
        // THEN
        // --------------------
        // the UnixPath is auto-dereferences to SystemStr::as_os_str() and
        // called
        let expected = OsStr::new("world");

        assert_eq!(result, expected);
        assert_eq!(os_str_ref, expected);
    }

    #[test]
    fn windowspath_as_bytes() {
        // --------------------
        // GIVEN
        // --------------------
        // a WindowsPath

        // --------------------
        // WHEN
        // --------------------
        // Path::as_bytes() is called on WindowsPath
        let path = WindowsPath::new("hello");
        let bytes_ref: &[u8] = path.as_ref();

        // --------------------
        // THEN
        // --------------------
        // the WindowsPath is auto-dereferenced to SystemStr::as_bytes() and
        // called
        let expected = b"hello";

        assert_eq!(path.as_bytes(), expected);
        assert_eq!(bytes_ref, expected);
    }

    #[test]
    fn windowspath_as_os_str() {
        // --------------------
        // GIVEN
        // --------------------
        // a WindowsPath
        let path = WindowsPath::new("world");

        // --------------------
        // WHEN
        // --------------------
        // Path::as_os_str() is called on WindowsPath
        let result = path.as_os_str();
        let os_str_ref: &OsStr = path.as_ref();

        // --------------------
        // THEN
        // --------------------
        // the WindowsPath is auto-dereferenced to SystemStr::as_os_str() and
        // called
        let expected = OsStr::new("world");

        assert_eq!(result, expected);
        assert_eq!(os_str_ref, expected);
    }
}

// ===========================================================================
//
// ===========================================================================
