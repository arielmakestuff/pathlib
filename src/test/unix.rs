// src/test/unix.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use crate::common::error::*;
use std::ffi::{OsStr, OsString};

// Third-party imports

// Local imports
use crate::path::PlatformPath;
use crate::unix::{Component, Iter, PathComponent};

// ===========================================================================
// Tests
// ===========================================================================

mod public_export {
    use super::*;
    use crate::common::string::{as_osstr, as_str};

    mod parseerror {
        use super::*;

        #[test]
        fn source_always_none() {
            let err = ParseError::new(
                UnixErrorKind::InvalidCharacter.into(),
                OsString::from("hello"),
                OsString::from(as_str(b"/hello/world")),
                1,
                6,
                String::from("message"),
            );

            assert!(err.source().is_none());
        }

        #[test]
        fn kind_value() {
            let err = ParseError::new(
                UnixErrorKind::InvalidCharacter.into(),
                OsString::from("hello"),
                OsString::from(as_str(b"/hello/world")),
                1,
                6,
                String::from("message"),
            );

            assert_eq!(err.kind(), UnixErrorKind::InvalidCharacter.into());
        }
    }

    mod component {
        use super::*;

        #[test]
        fn osstr_rootdir() {
            let path = b"/";
            let comp = Component::RootDir;
            let expected = as_osstr(&path[..]);

            assert_eq!(comp.as_os_str(), expected);
        }

        #[test]
        fn osstr_curdir() {
            let path = b".";
            let comp = Component::CurDir;
            let expected = as_osstr(&path[..]);

            assert_eq!(comp.as_os_str(), expected);
        }

        #[test]
        fn osstr_parentdir() {
            let path = b"..";
            let comp = Component::ParentDir;
            let expected = as_osstr(&path[..]);

            assert_eq!(comp.as_os_str(), expected);
        }

        #[test]
        fn osstr_normal() {
            let path = b"hello";
            let comp = Component::Normal(as_osstr(&path[..]));
            let expected = as_osstr(&path[..]);

            assert_eq!(comp.as_os_str(), expected);
        }

        #[test]
        fn str_rootdir() {
            let path = b"/";
            let expected = Component::RootDir;

            let result = Component::from(&path[..]);
            assert_eq!(result, expected);
        }

        #[test]
        fn str_curdir() {
            let path = b".";
            let expected = Component::CurDir;

            let result = Component::from(&path[..]);
            assert_eq!(result, expected);
        }

        #[test]
        fn str_parentdir() {
            let path = b"..";
            let expected = Component::ParentDir;

            let result = Component::from(&path[..]);
            assert_eq!(result, expected);
        }

        #[test]
        fn str_normal() {
            let path = b"hello";
            let expected = Component::Normal(OsStr::new(as_str(path)));

            let result = Component::from(&path[..]);
            assert_eq!(result, expected);
        }
    }
}

mod iter {
    use super::*;

    #[test]
    fn prefix_noroot<'path>() {
        let path = b"hello";
        let iter = Iter::new(PlatformPath::from_bytes(path));

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 1);

        let expected: Vec<PathComponent<'path>> =
            vec![Ok(Component::Normal(OsStr::new("hello")))];

        assert_eq!(comp, expected);
    }

    #[test]
    fn invalid_char<'path>() {
        let path = b"/hello\x00/world";
        let iter = Iter::new(PlatformPath::from_bytes(path));

        let comp: Vec<PathComponent> = iter.collect();

        assert_eq!(comp.len(), 2);

        let expected_ok: Vec<PathComponent<'path>> =
            vec![Ok(Component::RootDir)];

        assert_eq!(&comp[..1], &expected_ok[..]);

        // Check last element is an error
        let result = match &comp[1] {
            Ok(_) => false,
            Err(e) => match e.kind() {
                ParseErrorKind::Unix(UnixErrorKind::InvalidCharacter) => true,
                _ => false,
            },
        };

        assert!(result);
    }

    #[test]
    fn relative_path<'path>() {
        let path = b"hello/world";
        let iter = Iter::new(PlatformPath::from_bytes(path));

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 2);

        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::Normal(OsStr::new(r"hello"))),
            Ok(Component::Normal(OsStr::new(r"world"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn double_path_separator<'path>() {
        let path = br"hello//world";
        let iter = Iter::new(PlatformPath::from_bytes(path));

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::Normal(OsStr::new(r"hello"))),
            Ok(Component::CurDir),
            Ok(Component::Normal(OsStr::new(r"world"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn curdir<'path>() {
        let path = br"hello/world/./what/now";
        let iter = Iter::new(PlatformPath::from_bytes(path));

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 5);

        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::Normal(OsStr::new(r"hello"))),
            Ok(Component::Normal(OsStr::new(r"world"))),
            Ok(Component::CurDir),
            Ok(Component::Normal(OsStr::new(r"what"))),
            Ok(Component::Normal(OsStr::new(r"now"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn parentdir<'path>() {
        let path = br"hello/world/../what/now";
        let iter = Iter::new(PlatformPath::from_bytes(path));

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 5);

        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::Normal(OsStr::new(r"hello"))),
            Ok(Component::Normal(OsStr::new(r"world"))),
            Ok(Component::ParentDir),
            Ok(Component::Normal(OsStr::new(r"what"))),
            Ok(Component::Normal(OsStr::new(r"now"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn curdir_at_start<'path>() {
        let path = br"./hello/world";
        let iter = Iter::new(PlatformPath::from_bytes(path));

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::CurDir),
            Ok(Component::Normal(OsStr::new(r"hello"))),
            Ok(Component::Normal(OsStr::new(r"world"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn parentdir_at_start<'path>() {
        let path = br"../hello/world/";
        let iter = Iter::new(PlatformPath::from_bytes(path));

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::ParentDir),
            Ok(Component::Normal(OsStr::new(r"hello"))),
            Ok(Component::Normal(OsStr::new(r"world"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn absolute_path<'path>() {
        let path = b"/hello/world/what/now/brown/cow";
        let iter = Iter::new(PlatformPath::from_bytes(path));

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 7);

        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::RootDir),
            Ok(Component::Normal(OsStr::new(r"hello"))),
            Ok(Component::Normal(OsStr::new(r"world"))),
            Ok(Component::Normal(OsStr::new(r"what"))),
            Ok(Component::Normal(OsStr::new(r"now"))),
            Ok(Component::Normal(OsStr::new(r"brown"))),
            Ok(Component::Normal(OsStr::new(r"cow"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn empty_path<'path>() {
        let path = b"";
        let iter = Iter::new(PlatformPath::from_bytes(path));

        let comp: Vec<PathComponent> = iter.collect();
        let expected: Vec<PathComponent<'path>> = vec![Ok(Component::CurDir)];

        assert_eq!(comp, expected);
    }

    #[test]
    fn multibyte_chars<'path>() {
        let s = "/multibyte/Löwe 老虎 Léopard";
        let path = s.as_bytes();
        let iter = Iter::new(PlatformPath::from_bytes(path));

        let comp: Vec<PathComponent> = iter.collect();
        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::RootDir),
            Ok(Component::Normal(OsStr::new("multibyte"))),
            Ok(Component::Normal(OsStr::new("Löwe 老虎 Léopard"))),
        ];

        assert_eq!(comp, expected);
    }
}

// ===========================================================================
//
// ===========================================================================
