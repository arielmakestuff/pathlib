// tests/windows/simplepath.rs
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

use pathlib::windows::{
    Component as WindowsComponent, Prefix, WindowsMemoryPath,
    WindowsMemoryPathBuf,
};

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
    use std::ffi::OsString;

    #[test]
    fn simple_unix_path() {
        let path = Path::new("/hello/world/greetings/planet");
        let comp: Vec<_> = UnixMemoryPath::iter(path).collect();

        let expected = vec![
            Ok(UnixComponent::RootDir),
            Ok(UnixComponent::Normal(OsString::from("hello"))),
            Ok(UnixComponent::Normal(OsString::from("world"))),
            Ok(UnixComponent::Normal(OsString::from("greetings"))),
            Ok(UnixComponent::Normal(OsString::from("planet"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn simple_windows_path() {
        let path = Path::new("C:/hello/world/greetings/planet");
        let comp: Vec<_> = WindowsMemoryPath::iter(path).collect();

        let expected = vec![
            Ok(WindowsComponent::RootDir(OsStr::new(r"\"))),
            Ok(WindowsComponent::Normal(OsStr::new("hello"))),
            Ok(WindowsComponent::Normal(OsStr::new("world"))),
            Ok(WindowsComponent::Normal(OsStr::new("greetings"))),
            Ok(WindowsComponent::Normal(OsStr::new("planet"))),
        ];

        let result = match comp[0] {
            Ok(WindowsComponent::Prefix(prefix_comp)) => {
                (prefix_comp.as_os_str(), prefix_comp.kind())
                    == (OsStr::new("C:"), Prefix::Disk(b'C'))
            }
            _ => false,
        };
        assert!(result);
        assert_eq!(&comp[1..], &expected[..]);
    }

    #[test]
    fn simple_unix_pathbuf() {
        let path = PathBuf::from("/hello/world/greetings/planet");
        let comp: Vec<_> = UnixMemoryPathBuf::iter(&path).collect();

        let expected = vec![
            Ok(UnixComponent::RootDir),
            Ok(UnixComponent::Normal(OsString::from("hello"))),
            Ok(UnixComponent::Normal(OsString::from("world"))),
            Ok(UnixComponent::Normal(OsString::from("greetings"))),
            Ok(UnixComponent::Normal(OsString::from("planet"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn simple_windows_pathbuf() {
        let path = PathBuf::from("C:/hello/world/greetings/planet");
        let comp: Vec<_> = WindowsMemoryPathBuf::iter(&path).collect();

        let expected = vec![
            Ok(WindowsComponent::RootDir(OsStr::new(r"\"))),
            Ok(WindowsComponent::Normal(OsStr::new("hello"))),
            Ok(WindowsComponent::Normal(OsStr::new("world"))),
            Ok(WindowsComponent::Normal(OsStr::new("greetings"))),
            Ok(WindowsComponent::Normal(OsStr::new("planet"))),
        ];

        let result = match comp[0] {
            Ok(WindowsComponent::Prefix(prefix_comp)) => {
                (prefix_comp.as_os_str(), prefix_comp.kind())
                    == (OsStr::new("C:"), Prefix::Disk(b'C'))
            }
            _ => false,
        };
        assert!(result);
        assert_eq!(&comp[1..], &expected[..]);
    }
}

// ===========================================================================
//
// ===========================================================================
