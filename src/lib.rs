// src/lib.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

#[macro_use]
mod common;

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod test;

#[macro_use]
pub mod path;
pub mod unix;
pub mod windows;

// ===========================================================================
// Externs
// ===========================================================================

// Stdlib externs

// Third-party externs
#[macro_use]
extern crate derive_more;

// Local externs

// ===========================================================================
// Re-exports
// ===========================================================================

pub use crate::common::AsPath;
pub use crate::path::Path;

// ===========================================================================
//
// ===========================================================================
