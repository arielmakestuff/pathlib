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
// General tests
// ===========================================================================

#[test]
fn path_ref() {
    let expected = StdPath::new("/some/path");
    let newpath = Path::new(&expected);
    assert_eq!(newpath.as_ref(), expected);
}

// ===========================================================================
// Attr trait tests
// ===========================================================================

mod attr_trait {
    // Stdlib imports
    use std::path::{Component, Path as StdPath};

    // Third-party imports

    // Local imports
    use crate::path::Attr;
    use crate::Path;

    #[test]
    fn same_as_path_components() {
        let pathstr = "/some/path";
        let some_path = Path::new(&pathstr);
        let base_path = StdPath::new(&pathstr);

        let some_parts: Vec<Component> = some_path.components().collect();
        let base_parts: Vec<Component> = base_path.components().collect();

        assert_eq!(some_parts, base_parts);
    }
}

// ===========================================================================
//
// ===========================================================================
