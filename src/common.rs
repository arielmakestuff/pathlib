// src/common.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

pub mod error;
pub(crate) mod path_type;
pub(crate) mod string;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

// Local imports
use self::string::as_osstr;
use crate::path::SystemStr;

// ===========================================================================
// Macros
// ===========================================================================

#[macro_export]
macro_rules! component_asref_impl {
    ($type:ident, $path_lifetime:lifetime) => {
        impl<$path_lifetime> AsRef<OsStr> for $type<$path_lifetime> {
            fn as_ref(&self) -> &OsStr {
                self.as_os_str()
            }
        }

        impl<$path_lifetime> AsRef<SystemStr> for $type<$path_lifetime> {
            fn as_ref(&self) -> &SystemStr {
                SystemStr::new(self)
            }
        }
    };
}

#[macro_export]
macro_rules! pathiter_trait_impl {
    ($type:ident, $path_lifetime:lifetime) => {
        impl<$path_lifetime> PathData for $type<$path_lifetime> {
            fn path(&self) -> &[u8] {
                self.path
            }

            fn current_index(&self) -> usize {
                self.cur
            }
        }

        impl<$path_lifetime> AsSystemStr for $type<$path_lifetime> {}

        impl<$path_lifetime> AsRef<SystemStr> for $type<$path_lifetime> {
            fn as_ref(&self) -> &SystemStr {
                self.as_path()
            }
        }
    };
}

// ===========================================================================
// Traits
// ===========================================================================

pub trait PathData {
    fn path(&self) -> &[u8];
    fn current_index(&self) -> usize;
}

pub trait AsSystemStr: PathData {
    fn as_path(&self) -> &SystemStr {
        as_osstr(&self.path()[self.current_index()..]).as_ref()
    }
}

// ===========================================================================
//
// ===========================================================================
