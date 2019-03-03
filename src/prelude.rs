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
// TODO: remove the following once UnixMemoryPath and UnixMemoryPathBuf
// have been removed
pub use crate::path::{Path, PathBuf};

pub use crate::path::MemoryPath as _;
pub use crate::unix::{UnixPath, UnixPathBuf};
pub use crate::windows::{WindowsPath, WindowsPathBuf};

#[cfg(unix)]
pub use crate::unix::{
    Component as UnixComponent, UnixMemoryPath, UnixMemoryPathBuf,
};

#[cfg(windows)]
pub use crate::windows::{
    Component as WindowsComponent, Prefix, PrefixComponent, WindowsMemoryPath,
    WindowsMemoryPathBuf,
};

// ===========================================================================
//
// ===========================================================================
