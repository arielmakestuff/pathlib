// src/unix/path_type.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

// Local imports
use crate::mk_reverse_equal;

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
}

// ===========================================================================
//
// ===========================================================================
