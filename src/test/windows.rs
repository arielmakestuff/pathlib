// src/test/windows.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

// Local imports

// ===========================================================================
// Test constants
// ===========================================================================

mod constants {
    use std::collections::HashSet;

    use crate::windows;

    #[test]
    fn main_separator() {
        let expected = '\\';
        assert_eq!(windows::SEPARATOR, expected);
    }

    #[test]
    fn alt_separator() {
        let expected = '/';
        assert_eq!(windows::ALT_SEPARATOR, expected);
    }

    #[test]
    fn has_drive() {
        assert_eq!(windows::HAS_DRIVE, true);
    }

    #[test]
    fn drive_letters() {
        let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut expected = HashSet::with_capacity(letters.len());
        for l in letters.chars() {
            expected.insert(l);
        }

        for l in letters.chars() {
            assert!(windows::DRIVE_LETTERS.contains(&l));
        }

        // Derive equivalence by checking lengths are equal, and the 2 sets are
        // subsets of each other. This is needed since PartialEq not
        // implemented
        assert_eq!(windows::DRIVE_LETTERS.len(), expected.len());
        assert!(windows::DRIVE_LETTERS.is_subset(&expected));
        assert!(expected.is_subset(&windows::DRIVE_LETTERS));
    }

    #[test]
    fn reserved_names() {
        let base = ["CON", "PRN", "AUX", "NUL"];
        let numbered_base = ["COM", "LPT"];
        let mut reserved = HashSet::with_capacity(22);
        for b in base.iter() {
            reserved.insert(b.to_string());
        }
        for b in numbered_base.iter() {
            for i in 1..=9 {
                reserved.insert(format!("{}{}", b, i));
            }
        }

        // Derive equivalence by checking lengths are equal, and the 2 sets are
        // subsets of each other. This is needed since PartialEq not implemented
        assert_eq!(windows::RESERVED_NAMES.len(), reserved.len());
        assert_eq!(windows::RESERVED_NAMES.len(), 22);
        assert!(windows::RESERVED_NAMES.is_subset(&reserved));
        assert!(reserved.is_subset(&windows::RESERVED_NAMES));
    }
}

// ===========================================================================
//
// ===========================================================================
