// src/prelude.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Externs
// ===========================================================================

// Stdlib externs

// Third-party externs

// Local externs

// ===========================================================================
// Re-exports
// ===========================================================================

// Local imports
pub use crate::path::{Path, PathBuf};

#[cfg(unix)]
pub use crate::unix::{
    Component as UnixComponent, UnixMemoryPath, UnixMemoryPathBuf,
};

#[cfg(windows)]
pub use crate::windows::{
    Component as WindowsComponent, Prefix, WindowsMemoryPath,
    WindowsMemoryPathBuf,
};

// ===========================================================================
//
// ===========================================================================
