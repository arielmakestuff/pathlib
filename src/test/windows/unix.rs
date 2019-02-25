// src/test/unix.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

pub mod path;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use crate::common::error::*;
use std::ffi::OsString;

// Third-party imports

// Local imports
use crate::path::PathBuf;
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
            let path = "hello";
            let comp = Component::Normal(OsString::from(path));
            let expected = as_osstr(path.as_bytes());

            assert_eq!(comp.as_os_str(), expected);
        }

        #[test]
        fn str_rootdir() {
            let path = [b'/' as u16];
            let expected = Component::RootDir;

            let result = Component::from(&path[..]);
            assert_eq!(result, expected);
        }

        #[test]
        fn str_curdir() {
            let path = [b'.' as u16];
            let expected = Component::CurDir;

            let result = Component::from(&path[..]);
            assert_eq!(result, expected);
        }

        #[test]
        fn str_parentdir() {
            let path = [b'.' as u16; 2];
            let expected = Component::ParentDir;

            let result = Component::from(&path[..]);
            assert_eq!(result, expected);
        }

        #[test]
        fn str_normal() {
            let path = b"hello";
            let expected = Component::Normal(OsString::from(as_str(path)));
            let path: Vec<u16> = path.iter().map(|&e| e as u16).collect();

            let result = Component::from(&path[..]);
            assert_eq!(result, expected);
        }
    }
}

mod iter {
    use super::*;

    #[test]
    fn prefix_noroot() {
        let path = b"hello";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 1);

        let expected: Vec<PathComponent> =
            vec![Ok(Component::Normal(OsString::from("hello")))];

        assert_eq!(comp, expected);
    }

    #[test]
    fn invalid_char() {
        let path = b"/hello\x00/world";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();

        assert_eq!(comp.len(), 2);

        let expected_ok: Vec<PathComponent> =
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
    fn relative_path() {
        let path = b"hello/world";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 2);

        let expected: Vec<PathComponent> = vec![
            Ok(Component::Normal(OsString::from(r"hello"))),
            Ok(Component::Normal(OsString::from(r"world"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn double_path_separator() {
        let path = br"hello//world";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<PathComponent> = vec![
            Ok(Component::Normal(OsString::from(r"hello"))),
            Ok(Component::CurDir),
            Ok(Component::Normal(OsString::from(r"world"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn curdir() {
        let path = br"hello/world/./what/now";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 5);

        let expected: Vec<PathComponent> = vec![
            Ok(Component::Normal(OsString::from(r"hello"))),
            Ok(Component::Normal(OsString::from(r"world"))),
            Ok(Component::CurDir),
            Ok(Component::Normal(OsString::from(r"what"))),
            Ok(Component::Normal(OsString::from(r"now"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn parentdir() {
        let path = br"hello/world/../what/now";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 5);

        let expected: Vec<PathComponent> = vec![
            Ok(Component::Normal(OsString::from(r"hello"))),
            Ok(Component::Normal(OsString::from(r"world"))),
            Ok(Component::ParentDir),
            Ok(Component::Normal(OsString::from(r"what"))),
            Ok(Component::Normal(OsString::from(r"now"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn curdir_at_start() {
        let path = br"./hello/world";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<PathComponent> = vec![
            Ok(Component::CurDir),
            Ok(Component::Normal(OsString::from(r"hello"))),
            Ok(Component::Normal(OsString::from(r"world"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn parentdir_at_start() {
        let path = br"../hello/world/";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<PathComponent> = vec![
            Ok(Component::ParentDir),
            Ok(Component::Normal(OsString::from(r"hello"))),
            Ok(Component::Normal(OsString::from(r"world"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn absolute_path() {
        let path = b"/hello/world/what/now/brown/cow";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 7);

        let expected: Vec<PathComponent> = vec![
            Ok(Component::RootDir),
            Ok(Component::Normal(OsString::from(r"hello"))),
            Ok(Component::Normal(OsString::from(r"world"))),
            Ok(Component::Normal(OsString::from(r"what"))),
            Ok(Component::Normal(OsString::from(r"now"))),
            Ok(Component::Normal(OsString::from(r"brown"))),
            Ok(Component::Normal(OsString::from(r"cow"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn empty_path() {
        let path = b"";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        let expected: Vec<PathComponent> = vec![Ok(Component::CurDir)];

        assert_eq!(comp, expected);
    }

    #[test]
    fn multibyte_chars() {
        let s = "/multibyte/Löwe 老虎 Léopard";
        let path = s.as_bytes();
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        let expected: Vec<PathComponent> = vec![
            Ok(Component::RootDir),
            Ok(Component::Normal(OsString::from("multibyte"))),
            Ok(Component::Normal(OsString::from("Löwe 老虎 Léopard"))),
        ];

        assert_eq!(comp, expected);
    }
}

// ===========================================================================
//
// ===========================================================================
