// src/test/path.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::path::Path as StdPath;

// Third-party imports

// Local imports
use crate::Path;

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn path_ref() {
    let expected = StdPath::new("/some/path");
    let newpath = Path::new(&expected);
    assert_eq!(newpath.as_ref(), expected);
}

// ===========================================================================
//
// ===========================================================================
