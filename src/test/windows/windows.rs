// src/test/windows.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::{OsStr, OsString};
use std::path::Prefix;

// Third-party imports

// Local imports
use crate::common::error::*;
use crate::path::PathBuf;
use crate::windows::{Component, Iter, PathComponent};

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
            let pathbuf = PathBuf::from_bytes(path);
            let iter = Iter::new(pathbuf.as_ref());
            let mut compvec: Vec<PathComponent> = iter.collect();
            let prefix = compvec.remove(0).expect("Not a component");

            // prefix is the Component::Prefix variant
            assert_eq!(prefix.as_os_str(), as_osstr(br"\\?\hello"));

            // Make sure prefix contains the correct data
            let prefix_comp = match prefix {
                Component::Prefix(pc) => pc,
                _ => unreachable!()
            };
            let expected = Prefix::Verbatim(OsStr::new("hello"));
            assert_eq!(prefix_comp.kind(), expected);
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
}

mod iter {
    use super::*;

    #[test]
    fn verbatim_disk<'path>() {
        let path = br"\\?\C:\hello";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<PathComponent<'path>> = vec![
            // Ok(Component::Prefix(PrefixComponent::new(
            //     br"\\?\C:\",
            //     Prefix::VerbatimDisk(b'C'),
            // ))),
            Ok(Component::RootDir(OsStr::new(r"\"))),
            Ok(Component::Normal(OsStr::new(r"hello"))),
        ];

        assert_eq!(&comp[1..], &expected[..]);

        let prefix_comp = match comp[0] {
            Ok(Component::Prefix(pc)) => pc,
            _ => unreachable!()
        };

        // Make sure prefix comp contains expected data
        assert_eq!(prefix_comp.kind(), Prefix::VerbatimDisk(b'C'));
        assert_eq!(prefix_comp.as_os_str(), OsStr::new(r"\\?\C:"));
    }

    #[test]
    fn prefix_noroot<'path>() {
        let path = br"C:";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 1);

        let prefix_comp = match comp[0] {
            Ok(Component::Prefix(pc)) => pc,
            _ => unreachable!()
        };

        assert_eq!(prefix_comp.kind(), Prefix::Disk(b'C'));
        assert_eq!(prefix_comp.as_os_str(), OsStr::new("C:"));
    }

    #[test]
    fn invalid_char<'path>() {
        let path = br"C:\hello.";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected_ok: Vec<PathComponent<'path>> = vec![
            // Ok(Component::Prefix(PrefixComponent::new(
            //     br"C:",
            //     Prefix::Disk(b'C'),
            // ))),
            Ok(Component::RootDir(OsStr::new(r"\"))),
        ];

        assert_eq!(&comp[1..2], &expected_ok[..]);

        // Check prefix component is as expected
        let prefix_comp = match comp[0] {
            Ok(Component::Prefix(pc)) => pc,
            _ => unreachable!()
        };
        assert_eq!(prefix_comp.kind(), Prefix::Disk(b'C'));
        assert_eq!(prefix_comp.as_os_str(), OsStr::new("C:"));

        // Check last element is an error
        let result = match &comp[2] {
            Ok(_) => false,
            Err(e) => match e.kind() {
                ParseErrorKind::Windows(WindowsErrorKind::InvalidCharacter) => {
                    true
                }
                _ => false,
            },
        };

        assert!(result);
    }

    #[test]
    fn verbatim_path<'path>() {
        let path = br"\\?\hello\world";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<PathComponent<'path>> = vec![
            // Ok(Component::Prefix(PrefixComponent::new(
            //     br"\\?\hello",
            //     Prefix::Verbatim(OsStr::new(r"hello")),
            // ))),
            Ok(Component::RootDir(OsStr::new(r"\"))),
            Ok(Component::Normal(OsStr::new(r"world"))),
        ];

        assert_eq!(&comp[1..], &expected[..]);

        // Check prefix comp is as expected
        let prefix_comp = match comp[0] {
            Ok(Component::Prefix(pc)) => pc,
            _ => unreachable!()
        };
        assert_eq!(prefix_comp.kind(), Prefix::Verbatim(OsStr::new("hello")));
        assert_eq!(prefix_comp.as_os_str(), r"\\?\hello");
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
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

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
            // Ok(Component::Prefix(PrefixComponent::new(
            //     br"\\?\hello",
            //     Prefix::Verbatim(OsStr::new(r"hello")),
            // ))),
            Ok(Component::RootDir(OsStr::new(r"\"))),
        ];

        assert_eq!(&comp[1..2], &expected_ok[..]);

        // Check prefix component is as expected
        let prefix_comp = match comp[0] {
            Ok(Component::Prefix(pc)) => pc,
            _ => unreachable!()
        };
        assert_eq!(prefix_comp.kind(), Prefix::Verbatim(OsStr::new("hello")));
        assert_eq!(prefix_comp.as_os_str(), OsStr::new(r"\\?\hello"));

        // Check last element is an error
        let result = match &comp[2] {
            Ok(_) => false,
            Err(e) => match e.kind() {
                ParseErrorKind::Windows(WindowsErrorKind::RestrictedName) => {
                    true
                }
                _ => false,
            },
        };

        assert!(result);
    }

    #[test]
    fn relative_path<'path>() {
        let path = br"hello\world";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

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
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 2);

        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::Normal(OsStr::new(r"hello"))),
            Ok(Component::Normal(OsStr::new(r"world"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn curdir<'path>() {
        let path = br"hello\world\.\what\now";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        assert_eq!(comp.len(), 4);

        let expected: Vec<PathComponent<'path>> = vec![
            Ok(Component::Normal(OsStr::new(r"hello"))),
            Ok(Component::Normal(OsStr::new(r"world"))),
            Ok(Component::Normal(OsStr::new(r"what"))),
            Ok(Component::Normal(OsStr::new(r"now"))),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn parentdir<'path>() {
        let path = br"hello\world\..\what\now";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

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
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

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
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

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
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

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

    #[test]
    fn empty_path<'path>() {
        let path = b"";
        let pathbuf = PathBuf::from_bytes(path);
        let iter = Iter::new(pathbuf.as_ref());

        let comp: Vec<PathComponent> = iter.collect();
        let expected: Vec<PathComponent<'path>> = Vec::new();

        assert_eq!(comp, expected);
    }
}

// ===========================================================================
//
// ===========================================================================
