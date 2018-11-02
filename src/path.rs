// src/path.rs
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

// ===========================================================================
// Path
// ===========================================================================

pub struct Path<'path>(&'path StdPath);

impl<'path> Path<'path> {
    pub fn new<P: AsRef<StdPath>>(p: &'path P) -> Path {
        Path(p.as_ref())
    }
}

impl<'path> AsRef<StdPath> for Path<'path> {
    fn as_ref(&self) -> &StdPath {
        self.0
    }
}

// ===========================================================================
//
// ===========================================================================
