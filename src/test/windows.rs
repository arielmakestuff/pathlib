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
use crate::windows::{Component, PathIterator};

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn test_tmp() {
    // let path = br#"\\?\UNC\server\share"#;
    let path = br#"\\?\UNC"#;
    // let path = br#"."#;
    let iter = PathIterator::new(path);
    // let mut comp = iter.next();
    // println!("WHAT: {:?}", comp);

    // comp = iter.next();
    // println!("WHAT: {:?}", comp);

    // comp = iter.next();
    // println!("WHAT: {:?}", comp);

    // comp = iter.next();
    // println!("WHAT: {:?}", comp);

    // comp = iter.next();
    // println!("WHAT: {:?}", comp);

    // comp = iter.next();
    // println!("WHAT: {:?}", comp);
    let comp: Vec<Component> = iter.collect();
    println!("WHAT: {:?}", comp);
}

// ===========================================================================
//
// ===========================================================================
