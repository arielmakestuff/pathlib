// src/test/windows.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

#[cfg(feature = "parser-iter")]
mod parser;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::{OsStr, OsString};
use std::path::Prefix;

// Third-party imports

// Local imports
use crate::common::error::*;
use crate::path::{PathIterator, SystemStr};
use crate::windows::{Component, Iter, PrefixComponent};

// ===========================================================================
// Tests
// ===========================================================================

mod windowspathbuf {
    use crate::path::{SystemSeq, SystemString};
    use crate::windows::WindowsPathBuf;

    #[test]
    fn deref_to_systemstring() {
        let path = WindowsPathBuf::new();
        let inner: &SystemString = &path;
        assert_eq!(inner.as_os_str().len(), 0);
    }
}

mod public_export {
    use super::*;
    use crate::common::string::{as_osstr, as_str};

    mod parseerror {
        use super::*;

        #[test]
        fn source_always_none() {
            let err = ParseError::new(
                WindowsErrorKind::RestrictedName.into(),
                OsString::from("hello"),
                OsString::from(as_str(b"/hello/world")),
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

mod iter {
    use super::*;

    #[test]
    fn verbatim_disk<'path>() {
        let path = br"\\?\C:\hello";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<Component<'path>> = vec![
            Component::Prefix(PrefixComponent::new(
                br"\\?\C:\",
                Prefix::VerbatimDisk(b'C'),
            )),
            Component::RootDir(OsStr::new(r"\")),
            Component::Normal(OsStr::new(r"hello")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn prefix_noroot<'path>() {
        let path = br"C:";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 1);

        let expected: Vec<Component<'path>> = vec![Component::Prefix(
            PrefixComponent::new(br"C:", Prefix::Disk(b'C')),
        )];

        assert_eq!(comp, expected);
    }

    #[test]
    fn invalid_char<'path>() {
        let path = br"C:\hello.";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected_ok: Vec<Component<'path>> = vec![
            Component::Prefix(PrefixComponent::new(br"C:", Prefix::Disk(b'C'))),
            Component::RootDir(OsStr::new(r"\")),
        ];

        assert_eq!(&comp[..2], &expected_ok[..]);

        // Check last element is an error
        let result = match &comp[2] {
            Component::Error(info) => {
                let err = ParseError::from(info);
                match err.kind() {
                    ParseErrorKind::Windows(
                        WindowsErrorKind::InvalidCharacter,
                    ) => true,
                    _ => false,
                }
            }
            _ => false,
        };

        assert!(result);
    }

    #[test]
    fn verbatim_path<'path>() {
        let path = br"\\?\hello\world";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<Component<'path>> = vec![
            Component::Prefix(PrefixComponent::new(
                br"\\?\hello",
                Prefix::Verbatim(OsStr::new(r"hello")),
            )),
            Component::RootDir(OsStr::new(r"\")),
            Component::Normal(OsStr::new(r"world")),
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
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();

        // --------------------
        // THEN
        // --------------------
        // the returned value is a vec with 3 elements and
        // the first 2 elements of the vec are prefix and rootdir
        //     components respectively and
        // the last element is an error and
        // the error is a ParseErrorKind::RestrictedName kind
        assert_eq!(comp.len(), 3);

        let expected_ok: Vec<Component<'path>> = vec![
            Component::Prefix(PrefixComponent::new(
                br"\\?\hello",
                Prefix::Verbatim(OsStr::new(r"hello")),
            )),
            Component::RootDir(OsStr::new(r"\")),
        ];

        assert_eq!(&comp[..2], &expected_ok[..]);

        // Check last element is an error
        let result = match &comp[2] {
            Component::Error(info) => {
                let err = ParseError::from(info);
                match err.kind() {
                    ParseErrorKind::Windows(
                        WindowsErrorKind::RestrictedName,
                    ) => true,
                    _ => false,
                }
            }
            _ => false,
        };

        assert!(result);
    }

    #[test]
    fn relative_path<'path>() {
        let path = br"hello\world";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 2);

        let expected: Vec<Component<'path>> = vec![
            Component::Normal(OsStr::new(r"hello")),
            Component::Normal(OsStr::new(r"world")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn double_path_separator<'path>() {
        let path = br"hello\\world";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<Component<'path>> = vec![
            Component::Normal(OsStr::new(r"hello")),
            Component::CurDir,
            Component::Normal(OsStr::new(r"world")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn curdir<'path>() {
        let path = br"hello\world\.\what\now";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 5);

        let expected: Vec<Component<'path>> = vec![
            Component::Normal(OsStr::new(r"hello")),
            Component::Normal(OsStr::new(r"world")),
            Component::CurDir,
            Component::Normal(OsStr::new(r"what")),
            Component::Normal(OsStr::new(r"now")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn parentdir<'path>() {
        let path = br"hello\world\..\what\now";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 5);

        let expected: Vec<Component<'path>> = vec![
            Component::Normal(OsStr::new(r"hello")),
            Component::Normal(OsStr::new(r"world")),
            Component::ParentDir,
            Component::Normal(OsStr::new(r"what")),
            Component::Normal(OsStr::new(r"now")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn curdir_at_start<'path>() {
        let path = br".\hello\world";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<Component<'path>> = vec![
            Component::CurDir,
            Component::Normal(OsStr::new(r"hello")),
            Component::Normal(OsStr::new(r"world")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn parentdir_at_start<'path>() {
        let path = br"..\hello\world\";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<Component<'path>> = vec![
            Component::ParentDir,
            Component::Normal(OsStr::new(r"hello")),
            Component::Normal(OsStr::new(r"world")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn mixed_separator<'path>() {
        let path = br"hello\world/what\now/brown/cow";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 6);

        let expected: Vec<Component<'path>> = vec![
            Component::Normal(OsStr::new(r"hello")),
            Component::Normal(OsStr::new(r"world")),
            Component::Normal(OsStr::new(r"what")),
            Component::Normal(OsStr::new(r"now")),
            Component::Normal(OsStr::new(r"brown")),
            Component::Normal(OsStr::new(r"cow")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn empty_path<'path>() {
        let path = b"";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        let expected: Vec<Component<'path>> = vec![Component::CurDir];

        assert_eq!(comp, expected);
    }
}

// ===========================================================================
//
// ===========================================================================
