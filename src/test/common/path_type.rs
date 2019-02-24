// src/test/common/path_type.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

// Local imports
use crate::common::path_type::{CurrentDir, ParentDir};

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn convert_parentdir_to_str() {
    let expected = "..";
    let result: &str = ParentDir.as_ref();
    assert_eq!(result, expected);
}

#[test]
fn convert_currentdir_to_str() {
    let expected = ".";
    let result: &str = CurrentDir.as_ref();
    assert_eq!(result, expected);
}

// ===========================================================================
//
// ===========================================================================
