// tests/memorypath.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsString;

// Third-party imports

// Local imports
use pathlib::prelude::*;

// ===========================================================================
// Tests
// ===========================================================================

mod parts {
    use super::*;

    #[test]
    fn unixpath() {
        let path = UnixPath::new("/a/unix/path");
        let expected: Vec<OsString> = ["/", "a", "unix", "path"]
            .iter()
            .map(OsString::from)
            .collect();

        let parts: Vec<OsString> = path.parts().collect();

        assert_eq!(parts, expected);
    }

    #[test]
    fn windowspath_prefix_root() {
        let path = WindowsPath::new(r"C:\a\windows\path");
        let expected: Vec<OsString> = [r"C:\", "a", "windows", "path"]
            .iter()
            .map(OsString::from)
            .collect();

        let parts: Vec<OsString> = path.parts().collect();

        assert_eq!(parts, expected);
    }

    #[test]
    fn windowspath_prefix_noroot() {
        let path = WindowsPath::new(r"C:a\windows\path");
        let expected: Vec<OsString> = ["C:", "a", "windows", "path"]
            .iter()
            .map(OsString::from)
            .collect();

        let parts: Vec<OsString> = path.parts().collect();

        assert_eq!(parts, expected);
    }

    #[test]
    fn windowspath_noprefix() {
        let path = WindowsPath::new(r"\a\windows\path");
        let expected: Vec<OsString> = [r"\", "a", "windows", "path"]
            .iter()
            .map(OsString::from)
            .collect();

        let parts: Vec<OsString> = path.parts().collect();

        assert_eq!(parts, expected);
    }

    #[test]
    fn windowspath_prefix_only() {
        let path = WindowsPath::new("C:");
        let expected = vec![OsString::from("C:")];

        let parts: Vec<OsString> = path.parts().collect();

        assert_eq!(parts, expected);
    }
}

// ===========================================================================
//
// ===========================================================================
