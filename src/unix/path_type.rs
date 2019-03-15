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
// PlatformPath Prefix Types: Separator
// ===========================================================================

#[derive(Debug)]
pub struct Separator;

impl PartialEq<u8> for Separator {
    fn eq(&self, other: &u8) -> bool {
        let sep = b'/';
        *other == sep
    }
}

impl PartialEq<Separator> for u8 {
    fn eq(&self, other: &Separator) -> bool {
        other == self
    }
}

impl PartialEq<&[u8]> for Separator {
    fn eq(&self, other: &&[u8]) -> bool {
        other.len() == 1 && Separator == other[0]
    }
}

mk_reverse_equal!(Separator, &[u8]);

// ===========================================================================
// PlatformPath Prefix Types: Null
// ===========================================================================

#[derive(Debug)]
pub struct Null;

impl PartialEq<u8> for Null {
    fn eq(&self, other: &u8) -> bool {
        let null_char = b'\x00';
        *other == null_char
    }
}

impl PartialEq<Null> for u8 {
    fn eq(&self, other: &Null) -> bool {
        other == self
    }
}

impl PartialEq<&[u8]> for Null {
    fn eq(&self, other: &&[u8]) -> bool {
        other.len() == 1 && Null == other[0]
    }
}

mk_reverse_equal!(Null, &[u8]);

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod test {
    mod separator {
        use crate::unix::path_type::Separator;

        use proptest::{prop_assert, prop_assert_ne, prop_assume, proptest};

        #[test]
        fn self_equal() {
            assert_eq!(Separator, Separator);
        }

        #[test]
        fn equal_to_sep() {
            let s = vec![b'/'];
            assert_eq!(Separator, &s[..]);
            assert_eq!(&s[..], Separator);

            assert_eq!(Separator, s[0]);
            assert_eq!(s[0], Separator);
        }

        proptest! {
            #[test]
            fn invalid_value(s in r#".*"#) {
                prop_assume!(s.len() != 1 || s != "/");
                let arr: Vec<u8> = s.bytes().collect();
                prop_assert_ne!(Separator, &arr[..]);
            }
        }

    }

    mod null {
        use crate::unix::path_type::Null;

        use proptest::{prop_assert, prop_assert_ne, prop_assume, proptest};

        #[test]
        fn self_equal() {
            assert_eq!(Null, Null);
        }

        #[test]
        fn equal_to_sep() {
            let s = vec![b'\x00'];
            assert_eq!(Null, &s[..]);
            assert_eq!(&s[..], Null);

            assert_eq!(Null, s[0]);
            assert_eq!(s[0], Null);
        }

        proptest! {
            #[test]
            fn invalid_value(s in r#".*"#) {
                prop_assume!(s.len() != 1 || s != "\x00");
                let arr: Vec<u8> = s.bytes().collect();
                prop_assert_ne!(Null, &arr[..]);
            }
        }

    }
}

// ===========================================================================
//
// ===========================================================================
