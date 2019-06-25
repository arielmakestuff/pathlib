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
                OsString::from(as_str(b"/hello/world")),
                1,
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
    fn verbatim_disk() {
        let path = br"\\?\C:\hello";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<Component<'_>> = vec![
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
    fn prefix_noroot() {
        let path = br"C:";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 1);

        let expected: Vec<Component<'_>> = vec![Component::Prefix(
            PrefixComponent::new(br"C:", Prefix::Disk(b'C')),
        )];

        assert_eq!(comp, expected);
    }

    #[test]
    fn invalid_char() {
        let path = br"C:\hello.";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected_ok: Vec<Component<'_>> = vec![
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
    fn verbatim_path() {
        let path = br"\\?\hello\world";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 3);

        let expected: Vec<Component<'_>> = vec![
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
    fn invalid_filename() {
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

        let expected_ok: Vec<Component<'_>> = vec![
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
    fn relative_path() {
        let path = br"hello\world";
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
        let path = br"hello\\world";
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
        let path = br"hello\world\.\what\now";
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
        let path = br"hello\world\..\what\now";
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
        let path = br".\hello\world";
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
        let path = br"..\hello\world\";
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
    fn mixed_separator() {
        let path = br"hello\world/what\now/brown/cow";
        let iter = Iter::new(SystemStr::from_bytes(path));

        let comp: Vec<Component> = iter.collect();
        assert_eq!(comp.len(), 6);

        let expected: Vec<Component<'_>> = vec![
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
}

mod error {

    // --------------------
    // Stdlib imports
    // --------------------
    use std::collections::HashSet;

    // --------------------
    // Third-party imports
    // --------------------
    use lazy_static::lazy_static;
    use proptest::prelude::*;

    // --------------------
    // Local imports
    // --------------------
    use crate::common::string::as_str;
    use crate::prelude::*;
    use crate::windows::Component as WindowsComponent;

    // --------------------
    // Globals
    // --------------------
    lazy_static! {
        static ref RESTRICTED_CHARS: HashSet<u8> = {
            let chars = r#"<>:"/\|?*"#;
            let mut all_chars = HashSet::with_capacity(chars.len());
            for c in chars.chars() {
                all_chars.insert(c as u8);
            }

            // These are ascii chars code 0 - 31
            for i in 0..=31 {
                all_chars.insert(i);
            }
            all_chars
        };
    }

    // --------------------
    // Helpers
    // --------------------
    fn good_byte() -> impl Strategy<Value = u8> {
        prop::num::u8::ANY.prop_filter("Cannot contain restricted byte", |v| {
            !RESTRICTED_CHARS.contains(v)
        })
    }

    fn bad_byte() -> impl Strategy<Value = u8> {
        let bad_vals: Vec<u8> = RESTRICTED_CHARS
            .iter()
            .cloned()
            .filter(|el| *el != b'/' && *el != b'\\')
            .collect();
        prop::sample::select(bad_vals)
    }

    fn good_component() -> impl Strategy<Value = Vec<u8>> {
        prop::collection::vec(good_byte(), 0..10)
    }

    fn bad_component() -> impl Strategy<Value = Vec<u8>> {
        (good_component(), bad_byte()).prop_perturb(|(mut v, bad), mut rng| {
            if v.is_empty() {
                v.push(bad);
            } else {
                let insert_index = rng.gen_range(0, v.len());
                v.insert(insert_index, bad);
            }
            v
        })
    }

    fn bad_path() -> impl Strategy<Value = Vec<Vec<u8>>> {
        let comp = prop_oneof!(good_component(), bad_component());
        let body = prop::collection::vec(comp, 0..10);

        (good_component(), body, bad_component()).prop_perturb(
            |(head, mut body, tail), mut rng| {
                if body.is_empty() {
                    body.push(head);
                    body.push(tail);
                } else {
                    body.insert(0, head);

                    let insert_index: usize = rng.gen_range(0, body.len());
                    body.insert(insert_index, tail);
                }

                // Make sure that windows prefixes are not generated (since they
                // are valid but get falsely detected as invalid by the test) by
                // adding the "42" byte string at the beginning of the path
                for comp in body.iter_mut() {
                    if !comp.is_empty() {
                        let first = comp[0];
                        let is_disk = comp.len() >= 2
                            && first.is_ascii_alphabetic()
                            && comp[1] == b':';
                        let is_sep = first == b'/' || first == b'\\';
                        if is_disk || is_sep {
                            comp.splice(..0, b"42".iter().cloned());
                        }
                        break;
                    }
                }

                body
            },
        )
    }

    fn find_first_bad_byte(path: &[Vec<u8>]) -> usize {
        let mut i = 0usize;
        for comp in path.iter() {
            let part = &comp[..];
            if part.is_empty() {
                i += 1;
                continue;
            } else if part == b"." || part == b".." {
                i += part.len() + 1;
                continue;
            }

            let mut index = None;
            for (i, el) in comp.iter().enumerate() {
                match index {
                    None if RESTRICTED_CHARS.contains(el) => {
                        index = Some(i);
                        break;
                    }
                    _ => {}
                }
            }
            match index {
                None => {
                    if let Some(c) = part.last() {
                        match c {
                            b' ' | b'.' => {
                                i += part.len() - 1;
                                break;
                            }
                            _ => {
                                i += comp.len() + 1;
                            }
                        }
                    }
                }
                Some(err_pos) => {
                    i += err_pos;
                    break;
                }
            }
        }
        i
    }

    // --------------------
    // Tests
    // --------------------
    proptest! {
        #[test]
        fn badpath_raises_error(path in bad_path()) {
            let mut sep: HashSet<u8> = HashSet::with_capacity(2);
            sep.extend(b"/\\");

            let first_bad_byte = find_first_bad_byte(&path);
            let path_bytes = path.join(&b'/');

            let path_str = as_str(&path_bytes[..]);
            prop_assert!(!path_str.is_empty());

            let path = WindowsPath::new(path_str);
            let comp: Vec<WindowsComponent> = path.iter().collect();
            prop_assert!(!comp.is_empty());

            let result = match comp.last() {
                Some(WindowsComponent::Error(err)) => {
                    err.errpos() == first_bad_byte
                }
                _ => false,
            };
            prop_assert!(result);
        }
    }
}

// ===========================================================================
//
// ===========================================================================
