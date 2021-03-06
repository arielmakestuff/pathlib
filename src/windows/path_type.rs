// src/windows/path_type.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Externs
// ===========================================================================

// Stdlib externs

// Third-party externs

// Local externs

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::collections::HashSet;
use std::fmt;

// Third-party imports
use lazy_static::lazy_static;

// Local imports
use super::{DRIVE_LETTERS, RESERVED_NAMES, RESTRICTED_CHARS, SEPARATOR};
use crate::common::path_type::{CurrentDir, ParentDir};
use crate::mk_reverse_equal;

// ===========================================================================
// Globals
// ===========================================================================

lazy_static! {
    static ref UNC_LETTERS: HashSet<u8> =
        { b"uncUNC".iter().cloned().collect() };
    static ref UNC_WORD_BYTES: Vec<u8> = { b"UNC".to_vec() };
    static ref INVALID_LAST_CHAR: HashSet<u8> =
        { b" .".iter().cloned().collect() };
}

// ===========================================================================
// SystemStr Prefix Types: Disk
// ===========================================================================

#[derive(Debug)]
pub struct Disk;

impl PartialEq<&[u8]> for Disk {
    fn eq(&self, other: &&[u8]) -> bool {
        if other.len() != 2 {
            return false;
        }

        if !DRIVE_LETTERS.contains(&(other[0] as char)) {
            return false;
        }

        other[1] == b':'
    }
}

mk_reverse_equal!(Disk, &[u8]);

// ===========================================================================
// SystemStr Prefix Types: DiskRoot
// ===========================================================================

#[derive(Debug)]
pub struct DiskRoot;

impl PartialEq<&[u8]> for DiskRoot {
    fn eq(&self, other: &&[u8]) -> bool {
        if other.len() != 3 {
            return false;
        }

        if &other[..2] != Disk || other[2] != Separator {
            return false;
        }

        true
    }
}

mk_reverse_equal!(DiskRoot, &[u8]);

// ===========================================================================
// SystemStr Prefix Types: Separator
// ===========================================================================

#[derive(Debug)]
pub struct Separator;

impl PartialEq<u8> for Separator {
    fn eq(&self, other: &u8) -> bool {
        SEPARATOR.contains(other)
    }
}

mk_reverse_equal!(Separator, u8);

// ===========================================================================
// SystemStr Prefix Types: DoubleSlash
// ===========================================================================

// Starts with \\ or //
#[derive(Debug)]
pub struct DoubleSlash;

impl PartialEq<&[u8]> for DoubleSlash {
    fn eq(&self, other: &&[u8]) -> bool {
        if other.len() != 2 {
            return false;
        }

        other.iter().all(|&b| b == Separator)
    }
}

mk_reverse_equal!(DoubleSlash, &[u8]);

// ===========================================================================
// slash types
// ===========================================================================

macro_rules! slash_type {
    ($type_name:ident, $first_char:expr) => {
        #[derive(Debug)]
        pub struct $type_name;

        impl PartialEq<&[u8]> for $type_name {
            fn eq(&self, other: &&[u8]) -> bool {
                if other.len() != 2 {
                    return false;
                }

                if other[0] != $first_char as u8
                    || !SEPARATOR.contains(&other[1])
                {
                    return false;
                }

                true
            }
        }

        mk_reverse_equal!($type_name, &[u8]);
    };
}

slash_type!(QuestionSlash, '?');

slash_type!(DotSlash, '.');

// ===========================================================================
// Device
// ===========================================================================

#[derive(Debug)]
pub struct Device;

impl PartialEq<&[u8]> for Device {
    fn eq(&self, other: &&[u8]) -> bool {
        let ext_start = {
            let mut index = 0;
            for (i, byte) in other.iter().enumerate() {
                if *byte == b'.' {
                    index = i;
                }
            }

            index
        };

        let bytes = {
            if ext_start == 0 {
                other.to_vec()
            } else {
                other[..ext_start].to_vec()
            }
        };

        match String::from_utf8(bytes) {
            Err(_) => false,
            Ok(s) => RESERVED_NAMES.contains(&s.to_uppercase()),
        }
    }
}

mk_reverse_equal!(Device, &[u8]);

// ===========================================================================
// DeviceNamespace
// ===========================================================================

#[derive(Debug)]
pub struct DeviceNamespace;

