// src/test/windows/parser.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports
use combine::Parser;

// Local imports

// ===========================================================================
// Tests
// ===========================================================================

mod sep_byte {
    use super::*;
    use crate::windows::parser::separator;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn matches(sep in prop_oneof!(Just(b"/"), Just(b"\\"))) {
            let parse_result = separator().parse(&sep[..]);
            let result = match parse_result {
                Err(_) => false,
                Ok((cur, rest)) => {
                    cur == sep && rest.is_empty()
                }
            };
            assert!(result);
        }
    }
}

// ===========================================================================
//
// ===========================================================================
