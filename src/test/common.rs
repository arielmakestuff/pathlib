// src/test/common.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsStr;

// Third-party imports

// Local imports
use crate::common::{AsPath, PathData};
use crate::path::Path;
use crate::pathiter_trait_impl;

// ===========================================================================
// Setup
// ===========================================================================

macro_rules! build_path_struct {
    () => {
        struct TestPath<'path> {
            path: &'path [u8],
            cur: usize,
        }

        pathiter_trait_impl!(TestPath, 'path);
    }
}

macro_rules! build_component_struct {
    () => {
        struct TestComponent<'path> {
            inner: &'path OsStr
        }

        impl<'path> TestComponent<'path> {
            fn as_os_str(&self) -> &OsStr {
                self.inner
            }
        }

        component_asref_impl!(TestComponent, 'path);
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn impl_path_asref_path() {
    build_path_struct!();

    // current index points to the 'w'
    let path = TestPath {
        path: b"hello/world",
        cur: 6,
    };

    let expected = Path::new(OsStr::new("world"));
    assert_eq!(path.as_ref(), expected);
}

#[test]
fn impl_comp_asref_osstr() {
    build_component_struct!();

    let expected = OsStr::new("hello");
    let comp = TestComponent {
        inner: OsStr::new("hello"),
    };

    let ref_val: &OsStr = comp.as_ref();
    assert_eq!(ref_val, expected);
}

#[test]
fn impl_comp_asref_path() {
    build_component_struct!();

    let expected = Path::new(OsStr::new("hello"));
    let comp = TestComponent {
        inner: OsStr::new("hello"),
    };

    let ref_val: &Path = comp.as_ref();
    assert_eq!(ref_val, expected);
}

// ===========================================================================
//
// ===========================================================================
