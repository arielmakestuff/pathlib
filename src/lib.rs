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

pub mod path;
pub mod prelude;
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

pub use crate::common::AsSystemStr;
pub use crate::path::{SystemStr, SystemString};

// ===========================================================================
//
// ===========================================================================