impl PartialEq<&[u8]> for DeviceNamespace {
    fn eq(&self, other: &&[u8]) -> bool {
        !other.is_empty() && other.iter().all(|c| !RESTRICTED_CHARS.contains(c))
    }
}

mk_reverse_equal!(DeviceNamespace, &[u8]);

// ===========================================================================
// UNCPart
// ===========================================================================

#[derive(Debug)]
pub struct UNCPart;

impl UNCPart {
    pub fn as_str() -> &'static str {
        "UNC"
    }
}

impl fmt::Display for UNCPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", UNCPart::as_str())
    }
}

impl PartialEq<&[u8]> for UNCPart {
    fn eq(&self, other: &&[u8]) -> bool {
        if other.len() != 3 {
            return false;
        }

        for i in 0..3 {
            let cur = other[i];
            if !UNC_LETTERS.contains(&cur) {
                return false;
            }

            let mut cur = cur as char;
            cur.make_ascii_uppercase();

            if cur != UNC_WORD_BYTES[i] as char {
                return false;
            }
        }

        true
    }
}

mk_reverse_equal!(UNCPart, &[u8]);

// ===========================================================================
// UNCRootPart
// ===========================================================================

#[derive(Debug)]
pub struct UNCRootPart;

impl PartialEq<&[u8]> for UNCRootPart {
    fn eq(&self, other: &&[u8]) -> bool {
        other.len() == 4 && &other[..3] == UNCPart && other[3] == Separator
    }
}

mk_reverse_equal!(UNCRootPart, &[u8]);

// ===========================================================================
// NonUNCPart
// ===========================================================================

#[derive(Debug)]
pub struct NonUNCPart;

impl PartialEq<&[u8]> for NonUNCPart {
    fn eq(&self, other: &&[u8]) -> bool {
        if *other == UNCPart || *other == CurrentDir || *other == ParentDir {
            false
        } else {
            *other == NonDevicePart
        }
    }
}

mk_reverse_equal!(NonUNCPart, &[u8]);

// ===========================================================================
// NonDevicePart
// ===========================================================================

#[derive(Debug)]
pub struct NonDevicePart;

impl PartialEq<&[u8]> for NonDevicePart {
    fn eq(&self, other: &&[u8]) -> bool {
        if *other == Device
            || (*other != CurrentDir
                && *other != ParentDir
                && *other == InvalidLastChar)
        {
            return false;
        }

        !other.iter().any(|b| RESTRICTED_CHARS.contains(b))
    }
}

mk_reverse_equal!(NonDevicePart, &[u8]);

// ===========================================================================
// ServerShare
// ===========================================================================

#[derive(Debug)]
pub struct ServerShare;

impl PartialEq<&[u8]> for ServerShare {
    fn eq(&self, other: &&[u8]) -> bool {
        let mut found = 0;
        for part in other.split(|&sep| sep == Separator) {
            if found > 2 || part != NonDevicePart {
                return false;
            }
            found += 1;
        }

        found == 2
    }
}

mk_reverse_equal!(ServerShare, &[u8]);

// ===========================================================================
// ValidLastChar
// ===========================================================================

#[derive(Debug)]
pub struct ValidLastChar;

impl PartialEq<&[u8]> for ValidLastChar {
    fn eq(&self, other: &&[u8]) -> bool {
        let last_index = other.len().checked_sub(1);
        match last_index {
            None => false,
            Some(i) => {
                let last_char = &other[i];
                !INVALID_LAST_CHAR.contains(last_char)
                    && !RESTRICTED_CHARS.contains(last_char)
            }
        }
    }
}

mk_reverse_equal!(ValidLastChar, &[u8]);

// ===========================================================================
// InvalidLastChar
// ===========================================================================

#[derive(Debug)]
pub struct InvalidLastChar;

impl PartialEq<&[u8]> for InvalidLastChar {
    fn eq(&self, other: &&[u8]) -> bool {
        *other != ValidLastChar
    }
}

mk_reverse_equal!(InvalidLastChar, &[u8]);

// ===========================================================================
// FileExtension
// ===========================================================================

#[derive(Debug)]
pub struct FileExtension;

