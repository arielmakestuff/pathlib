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
// Globals
// ===========================================================================

#[cfg(unix)]
pub type CodePoint = u8;

#[cfg(windows)]
pub type CodePoint = u16;

// ===========================================================================
// Path Prefix Types: Separator
// ===========================================================================

#[derive(Debug)]
pub struct Separator;

impl PartialEq<CodePoint> for Separator {
    fn eq(&self, other: &CodePoint) -> bool {
        let sep = b'/';

        #[cfg(windows)]
        let sep = sep as CodePoint;

        *other == sep
    }
}

impl PartialEq<Separator> for CodePoint {
    fn eq(&self, other: &Separator) -> bool {
        other == self
    }
}

impl PartialEq<&[CodePoint]> for Separator {
    fn eq(&self, other: &&[CodePoint]) -> bool {
        other.len() == 1 && Separator == other[0]
    }
}

mk_reverse_equal!(Separator, &[CodePoint]);

// ===========================================================================
// Path Prefix Types: Null
// ===========================================================================

#[derive(Debug)]
pub struct Null;

impl PartialEq<CodePoint> for Null {
    fn eq(&self, other: &CodePoint) -> bool {
        let null_char = b'\x00';

        #[cfg(windows)]
        let null_char = null_char as CodePoint;

        *other == null_char
    }
}

impl PartialEq<Null> for CodePoint {
    fn eq(&self, other: &Null) -> bool {
        other == self
    }
}

impl PartialEq<&[CodePoint]> for Null {
    fn eq(&self, other: &&[CodePoint]) -> bool {
        other.len() == 1 && Null == other[0]
    }
}

mk_reverse_equal!(Null, &[CodePoint]);

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod test {
    use super::CodePoint;

    mod separator {
        use super::*;
        use crate::unix::path_type::Separator;

        use proptest::{prop_assert, prop_assert_ne, prop_assume, proptest};

        #[test]
        fn self_equal() {
            assert_eq!(Separator, Separator);
        }

        #[test]
        fn equal_to_sep() {
            let s = vec![b'/' as CodePoint];
            assert_eq!(Separator, &s[..]);
            assert_eq!(&s[..], Separator);

            assert_eq!(Separator, s[0]);
            assert_eq!(s[0], Separator);
        }

        proptest! {
            #[cfg(unix)]
            #[test]
            fn invalid_value(s in r#".*"#) {
                prop_assume!(s.len() != 1 || s != "/");
                let arr: Vec<u8> = s.bytes().collect();
                prop_assert_ne!(Separator, &arr[..]);
            }

            #[cfg(windows)]
            #[test]
            fn invalid_value(s in r#".*"#) {
                prop_assume!(s.len() != 1 || s != "/");
                let arr: Vec<u16> = s.encode_utf16()
                    .collect();
                prop_assert_ne!(Separator, &arr[..]);
            }
        }

    }

    mod null {
        use super::*;
        use crate::unix::path_type::Null;

        use proptest::{prop_assert, prop_assert_ne, prop_assume, proptest};

        #[test]
        fn self_equal() {
            assert_eq!(Null, Null);
        }

        #[test]
        fn equal_to_sep() {
            let s = vec![b'\x00' as CodePoint];
            assert_eq!(Null, &s[..]);
            assert_eq!(&s[..], Null);

            assert_eq!(Null, s[0]);
            assert_eq!(s[0], Null);
        }

        proptest! {
            #[cfg(unix)]
            #[test]
            fn invalid_value(s in r#".*"#) {
                prop_assume!(s.len() != 1 || s != "\x00");
                let arr: Vec<u8> = s.bytes().collect();
                prop_assert_ne!(Null, &arr[..]);
            }

            #[cfg(windows)]
            #[test]
            fn invalid_value(s in r#".*"#) {
                prop_assume!(s.len() != 1 || s != "\x00");
                let arr: Vec<u16> = s.encode_utf16()
                    .collect();
                prop_assert_ne!(Null, &arr[..]);
            }
        }

    }
}

// ===========================================================================
//
// ===========================================================================
