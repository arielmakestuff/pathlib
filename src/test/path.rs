// src/test/path.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

// Local imports
use crate::path::{Path, PlatformPath, PlatformPathBuf};

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn convert_path_to_vec_with_into() {
    // --------------------
    // GIVEN
    // --------------------
    // a PlatformPath instance

    let pathstr = b"/hello/world";
    let path = PlatformPath::from_bytes(pathstr);

    // --------------------
    // WHEN
    // --------------------
    // Converting the PlatformPath to a Vec<u8>
    let bytes: Vec<u8> = path.into();

    // --------------------
    // THEN
    // --------------------
    // a Vec<u8> containing code points representing the path is returned
    let expected = pathstr.to_vec();
    assert_eq!(bytes, expected);
}

#[test]
fn convert_path_to_vec_with_from() {
    // --------------------
    // GIVEN
    // --------------------
    // a PlatformPath instance

    let pathstr = b"/hello/world";
    let path = PlatformPath::from_bytes(pathstr);

    // --------------------
    // WHEN
    // --------------------
    // Converting the PlatformPath to a Vec<u8>
    let bytes: Vec<u8> = Vec::from(path);

    // --------------------
    // THEN
    // --------------------
    // a Vec<u8> containing code points representing the path is returned
    let expected: Vec<u8> = pathstr.to_vec();
    assert_eq!(bytes, expected);
}

#[test]
fn convert_pathbuf_to_vec_with_into() {
    // --------------------
    // GIVEN
    // --------------------
    // a PlatformPath instance

    let pathstr = b"/hello/world";
    let path = PlatformPathBuf::from_bytes(pathstr);

    // --------------------
    // WHEN
    // --------------------
    // Converting the PlatformPath to a Vec<u8>
    let bytes: Vec<u8> = path.into();

    // --------------------
    // THEN
    // --------------------
    // a Vec<u8> containing code points representing the path is returned
    let expected = pathstr.to_vec();
    assert_eq!(bytes, expected);
}

#[test]
fn convert_pathbuf_to_vec_with_from() {
    // --------------------
    // GIVEN
    // --------------------
    // a PlatformPath instance

    let pathstr = b"/hello/world";
    let path = PlatformPathBuf::from_bytes(pathstr);

    // --------------------
    // WHEN
    // --------------------
    // Converting the PlatformPath to a Vec<u8>
    let bytes: Vec<u8> = Vec::from(path);

    // --------------------
    // THEN
    // --------------------
    // a Vec<u8> containing code points representing the path is returned
    let expected: Vec<u8> = pathstr.to_vec();
    assert_eq!(bytes, expected);
}

#[test]
fn empty_pathbuf() {
    // --------------------
    // WHEN
    // --------------------
    // Creating a new empty PlatformPathBuf
    let path = PlatformPathBuf::new();

    // --------------------
    // THEN
    // --------------------
    // the PlatformPathBuf has a length of zero
    assert_eq!(path.as_os_str().len(), 0);
}

// ===========================================================================
//
// ===========================================================================
