// src/test/unix/path.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

// Local imports
use crate::path::{Path, PathBuf};

// ===========================================================================
// Globals
// ===========================================================================

#[cfg(unix)]
type CodePoint = u8;

#[cfg(windows)]
type CodePoint = u16;

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn convert_path_to_vec_with_into() {
    // --------------------
    // GIVEN
    // --------------------
    // a Path instance

    let pathstr = b"/hello/world";
    let path = Path::from_bytes(pathstr);

    // --------------------
    // WHEN
    // --------------------
    // Converting the Path to a Vec<CodePoint>
    let bytes: Vec<CodePoint> = path.into();

    // --------------------
    // THEN
    // --------------------
    // a Vec<CodePoint> containing code points representing the path is returned
    let expected = pathstr.to_vec();
    assert_eq!(bytes, expected);
}

#[test]
fn convert_path_to_vec_with_from() {
    // --------------------
    // GIVEN
    // --------------------
    // a Path instance

    let pathstr = b"/hello/world";
    let path = Path::from_bytes(pathstr);

    // --------------------
    // WHEN
    // --------------------
    // Converting the Path to a Vec<CodePoint>
    let bytes = Vec::from(path);

    // --------------------
    // THEN
    // --------------------
    // a Vec<CodePoint> containing code points representing the path is returned
    let expected: Vec<CodePoint> = pathstr.to_vec();
    assert_eq!(bytes, expected);
}

#[test]
fn convert_pathbuf_to_vec_with_into() {
    // --------------------
    // GIVEN
    // --------------------
    // a Path instance

    let pathstr = b"/hello/world";
    let path = PathBuf::from_bytes(pathstr);

    // --------------------
    // WHEN
    // --------------------
    // Converting the Path to a Vec<CodePoint>
    let bytes: Vec<CodePoint> = path.into();

    // --------------------
    // THEN
    // --------------------
    // a Vec<CodePoint> containing code points representing the path is returned
    let expected = pathstr.to_vec();
    assert_eq!(bytes, expected);
}

#[test]
fn convert_pathbuf_to_vec_with_from() {
    // --------------------
    // GIVEN
    // --------------------
    // a Path instance

    let pathstr = b"/hello/world";
    let path = PathBuf::from_bytes(pathstr);

    // --------------------
    // WHEN
    // --------------------
    // Converting the Path to a Vec<CodePoint>
    let bytes = Vec::from(path);

    // --------------------
    // THEN
    // --------------------
    // a Vec<CodePoint> containing code points representing the path is returned
    let expected: Vec<CodePoint> = pathstr.to_vec();
    assert_eq!(bytes, expected);
}

#[test]
fn empty_pathbuf() {
    // --------------------
    // WHEN
    // --------------------
    // Creating a new empty PathBuf
    let path = PathBuf::new();

    // --------------------
    // THEN
    // --------------------
    // the PathBuf has a length of zero
    assert_eq!(path.as_os_str().len(), 0);
}

// ===========================================================================
//
// ===========================================================================
