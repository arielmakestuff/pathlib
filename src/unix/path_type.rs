// src/unix/path_type.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::collections::HashSet;

// Third-party imports
use lazy_static::lazy_static;

// Local imports
use crate::mk_reverse_equal;

// ===========================================================================
// Globals
// ===========================================================================

lazy_static! {
    static ref INVALID_CHAR: HashSet<u8> = {
        let chars = b"/\x00";
        let all_chars: HashSet<u8> = chars.iter().map(|&c| c).collect();
        all_chars
    };
}
// ===========================================================================
// Path Prefix Types: Separator
// ===========================================================================

#[derive(Debug)]
pub struct Separator;

impl PartialEq<u8> for Separator {
    fn eq(&self, other: &u8) -> bool {
        return *other == b'/';
    }
}

impl PartialEq<Separator> for u8 {
    fn eq(&self, other: &Separator) -> bool {
        other == self
    }
}

impl PartialEq<&[u8]> for Separator {
    fn eq(&self, other: &&[u8]) -> bool {
        return other == b"/";
    }
}

mk_reverse_equal!(Separator, &[u8]);

// ===========================================================================
// Path Prefix Types: Part
// ===========================================================================

#[derive(Debug)]
pub struct Part;

impl PartialEq<&[u8]> for Part {
    fn eq(&self, other: &&[u8]) -> bool {
        return !other.iter().any(|b| INVALID_CHAR.contains(b));
    }
}

mk_reverse_equal!(Part, &[u8]);

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod test {
    mod separator {
        use crate::unix::path_type::Separator;

        use proptest::{
            prop_assert, prop_assert_ne, prop_assume, proptest, proptest_helper,
        };

        #[test]
        fn self_equal() {
            assert_eq!(Separator, Separator);
        }

        #[test]
        fn equal_to_sep() {
            let s = b"/";
            assert_eq!(Separator, &s[..]);
            assert_eq!(&s[..], Separator);

            assert_eq!(Separator, s[0]);
            assert_eq!(s[0], Separator);
        }

        proptest! {
            #[test]
            fn invalid_value(s in r#".*"#) {
                prop_assume!(s.len() != 1 || s != "/");
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assert_ne!(Separator, &arr[..]);
            }
        }

    }

    mod part {
        use crate::unix::path_type::Part;

        use proptest::{
            prop_assert, prop_assert_ne, proptest, proptest_helper,
        };

        #[test]
        fn self_equal() {
            assert_eq!(Part, Part);
        }

        proptest! {
            #[test]
            fn valid_value(s in r#"[^/\x00]*"#) {
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                assert_eq!(Part, &arr[..]);
            }

            #[test]
            fn invalid_value(s in r#"(.*[/\x00]+.*)+"#) {
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assert_ne!(Part, &arr[..]);
            }
        }
    }
}

// ===========================================================================
//
// ===========================================================================
