// src/lib.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

pub mod path;
pub mod unix;
pub mod windows;

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod test;

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

pub use crate::path::Path;

// ===========================================================================
//
// ===========================================================================
