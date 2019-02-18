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
use crate::common::{string::as_str, AsPath, PathData};
use crate::path::Path;
use crate::pathiter_trait_impl;
use crate::{unix, windows};

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

trait PathIterBuilder {
    fn build(&self, path: &'static [u8], index: usize) -> Box<dyn AsRef<Path>>;
}

trait CompBuilder {
    fn build_osstr(&self, path: &'static OsStr) -> Box<dyn AsRef<OsStr>>;
    fn build_path(&self, path: &'static OsStr) -> Box<dyn AsRef<Path>>;
}

struct TestPathIterBuilder;

struct UnixPathIterBuilder;

struct WindowsPathIterBuilder;

impl<'path> PathIterBuilder for TestPathIterBuilder {
    fn build(&self, path: &'static [u8], index: usize) -> Box<dyn AsRef<Path>> {
        build_path_struct!();

        Box::new(TestPath { path, cur: index })
    }
}

impl<'path> PathIterBuilder for UnixPathIterBuilder {
    fn build(&self, path: &'static [u8], index: usize) -> Box<dyn AsRef<Path>> {
        let mut pathiter = unix::PathIterator::new(path);

        // make sure the iterator's internal index matches index
        let mut cur = 0;
        while pathiter.next().is_some() {
            cur = pathiter.current_index();
            if cur == index {
                break;
            }
        }
        assert_eq!(cur, index);

        Box::new(pathiter)
    }
}

impl<'path> PathIterBuilder for WindowsPathIterBuilder {
    fn build(&self, path: &'static [u8], index: usize) -> Box<dyn AsRef<Path>> {
        let mut pathiter = windows::PathIterator::new(path);

        // make sure the iterator's internal index matches index
        let mut cur = 0;
        while pathiter.next().is_some() {
            cur = pathiter.current_index();
            if cur == index {
                break;
            }
        }
        assert_eq!(cur, index);

        Box::new(pathiter)
    }
}

struct TestCompBuilder;

struct UnixCompBuilder;

struct WindowsCompBuilder;

impl<'path> CompBuilder for TestCompBuilder {
    fn build_osstr(&self, path: &'static OsStr) -> Box<dyn AsRef<OsStr>> {
        build_component_struct!();

        Box::new(TestComponent { inner: path })
    }

    fn build_path(&self, path: &'static OsStr) -> Box<dyn AsRef<Path>> {
        build_component_struct!();

        Box::new(TestComponent { inner: path })
    }
}

impl<'path> CompBuilder for UnixCompBuilder {
    fn build_osstr(&self, path: &'static OsStr) -> Box<dyn AsRef<OsStr>> {
        Box::new(unix::Component::Normal(path))
    }

    fn build_path(&self, path: &'static OsStr) -> Box<dyn AsRef<Path>> {
        Box::new(unix::Component::Normal(path))
    }
}

impl<'path> CompBuilder for WindowsCompBuilder {
    fn build_osstr(&self, path: &'static OsStr) -> Box<dyn AsRef<OsStr>> {
        Box::new(windows::Component::Normal(path))
    }

    fn build_path(&self, path: &'static OsStr) -> Box<dyn AsRef<Path>> {
        Box::new(windows::Component::Normal(path))
    }
}

// ===========================================================================
// AsRef<Path> for PathIterator tests
// ===========================================================================

// Make impl_pathiter_asref_path tests
macro_rules! impl_pathiter_asref_path {
    ($testname:ident, $builder:ident) => {
        #[test]
        fn $testname() {
            let path = b"hello/world";

            // index points to the 'w'
            let index = 6;

            let pathobj = $builder.build(path, index);
            let pathobj = pathobj.as_ref();

            let expected = Path::new(OsStr::new(as_str(&path[index..])));
            assert_eq!(pathobj.as_ref(), expected);
        }
    };
}

impl_pathiter_asref_path!(ref_path_asref_path, TestPathIterBuilder);
impl_pathiter_asref_path!(unix_path_asref_path, UnixPathIterBuilder);
impl_pathiter_asref_path!(windows_path_asref_path, WindowsPathIterBuilder);

// ===========================================================================
// AsRef<OsStr> and AsRef<Path> for Component tests
// ===========================================================================

macro_rules! impl_comp_asref {
    ($test_osstr:ident, $test_path:ident, $builder:ident) => {
        #[test]
        fn $test_osstr() {
            let expected = OsStr::new("hello");
            let comp = $builder.build_osstr(&expected);
            let comp = comp.as_ref();

            let ref_val: &OsStr = comp.as_ref();
            assert_eq!(ref_val, expected);
        }

        #[test]
        fn $test_path() {
            let expected = Path::new(OsStr::new("hello"));
            let comp = $builder.build_path(expected.as_ref());
            let comp = comp.as_ref();

            let ref_val: &Path = comp.as_ref();
            assert_eq!(ref_val, expected);
        }
    };
}

impl_comp_asref!(ref_comp_asref_osstr, ref_comp_asref_path, TestCompBuilder);
impl_comp_asref!(unix_comp_asref_osstr, unix_comp_asref_path, UnixCompBuilder);
impl_comp_asref!(
    windows_comp_asref_osstr,
    windows_comp_asref_path,
    WindowsCompBuilder
);

// ===========================================================================
//
// ===========================================================================
