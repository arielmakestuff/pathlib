// src/test/windows.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsStr;
use std::path::Prefix;

// Third-party imports

// Local imports
use crate::windows::{
    Component, ParseErrorKind, PathComponent, PathIterator, PrefixComponent,
};

// ===========================================================================
// Tests
// ===========================================================================

mod public_export {
    use super::*;
    use std::str;

    fn as_str<'path>(path: &'path [u8]) -> &'path str {
        unsafe { str::from_utf8_unchecked(path) }
    }

    fn as_osstr<'path>(path: &'path [u8]) -> &'path OsStr {
        OsStr::new(as_str(path))
    }

    mod parseerror {
        use super::*;
        use crate::windows::{test::NewParseError, ParseError};
        use std::error::Error;

        #[test]
        fn source_always_none() {
            let err = ParseError::new(
                ParseErrorKind::RestrictedName,
                OsStr::new("hello"),
                as_osstr(b"/hello/world"),
                1,
                6,
                String::from("message"),
            );

            assert!(err.source().is_none());
        }
    }

    mod component {
        use super::*;
        use std::path::Prefix;

        #[test]
        fn osstr_prefix() {
            let path = br#"\\?\hello\world"#;
            let prefix = Prefix::Verbatim(OsStr::new("hello"));
            let prefix_comp = PrefixComponent::new(&path[..], prefix);
            let comp = Component::Prefix(prefix_comp);
            let expected = &path[..];

            assert_eq!(comp.as_os_str(), as_osstr(expected));
        }

        #[test]
        fn osstr_rootdir() {
            let path = br#"\"#;
            let comp = Component::RootDir(OsStr::new(as_str(&path[..])));
            let expected = as_osstr(&path[..]);

            assert_eq!(comp.as_os_str(), expected);
        }

        #[test]
        fn osstr_curdir() {
            let path = br#"."#;
            let comp = Component::CurDir;
            let expected = as_osstr(&path[..]);

            assert_eq!(comp.as_os_str(), expected);
        }

        #[test]
        fn osstr_parentdir() {
            let path = br#".."#;
            let comp = Component::ParentDir;
            let expected = as_osstr(&path[..]);

            assert_eq!(comp.as_os_str(), expected);
        }

        #[test]
        fn osstr_normal() {
            let path = br#"hello"#;
            let comp = Component::Normal(as_osstr(&path[..]));
            let expected = as_osstr(&path[..]);

            assert_eq!(comp.as_os_str(), expected);
        }
    }

    mod prefixcomponent {
        use super::*;

        #[test]
        fn kind() {
            let path = br#"\\?\hello\world"#;
            let prefix = Prefix::Verbatim(OsStr::new("hello"));
            let prefix_comp = PrefixComponent::new(&path[..], prefix.clone());

            assert_eq!(prefix_comp.kind(), prefix);
        }
    }
}

mod pathiterator {
    use super::*;

    #[test]
    fn verbatim_disk<'path>() {
        let path = br"\\?\C:\hello";

        let iter = PathIterator::new(path);
        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::Prefix(PrefixComponent::new(
                br"\\?\C:\",
                Prefix::VerbatimDisk(b'C'),
            ))),
            Ok(Component::RootDir(OsStr::new(r"\"))),
            Ok(Component::Normal(OsStr::new(r"hello"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn prefix_noroot<'path>() {
        let path = br"C:";

        let iter = PathIterator::new(path);
        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 1);

        let expected: Vec<PathComponent<'path>> = vec![Ok(Component::Prefix(
            PrefixComponent::new(br"C:", Prefix::Disk(b'C')),
        ))];

        assert_eq!(comp, expected);
    }

    #[test]
    fn invalid_char<'path>() {
        let path = br"C:\hello.";

        let iter = PathIterator::new(path);
        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected_ok: Vec<PathComponent<'path>> = vec![
            Ok(Component::Prefix(PrefixComponent::new(
                br"C:",
                Prefix::Disk(b'C'),
            ))),
            Ok(Component::RootDir(OsStr::new(r"\"))),
        ];

        assert_eq!(&comp[..2], &expected_ok[..]);

        // Check last element is an error
        let result = match &comp[2] {
            Ok(_) => false,
            Err(e) => match e.kind() {
                ParseErrorKind::InvalidCharacter => true,
                _ => false,
            },
        };

        assert!(result);
    }

    #[test]
    fn verbatim_path<'path>() {
        let path = br"\\?\hello\world";

        let iter = PathIterator::new(path);
        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::Prefix(PrefixComponent::new(
                br"\\?\hello",
                Prefix::Verbatim(OsStr::new(r"hello")),
            ))),
            Ok(Component::RootDir(OsStr::new(r"\"))),
            Ok(Component::Normal(OsStr::new(r"world"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn invalid_filename<'path>() {
        // --------------------
        // GIVEN
        // --------------------
        // an absolute path with a file using a restricted name
        let path = br"\\?\hello\nul.txt";

        // --------------------
        // WHEN
        // --------------------
        // iterating over the path
        let iter = PathIterator::new(path);
        let comp: Vec<PathComponent> = iter.collect();

        // --------------------
        // THEN
        // --------------------
        // the returned value is a vec with 3 elements and
        // the first 2 elements of the vec are prefix and rootdir
        //     components respectively and
        // the last element is an error and
        // the error is a ParseErrorKind::RestrictedName kind
        assert_eq!(comp.len(), 3);

        let expected_ok: Vec<PathComponent<'path>> = vec![
            Ok(Component::Prefix(PrefixComponent::new(
                br"\\?\hello",
                Prefix::Verbatim(OsStr::new(r"hello")),
            ))),
            Ok(Component::RootDir(OsStr::new(r"\"))),
        ];

        assert_eq!(&comp[..2], &expected_ok[..]);

        // Check last element is an error
        let result = match &comp[2] {
            Ok(_) => false,
            Err(e) => match e.kind() {
                ParseErrorKind::RestrictedName => true,
                _ => false,
            },
        };

        assert!(result);
    }

    #[test]
    fn relative_path<'path>() {
        let path = br"hello\world";

        let iter = PathIterator::new(path);
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
        let path = br"hello\\world";

        let iter = PathIterator::new(path);
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
        let path = br"hello\world\.\what\now";

        let iter = PathIterator::new(path);
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
        let path = br"hello\world\..\what\now";

        let iter = PathIterator::new(path);
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
        let path = br".\hello\world";

        let iter = PathIterator::new(path);
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
        let path = br"..\hello\world\";

        let iter = PathIterator::new(path);
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
    fn mixed_separator<'path>() {
        let path = br"hello\world/what\now/brown/cow";

        let iter = PathIterator::new(path);
        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 6);

        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::Normal(OsStr::new(r"hello"))),
            Ok(Component::Normal(OsStr::new(r"world"))),
            Ok(Component::Normal(OsStr::new(r"what"))),
            Ok(Component::Normal(OsStr::new(r"now"))),
            Ok(Component::Normal(OsStr::new(r"brown"))),
            Ok(Component::Normal(OsStr::new(r"cow"))),
        ];

        assert_eq!(comp, expected);
    }
}

// ===========================================================================
//
// ===========================================================================