impl PartialEq<&[u8]> for FileExtension {
    fn eq(&self, other: &&[u8]) -> bool {
        let len = other.len();
        if len <= 1 || *other == InvalidLastChar || other[0] != b'.' {
            return false;
        }

        // Already confirmed first and last char, only need to check if the
        // other chars are valid
        let found_invalid_char = &other[1..len - 1]
            .iter()
            .any(|el| *el == b'.' || RESTRICTED_CHARS.contains(el));

        !found_invalid_char
    }
}

mk_reverse_equal!(FileExtension, &[u8]);

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod test {
    // Exclude restricted printable chars and any char with ascii code 0 - 31
    const CHAR_REGEX: &str = r#"[/\\<>:"|?*\x00-\x1F]"#;
    const COMP_REGEX: &str = r#"[^/\\<>:"|?*\x00-\x1F]+"#;
    const VALID_CHARS_NOEXT: &str =
        r#"[^./\\<>:"|?*\x00-\x1F]*[^./\\<>:"|?*\x00-\x1F ]+"#;

    mod disk {
        use crate::windows::path_type::{Disk, DRIVE_LETTERS};

        use proptest::{
            prop_assert, prop_assert_eq, prop_assert_ne, prop_assume, proptest,
        };

        #[test]
        fn self_equal() {
            assert_eq!(Disk, Disk);
        }

        proptest! {
            #[test]
            fn valid_value(s in r#"[a-zA-Z][:]"#) {
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assert_eq!(Disk, &arr[..]);
            }

            #[test]
            fn ne_len_value(s in r#".*"#) {
                prop_assume!(s.len() != 2);
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assert_ne!(Disk, &arr[..]);
            }

            #[test]
            fn ne_value(s in r#".."#) {
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assume!(arr.len() == 2);
                prop_assume!(!DRIVE_LETTERS.contains(&(arr[0] as char)) ||
                             arr[1] != ':' as u8);

                prop_assert_ne!(Disk, &arr[..]);
            }
        }
    }

    mod diskroot {
        use crate::windows::path_type::{DiskRoot, DRIVE_LETTERS, SEPARATOR};

        use proptest::{
            prop_assert, prop_assert_eq, prop_assert_ne, prop_assume, proptest,
        };

        #[test]
        fn self_equal() {
            assert_eq!(DiskRoot, DiskRoot);
        }

        proptest! {
            #[test]
            fn valid_value(s in r#"[a-zA-Z][:]\\"#) {
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assert_eq!(DiskRoot, &arr[..]);
            }

            #[test]
            fn ne_len_value(s in r#".*"#) {
                prop_assume!(s.len() != 3);
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assert_ne!(DiskRoot, &arr[..]);
            }

            #[test]
            fn ne_value(s in r#"..."#) {
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assume!(arr.len() == 3);
                prop_assume!(!DRIVE_LETTERS.contains(&(arr[0] as char)) ||
                             arr[1] != ':' as u8 ||
                             !SEPARATOR.contains(&arr[2]));
                prop_assert_ne!(DiskRoot, &arr[..]);
            }
        }
    }

    mod separator {
        use crate::windows::path_type::Separator;

        #[test]
        fn self_equal() {
            assert_eq!(Separator, Separator);
        }
    }

    mod doubleslash {
        use crate::windows::path_type::{DoubleSlash, SEPARATOR};

        use proptest::{
            prop_assert, prop_assert_eq, prop_assert_ne, prop_assume, proptest,
        };

        #[test]
        fn self_equal() {
            assert_eq!(DoubleSlash, DoubleSlash);
        }

        proptest! {
            #[test]
            fn any_separator(s in r#"[/\\][/\\]"#) {
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assert_eq!(DoubleSlash, &arr[..]);
            }

            #[test]
            fn ne_len_value(s in r#".*"#) {
                prop_assume!(s.len() != 2);
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assert_ne!(DoubleSlash, &arr[..]);
            }

            #[test]
            fn ne_value(s in r#".."#) {
                prop_assume!(s.len() == 2);
                prop_assume!(!s.bytes().take(2).all(|c| SEPARATOR.contains(&c)));

                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assert_ne!(DoubleSlash, &arr[..]);
            }
        }
    }

    // Create common test modules
    macro_rules! mk_slash_type_test {
        ($mod_name:ident, $type_name:ident, $first_test:expr, $first_char:expr) => {
            mod $mod_name {
                use crate::windows::path_type::{$type_name, SEPARATOR};

                use proptest::{
                    prop_assert, prop_assert_eq, prop_assert_ne, prop_assume,
                    proptest,
                };

                #[test]
                fn self_equal() {
                    assert_eq!($type_name, $type_name);
                }

                #[test]
                fn has_debug() {
                    assert!(format!("{:?}", $type_name).len() > 0,);
                }

                proptest! {
                    #[test]
                    fn any_separator(s in $first_test) {
                        let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                        prop_assert_eq!($type_name, &arr[..]);
                    }

                    #[test]
                    fn ne_len_value(s in r#".*"#) {
                        prop_assume!(s.len() != 2);
                        let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                        prop_assert_ne!($type_name, &arr[..]);
                    }

                    #[test]
                    fn ne_value(s in r#".."#) {
                        prop_assume!(s.len() == 2);

                        let arr: Vec<u8> = s.bytes().map(|b| b as u8).collect();
                        prop_assume!(arr[0] != $first_char as u8 ||
                                     !SEPARATOR.contains(&arr[1]));

                        let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                        prop_assert_ne!($type_name, &arr[..]);
                    }
                }
            }
        };
    }

    mk_slash_type_test!(questionslash, QuestionSlash, r#"\?[/\\]"#, '?');

    mk_slash_type_test!(dotslash, DotSlash, r#"\.[/\\]"#, '.');

    mod device {
        use super::*;

        use crate::windows::path_type::{
            Device, FileExtension, RESERVED_NAMES,
        };

        use proptest::{
            prop_assert, prop_assert_eq, prop_assert_ne, prop_assume, proptest,
        };

        #[test]
        fn self_equal() {
            assert_eq!(Device, Device);
        }

        proptest! {
            #[test]
            fn valid_value(i in 0..RESERVED_NAMES.len(),
                           ext in VALID_CHARS_NOEXT)
            {
                let arr: Vec<&[u8]> = RESERVED_NAMES.iter()
                    .map(|s| s.as_bytes()).collect();
                let val: Vec<u8> = arr[i].iter()
                    .map(|&b| b as u8).collect();
                prop_assert_eq!(Device, &val[..]);

                let mut val_ext: Vec<u8> = val.clone();
                val_ext.push(b'.');
                val_ext.extend(ext.bytes());
                prop_assert_eq!(FileExtension, &val_ext[val.len()..]);
                prop_assert_eq!(Device, &val_ext[..]);
            }

            #[test]
            fn ne_value(s in r#".*"#) {
                prop_assume!(!RESERVED_NAMES.contains(&s));

                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assert_ne!(Device, &arr[..]);
            }
        }
    }

    mod devicenamespace {
        use super::*;

        use crate::windows::path_type::{DeviceNamespace, RESTRICTED_CHARS};

        use proptest::{prop_assert, prop_assert_eq, prop_assert_ne, proptest};

        #[test]
        fn self_equal() {
            assert_eq!(DeviceNamespace, DeviceNamespace);
        }

        proptest! {
            #[test]
            fn valid_value(name in COMP_REGEX) {
                let val = name.as_bytes();
                prop_assert_eq!(DeviceNamespace, val);
            }

            #[test]
            fn ne_value(s in r#".*"#, c in CHAR_REGEX) {
                let mut bytes = Vec::from(s.as_bytes());

                // Make sure the generated string contains at least a single
                // restricted character
                if !bytes.iter().any(|b| RESTRICTED_CHARS.contains(b)) {
                    let mid = bytes.len() / 2;
                    let mut val = Vec::with_capacity(bytes.len() + c.len());
                    val.extend(&bytes[..mid]);
                    val.extend(c.as_bytes());
                    val.extend(&bytes[mid..]);
                    bytes = val;
                }

                prop_assert_ne!(DeviceNamespace, &bytes[..]);
            }
        }
    }

    mod uncpart {
        use crate::windows::path_type::{UNCPart, UNC_LETTERS};

        use proptest::{
            prop_assert, prop_assert_eq, prop_assert_ne, prop_assume, proptest,
        };

        #[test]
        fn self_equal() {
            assert_eq!(UNCPart, UNCPart);
        }

        #[test]
        fn as_str() {
            assert_eq!(UNCPart::as_str(), "UNC");
        }

        #[test]
        fn display() {
            assert_eq!(format!("{}", UNCPart), String::from("UNC"));
        }

        proptest! {
            #[test]
            fn valid_value(u_char in r#"[uU]"#,
                           n_char in r#"[nN]"#,
                           c_char in r#"[cC]"#)
            {
                let unc_word = format!("{}{}{}", u_char, n_char, c_char);
                let bytes: Vec<u8> = unc_word.bytes()
                    .map(|b| b as u8).collect();
                prop_assert_eq!(UNCPart, &bytes[..]);
            }

            #[test]
            fn ne_len_value(s in r#".*"#) {
                prop_assume!(s.len() != 3);
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assert_ne!(UNCPart, &arr[..]);
            }

            #[test]
            fn ne_value(s in r#"..."#) {
                prop_assume!(s.len() == 3);
                let bytes = s.as_bytes();

                prop_assume!(!UNC_LETTERS.contains(&(bytes[0] as u8))
                             && !UNC_LETTERS.contains(&(bytes[1] as u8))
                             && !UNC_LETTERS.contains(&(bytes[2] as u8)));

                let arr: Vec<u8> = bytes.iter().map(|&c| c as u8).collect();
                prop_assert_ne!(UNCPart, &arr[..]);
            }
        }
    }

    mod uncrootpart {
        use crate::windows::path_type::{Separator, UNCPart, UNCRootPart};

        use proptest::{
            prop_assert, prop_assert_eq, prop_assert_ne, prop_assume, proptest,
        };

        #[test]
        fn self_equal() {
            assert_eq!(UNCRootPart, UNCRootPart);
        }

        proptest! {
            #[test]
            fn valid_value(u_char in r#"[uU]"#,
                           n_char in r#"[nN]"#,
                           c_char in r#"[cC]"#,
                           sep in r#"[/\\]"#)
            {
                let unc_word = format!("{}{}{}{}", u_char, n_char, c_char, sep);
                let bytes: Vec<u8> = unc_word.bytes()
                    .map(|b| b as u8).collect();
                prop_assert_eq!(UNCRootPart, &bytes[..]);
            }

            #[test]
            fn ne_len_value(s in r#".*"#) {
                prop_assume!(s.len() != 4);
                let arr: Vec<u8> = s.bytes().map(|c| c as u8).collect();
                prop_assert_ne!(UNCPart, &arr[..]);
            }

            #[test]
            fn ne_value(s in r#"...."#) {
                let bytes = s.as_bytes();

                prop_assume!(&bytes[..3] != UNCPart || bytes[3] != Separator);

                let arr: Vec<u8> = bytes.iter().map(|&c| c as u8).collect();
                prop_assert_ne!(UNCRootPart, &arr[..]);
            }
        }
    }

    mod nonuncpart {
        use super::*;

        use crate::windows::path_type::{
            Device, NonUNCPart, UNCPart, INVALID_LAST_CHAR, RESERVED_NAMES,
            SEPARATOR,
        };

        use proptest::{
            prop_assert, prop_assert_eq, prop_assert_ne, prop_assume, proptest,
        };

        #[test]
        fn self_equal() {
            assert_eq!(NonUNCPart, NonUNCPart);
        }

        proptest! {
            #[test]
            fn valid_value(s in COMP_REGEX) {
                prop_assume!(
                    !INVALID_LAST_CHAR.contains(&s.as_bytes()[s.len() - 1])
                );

                let bytes: Vec<u8> = s.bytes().map(|b| b as u8).collect();
                prop_assume!(&bytes[..] != UNCPart && &bytes[..] != Device);
                prop_assert_eq!(NonUNCPart, &bytes[..]);
            }

            #[test]
            fn ne_uncpart_value(u_char in r#"[uU]"#,
                                n_char in r#"[nN]"#,
                                c_char in r#"[cC]"#)
            {
                let unc_word = format!("{}{}{}", u_char, n_char, c_char);
                let bytes: Vec<u8> = unc_word.bytes()
                    .map(|b| b as u8).collect();
                prop_assert_ne!(NonUNCPart, &bytes[..]);
            }

            #[test]
            fn ne_device_value(i in 0..RESERVED_NAMES.len()) {
                let arr: Vec<&[u8]> = RESERVED_NAMES.iter()
                    .map(|s| s.as_bytes()).collect();
                let val: Vec<u8> = arr[i].iter()
                    .map(|&b| b as u8).collect();
                prop_assert_ne!(NonUNCPart, &val[..]);
            }

            #[test]
            fn ne_has_separator(s in r#".*"#, sep in r#"[/\\]"#) {
                let mut bytes: Vec<u8> = s.bytes()
                    .map(|b| b as u8).collect();
                prop_assume!(&bytes[..] != UNCPart
                             && &bytes[..] != Device);

                if bytes.iter().all(|b| !SEPARATOR.contains(b)) {
                    bytes.push(sep.bytes().nth(0).unwrap() as u8);
                }
                prop_assert_ne!(NonUNCPart, &bytes[..]);
            }

            #[test]
            fn ne_invalid_lastchar(s in r#".*[ .]"#) {
                let bytes: Vec<u8> = s.bytes()
                    .map(|b| b as u8).collect();
                prop_assume!(&bytes[..] != UNCPart
                             && &bytes[..] != Device);

                prop_assert_ne!(NonUNCPart, &bytes[..]);
            }

            #[test]
            fn ne_invalid_dirname(s in r#"([.])|([.][.])"#) {
                let bytes: Vec<u8> = s.bytes()
                    .map(|b| b as u8).collect();
                prop_assert_ne!(NonUNCPart, &bytes[..]);
            }
        }
    }

    mod nondevicepart {
        use super::*;

        use crate::windows::path_type::{
            CurrentDir, Device, NonDevicePart, ParentDir, INVALID_LAST_CHAR,
            RESERVED_NAMES, SEPARATOR,
        };

        use proptest::{
            prop_assert, prop_assert_eq, prop_assert_ne, prop_assume, proptest,
        };

        #[test]
        fn self_equal() {
            assert_eq!(NonDevicePart, NonDevicePart);
        }

        proptest! {
            #[test]
            fn valid_value(s in COMP_REGEX) {
                let bytes: Vec<u8> = s.bytes().map(|b| b as u8).collect();
                prop_assume!(
                    &bytes[..] != Device
                    && !INVALID_LAST_CHAR
                           .contains(&bytes[s.len() - 1])
                 );
                prop_assert_eq!(NonDevicePart, &bytes[..]);
            }

            #[test]
            fn eq_uncpart_value(u_char in r#"[uU]"#,
                                n_char in r#"[nN]"#,
                                c_char in r#"[cC]"#)
            {
                let unc_word = format!("{}{}{}", u_char, n_char, c_char);
                let bytes: Vec<u8> = unc_word.bytes()
                    .map(|b| b as u8).collect();
                prop_assert_eq!(NonDevicePart, &bytes[..]);
            }

            #[test]
            fn ne_device_value(i in 0..RESERVED_NAMES.len()) {
                let arr: Vec<&[u8]> = RESERVED_NAMES.iter()
                    .map(|s| s.as_bytes()).collect();
                let val: Vec<u8> = arr[i].iter()
                    .map(|&b| b as u8).collect();
                prop_assert_ne!(NonDevicePart, &val[..]);
            }

            #[test]
            fn ne_has_separator(s in r#".*"#, sep in r#"[/\\]"#) {
                let mut bytes: Vec<u8> = s.bytes()
                    .map(|b| b as u8).collect();
                prop_assume!(&bytes[..] != Device);

                if bytes.iter().all(|b| !SEPARATOR.contains(b)) {
                    bytes.push(sep.bytes().nth(0).unwrap() as u8);
                }
                prop_assert_ne!(NonDevicePart, &bytes[..]);
            }

            #[test]
            fn ne_invalid_lastchar(s in r#".*[ .]"#) {
                let bytes: Vec<u8> = s.bytes()
                    .map(|b| b as u8).collect();
                prop_assume!(&bytes[..] != ParentDir
                             && &bytes[..] != CurrentDir);

                prop_assert_ne!(NonDevicePart, &bytes[..]);
            }

            #[test]
            fn ne_reserved_dirname(s in r#"([.])|([.][.])"#) {
                let bytes: Vec<u8> = s.bytes()
                    .map(|b| b as u8).collect();
                prop_assert_eq!(NonDevicePart, &bytes[..]);
            }
        }
    }

    mod servershare {
        use super::*;

        use crate::windows::path_type::{
            Device, NonDevicePart, ServerShare, RESERVED_NAMES, SEPARATOR,
        };

        use proptest::{
            prop_assert, prop_assert_eq, prop_assert_ne, prop_assume, proptest,
        };

        #[test]
        fn self_equal() {
            assert_eq!(ServerShare, ServerShare);
        }

        proptest! {
            #[test]
            fn valid_value(server in COMP_REGEX,
                           share in COMP_REGEX,
                           sep in r#"[/\\]"#)
            {
                let server_bytes: Vec<u8> = server
                    .bytes().map(|b| b as u8).collect();
                let share_bytes: Vec<u8> = share
                    .bytes().map(|b| b as u8).collect();
                prop_assume!(
                    &server_bytes[..] == NonDevicePart
                    && &share_bytes[..] == NonDevicePart
                );

                let mut server_share: Vec<u8> = Vec::new();
                server_share.extend(server_bytes.iter());
                server_share.push(
                    *sep.as_bytes().iter().nth(0).unwrap() as u8
                );
                server_share.extend(share_bytes.iter());

                prop_assert_eq!(&server_share[..], ServerShare);
            }

            #[test]
            fn ne_too_many_parts(server in COMP_REGEX,
                                 share in COMP_REGEX,
                                 other in COMP_REGEX,
                                 sep in r#"[/\\]"#)
            {
                let server_bytes: Vec<u8> = server
                    .bytes().map(|b| b as u8).collect();
                let share_bytes: Vec<u8> = share
                    .bytes().map(|b| b as u8).collect();
                let other_bytes: Vec<u8> = other
                    .bytes().map(|b| b as u8).collect();
                prop_assume!(
                    &server_bytes[..] == NonDevicePart
                    && &share_bytes[..] == NonDevicePart
                );

                let mut server_share: Vec<u8> = Vec::new();
                let sep_elem = *sep.as_bytes().iter().nth(0).unwrap() as u8;
                server_share.extend(server_bytes.iter());
                server_share.push(sep_elem);
                server_share.extend(share_bytes.iter());
                server_share.push(sep_elem);
                server_share.extend(other_bytes.iter());

                prop_assert_ne!(&server_share[..], ServerShare);
            }

            #[test]
            fn ne_no_separator(s in r#".*"#) {
                let bytes: Vec<u8> = s.bytes().map(|b| b as u8).collect();
                prop_assume!(&bytes[..] != Device
                             && !bytes.iter().any(|b| SEPARATOR.contains(b)));
                prop_assert_ne!(ServerShare, &bytes[..]);
            }

            #[test]
            fn ne_server_device(i in 0..RESERVED_NAMES.len(),
                                share in r#".*"#,
                                sep in r#"[/\\]"#)
            {

                let share_bytes: Vec<u8> = share.bytes()
                    .map(|b| b as u8).collect();
                prop_assume!(
                    &share_bytes[..] != Device
                    && !share_bytes.iter().any(|b| SEPARATOR.contains(b))
                );

                let arr: Vec<&[u8]> = RESERVED_NAMES.iter()
                    .map(|s| s.as_bytes()).collect();
                let server: Vec<u8> = arr[i].iter()
                    .map(|&b| b as u8).collect();

                let mut server_share: Vec<u8> = Vec::new();
                server_share.extend(server.iter());
                server_share.push(
                    *sep.as_bytes().iter().nth(0).unwrap() as u8
                );
                server_share.extend(share_bytes.iter());

                prop_assert_ne!(ServerShare, &server_share[..]);
            }

            #[test]
            fn ne_share_device(i in 0..RESERVED_NAMES.len(),
                               server in r#".*"#,
                               sep in r#"[/\\]"#)
            {

                let server_bytes: Vec<u8> = server.bytes()
                    .map(|b| b as u8).collect();
                prop_assume!(
                    &server_bytes[..] != Device
                    && !server_bytes.iter().any(|b| SEPARATOR.contains(b))
                );

                let arr: Vec<&[u8]> = RESERVED_NAMES.iter()
                    .map(|s| s.as_bytes()).collect();
                let share: Vec<u8> = arr[i].iter()
                    .map(|&b| b as u8).collect();

                let mut server_share: Vec<u8> = Vec::new();
                server_share.extend(server_bytes.iter());
                server_share.push(
                    *sep.as_bytes().iter().nth(0).unwrap() as u8
                );
                server_share.extend(share.iter());

                prop_assert_ne!(ServerShare, &server_share[..]);
            }
        }
    }

    mod validlastchar {
        use super::*;

        use crate::windows::path_type::{ValidLastChar, INVALID_LAST_CHAR};

        use proptest::{
            prop_assert, prop_assert_eq, prop_assert_ne, prop_assume, proptest,
        };

        #[test]
        fn self_equal() {
            assert_eq!(ValidLastChar, ValidLastChar);
        }

        proptest! {
            #[test]
            fn valid_value(comp in COMP_REGEX) {
                prop_assume!(
                    !INVALID_LAST_CHAR
                        .contains(&comp.as_bytes()[comp.len() - 1])
                );

                prop_assert_eq!(comp.as_bytes(), ValidLastChar);
            }

            #[test]
            fn invalid_value(comp in r#".*[. ]"#) {
                prop_assert_ne!(comp.as_bytes(), ValidLastChar);
            }
        }
    }

    mod invalidlastchar {
        use super::*;

        use crate::windows::path_type::{InvalidLastChar, INVALID_LAST_CHAR};

        use proptest::{
            prop_assert, prop_assert_eq, prop_assert_ne, prop_assume, proptest,
        };

        #[test]
        fn self_equal() {
            assert_eq!(InvalidLastChar, InvalidLastChar);
        }

        proptest! {
            #[test]
            fn valid_value(comp in COMP_REGEX) {
                prop_assume!(
                    !INVALID_LAST_CHAR
                        .contains(&comp.as_bytes()[comp.len() - 1])
                );

                prop_assert_ne!(comp.as_bytes(), InvalidLastChar);
            }

            #[test]
            fn invalid_value(comp in r#".*[. ]"#) {
                prop_assert_eq!(comp.as_bytes(), InvalidLastChar);
            }
        }
    }

    mod fileextension {
        use super::*;

        use crate::windows::path_type::{FileExtension, ValidLastChar};

        use proptest::{
            prop_assert, prop_assert_eq, prop_assert_ne, prop_assume, proptest,
        };

        const INVALID_CHARS: &str = r#"[/\\<>:"|?*\x00-\x1F]+"#;
        const VALID_CHARS: &str = r#"[^./\\<>:"|?*\x00-\x1F]+"#;

        #[test]
        fn self_equal() {
            assert_eq!(FileExtension, FileExtension);
        }

        #[test]
        fn empty_string() {
            let empty = "".as_bytes();
            assert_ne!(empty, FileExtension);
        }

        proptest! {
            #[test]
            fn invalid_last_char(head in COMP_REGEX,
                                 tail in r"[ \.]")
            {
                let name = format!(".{head}{tail}", head = head, tail = tail);
                prop_assert_ne!(name.as_bytes(), FileExtension);
            }

            #[test]
            fn single_char_is_invalid(name in CHAR_REGEX) {
                prop_assert_ne!(name.as_bytes(), FileExtension);
            }

            #[test]
            fn non_period_head_is_invalid(head in CHAR_REGEX,
                                          tail in COMP_REGEX)
            {
                prop_assume!(
                    head != "."
                    && tail.as_bytes() == ValidLastChar
                );

                let name = format!("{}{}", head, tail);
                prop_assert_ne!(name.as_bytes(), FileExtension);
            }

            #[test]
            fn restricted_char_is_invalid(first in COMP_REGEX,
                                          last in COMP_REGEX,
                                          invalid in INVALID_CHARS)
            {
                prop_assume!(last.as_bytes() == ValidLastChar);

                let name = format!(".{}{}{}", first, invalid, last);
                prop_assert_ne!(name.as_bytes(), FileExtension);
            }

            #[test]
            fn period_char_is_invalid(first in COMP_REGEX,
                                      last in COMP_REGEX)
            {
                prop_assume!(last.as_bytes() == ValidLastChar);

                let name = format!(".{}.{}", first, last);
                prop_assert_ne!(name.as_bytes(), FileExtension);
            }

            #[test]
            fn valid_extension(ext in VALID_CHARS)
            {
                let name = format!(".{}", ext);
                let bytes = name.as_bytes();

                prop_assume!(bytes == ValidLastChar);

                prop_assert_eq!(bytes, FileExtension);
            }
        }
    }
}

// ===========================================================================
//
// ===========================================================================
