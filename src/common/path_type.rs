// src/common/path_type.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

// Local imports

// ===========================================================================
// Macro used for path component types
// ===========================================================================

#[doc(hidden)]
#[macro_export]
macro_rules! mk_reverse_equal {
    ($type:ty, $reverse_type:ty) => {
        impl PartialEq<$type> for $reverse_type {
            fn eq(&self, other: &$type) -> bool {
                other == self
            }
        }

        impl PartialEq for $type {
            fn eq(&self, _other: &$type) -> bool {
                true
            }
        }

        impl Eq for $type {}
    };
}

// ===========================================================================
// CurrentDir
// ===========================================================================

#[derive(Debug)]
pub struct CurrentDir;

impl CurrentDir {
    pub fn as_str() -> &'static str {
        "."
    }
}

impl PartialEq<&[u8]> for CurrentDir {
    fn eq(&self, other: &&[u8]) -> bool {
        *other == b"."
    }
}

mk_reverse_equal!(CurrentDir, &[u8]);

// ===========================================================================
// ParentDir
// ===========================================================================

#[derive(Debug)]
pub struct ParentDir;

impl ParentDir {
    pub fn as_str() -> &'static str {
        ".."
    }
}

impl PartialEq<&[u8]> for ParentDir {
    fn eq(&self, other: &&[u8]) -> bool {
        *other == b".."
    }
}

mk_reverse_equal!(ParentDir, &[u8]);

#[cfg(test)]
mod test {
    mod currentdir {
        use crate::common::path_type::CurrentDir;

        use proptest::{
            prop_assert, prop_assert_ne, prop_assume, proptest, proptest_helper,
        };

        #[test]
        fn self_equal() {
            assert_eq!(CurrentDir, CurrentDir);
        }

        #[test]
        fn valid_value() {
            let dir = ".";
            assert_eq!(dir.as_bytes(), CurrentDir);
        }

        #[test]
        fn as_str() {
            assert_eq!(CurrentDir::as_str(), ".");
        }

        proptest! {
            #[test]
            fn invalid_value(dir in r#"[^/\\]"#) {
                prop_assume!(dir != ".");

                let dir_bytes = dir.as_bytes();
                prop_assert_ne!(dir_bytes, CurrentDir);
            }
        }
    }

    mod parentdir {
        use crate::common::path_type::ParentDir;

        use proptest::{
            prop_assert, prop_assert_ne, prop_assume, proptest, proptest_helper,
        };

        #[test]
        fn self_equal() {
            assert_eq!(ParentDir, ParentDir);
        }

        #[test]
        fn valid_value() {
            let dir = "..";
            assert_eq!(dir.as_bytes(), ParentDir);
        }

        #[test]
        fn as_str() {
            assert_eq!(ParentDir::as_str(), "..");
        }

        proptest! {
            #[test]
            fn invalid_value(dir in r#"[^/\\]"#) {
                prop_assume!(dir != "..");

                let dir_bytes = dir.as_bytes();
                prop_assert_ne!(dir_bytes, ParentDir);
            }
        }
    }
}

// ===========================================================================
//
// ===========================================================================
