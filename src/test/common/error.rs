// src/test/common/error.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

// ===========================================================================
// Imports
// ===========================================================================

// --------------------
// Stdlib imports
// --------------------

// --------------------
// Third-party imports
// --------------------

// --------------------
// Local imports
// --------------------

// ===========================================================================
// Tests
// ===========================================================================

mod errorinfo {
    use crate::common::error::{ErrorInfo, ParseErrorKind};

    #[cfg(unix)]
    use crate::unix::UnixErrorKind as ErrorKind;

    #[cfg(windows)]
    use crate::windows::WindowsErrorKind as ErrorKind;

    // --------------------
    // Fixtures
    // --------------------
    fn fixture_errorinfo_vals(
    ) -> (ParseErrorKind, &'static [u8], usize, &'static str) {
        let kind = ErrorKind::InvalidCharacter.into();
        let path = b"/this/is/a/path";
        let end = path.len();
        let errpos = end / 2;
        let msg = "this is a message";
        (kind, path, errpos, msg)
    }

    fn fixture_errorinfo() -> ErrorInfo<'static> {
        let (kind, path, pos, msg) = fixture_errorinfo_vals();
        ErrorInfo::new(kind, path, pos, msg)
    }

    // --------------------
    // Tests
    // --------------------

    mod attr_methods {
        use super::*;

        #[test]
        fn kind_matches_stored_value() {
            let (expected, _, _, _) = fixture_errorinfo_vals();
            let info = fixture_errorinfo();

            let ret = info.kind();
            assert_eq!(ret, &expected)
        }

        #[test]
        fn path_matches_stored_value() {
            let (_, expected, _, _) = fixture_errorinfo_vals();
            let info = fixture_errorinfo();

            let ret = info.path();
            assert_eq!(ret, expected)
        }

        #[test]
        fn pos_matches_stored_value() {
            let (_, _, errpos, _) = fixture_errorinfo_vals();
            let expected = errpos;
            let info = fixture_errorinfo();

            let ret = info.errpos();
            assert_eq!(ret, expected);
        }

        #[test]
        fn msg_matches_stored_value() {
            let (_, _, _, expected) = fixture_errorinfo_vals();
            let info = fixture_errorinfo();

            let ret = info.msg();
            assert_eq!(ret, expected);
        }
    }

    mod with_errmsg {
        use super::*;

        #[test]
        fn create_custom_error_message() {
            let (_, _, _, msg) = fixture_errorinfo_vals();
            let info = fixture_errorinfo();
            let err = info.with_errmsg(|info| {
                format!("Some error occurred: {}", info.msg())
            });

            let expected = format!("Some error occurred: {}", msg);
            let errmsg = err.msg();
            assert_eq!(errmsg, &expected);
        }
    }
}

// ===========================================================================
//
// ===========================================================================
