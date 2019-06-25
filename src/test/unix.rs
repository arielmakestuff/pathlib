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
use crate::path::{PathIterator, SystemStr};
use crate::unix::{Component, Iter};

// ===========================================================================
// Tests
// ===========================================================================

mod unixpathbuf {
    use crate::path::{SystemSeq, SystemString};
    use crate::unix::UnixPathBuf;

    #[test]
    fn deref_to_systemstring() {
        let path = UnixPathBuf::new();
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
                UnixErrorKind::InvalidCharacter.into(),
                OsString::from(as_str(b"/hello/world")),
                1,
                String::from("message"),
            );

            assert!(err.source().is_none());
        }

        #[test]
        fn kind_value() {
            let err = ParseError::new(
                UnixErrorKind::InvalidCharacter.into(),
                OsString::from(as_str(b"/hello/world")),
                1,
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
    fn prefix_noroot() {
        let path = b"hello";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 1);

        let expected: Vec<Component<'_>> =
            vec![Component::Normal(OsStr::new("hello"))];

        assert_eq!(comp, expected);
    }

    #[test]
    fn invalid_char() {
        let path = b"/hello\x00/world";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();

        assert_eq!(comp.len(), 2);

        let expected_ok: Vec<Component<'_>> = vec![Component::RootDir];

        assert_eq!(&comp[..1], &expected_ok[..]);

        // Check last element is an error
        let result = match &comp[1] {
            Component::Error(info) => {
                let err = ParseError::from(info);
                match err.kind() {
                    ParseErrorKind::Unix(UnixErrorKind::InvalidCharacter) => {
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        };

        assert!(result);
    }

    #[test]
    fn relative_path() {
        let path = b"hello/world";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 2);

        let expected: Vec<Component<'_>> = vec![
            Component::Normal(OsStr::new(r"hello")),
            Component::Normal(OsStr::new(r"world")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn double_path_separator() {
        let path = br"hello//world";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<Component<'_>> = vec![
            Component::Normal(OsStr::new(r"hello")),
            Component::CurDir,
            Component::Normal(OsStr::new(r"world")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn curdir() {
        let path = br"hello/world/./what/now";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 5);

        let expected: Vec<Component<'_>> = vec![
            Component::Normal(OsStr::new(r"hello")),
            Component::Normal(OsStr::new(r"world")),
            Component::CurDir,
            Component::Normal(OsStr::new(r"what")),
            Component::Normal(OsStr::new(r"now")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn parentdir() {
        let path = br"hello/world/../what/now";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 5);

        let expected: Vec<Component<'_>> = vec![
            Component::Normal(OsStr::new(r"hello")),
            Component::Normal(OsStr::new(r"world")),
            Component::ParentDir,
            Component::Normal(OsStr::new(r"what")),
            Component::Normal(OsStr::new(r"now")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn curdir_at_start() {
        let path = br"./hello/world";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<Component<'_>> = vec![
            Component::CurDir,
            Component::Normal(OsStr::new(r"hello")),
            Component::Normal(OsStr::new(r"world")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn parentdir_at_start() {
        let path = br"../hello/world/";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<Component<'_>> = vec![
            Component::ParentDir,
            Component::Normal(OsStr::new(r"hello")),
            Component::Normal(OsStr::new(r"world")),
        ];

        assert_eq!(comp, expected);
    }

    #[test]
    fn absolute_path() {
        let path = b"/hello/world/what/now/brown/cow";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 7);

        let expected: Vec<Component<'_>> = vec![
            Component::RootDir,
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
    fn empty_path() {
        let path = b"";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        let expected: Vec<Component<'_>> = vec![Component::CurDir];

        assert_eq!(comp, expected);
    }

    #[test]
    fn multibyte_chars() {
        let s = "/multibyte/Löwe 老虎 Léopard";
        let path = s.as_bytes();
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        let expected: Vec<Component<'_>> = vec![
            Component::RootDir,
            Component::Normal(OsStr::new("multibyte")),
            Component::Normal(OsStr::new("Löwe 老虎 Léopard")),
        ];

        assert_eq!(comp, expected);
    }
}

mod error {

    // --------------------
    // Stdlib imports
    // --------------------

    // --------------------
    // Third-party imports
    // --------------------
    use proptest::prelude::*;

    // --------------------
    // Local imports
    // --------------------
    use crate::common::string::as_str;
    use crate::prelude::*;
    use crate::unix::Component as UnixComponent;

    // --------------------
    // Tests
    // --------------------
    fn good_byte() -> impl Strategy<Value = u8> {
        prop::num::u8::ANY
            .prop_filter("Cannot be null or '/'", |v| *v != 0 && *v != b'/')
    }

    fn good_component() -> impl Strategy<Value = Vec<u8>> {
        prop::collection::vec(good_byte(), 1..10)
    }

    fn bad_component() -> impl Strategy<Value = Vec<u8>> {
        good_component().prop_perturb(|mut v, mut rng| {
            let insert_index: usize = rng.gen_range(0, v.len());
            v.insert(insert_index, 0);
            v
        })
    }

    fn bad_path() -> impl Strategy<Value = Vec<Vec<u8>>> {
        let comp = prop_oneof!(good_component(), bad_component());
        let head = prop::collection::vec(comp, 0..10);

        (head, bad_component()).prop_perturb(|(mut head, tail), mut rng| {
            if head.is_empty() {
                head.push(tail);
            } else {
                let insert_index: usize = rng.gen_range(0, head.len());
                head.insert(insert_index, tail);
            }
            head
        })
    }

    proptest! {
        #[test]
        fn badpath_raises_error(path in bad_path()) {
            let path_bytes = path.join(&b'/');
            let first_zero = path_bytes
                .iter()
                .enumerate()
                .fold(None, |ret, (i, el)| {
                    if *el == 0 {
                        match ret {
                            Some(_) => ret,
                            None => Some(i)
                        }
                    } else {
                        ret
                    }
                }).expect("null byte not found");

           let path_str = as_str(&path_bytes[..]);
           prop_assert!(!path_str.is_empty());

           let path = UnixPath::new(path_str);
           let comp: Vec<UnixComponent> = path.iter().collect();
           prop_assert!(!comp.is_empty());

           let err = comp.last().unwrap();
           let result = match err {
               UnixComponent::Error(err) => {
                   err.errpos() == first_zero
               }
               _ => false

           };
           prop_assert!(result);
        }
    }
}

// ===========================================================================
//
// ===========================================================================
