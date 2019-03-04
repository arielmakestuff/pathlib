// src/windows/match_prefix.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::ffi::OsStr;
use std::path::Prefix;

// Third-party imports

// Local imports
use super::{path_type, SEPARATOR};
use crate::common::string::{as_osstr, as_str};

// ===========================================================================
// Helpers
// ===========================================================================

fn ascii_uppercase(letter: u8) -> u8 {
    (letter as char).to_ascii_uppercase() as u8
}

// ===========================================================================
// Matcher functions
// ===========================================================================

pub fn match_prefix(path: &[u8]) -> Option<(usize, Prefix)> {
    let end = 2;
    if path.len() < end {
        return None;
    }

    let part = &path[..end];
    if part == path_type::DoubleSlash {
        match_doubleslash(path, end)
    } else {
        match_disk(path, end)
    }
}

// Endpoint (from prefix)
fn match_disk(path: &[u8], first: usize) -> Option<(usize, Prefix)> {
    let part = &path[..first];

    if part == path_type::Disk {
        let letter = ascii_uppercase(part[0]);
        Some((first, Prefix::Disk(letter)))
    } else {
        None
    }
}

// from prefix
fn match_doubleslash(path: &[u8], first: usize) -> Option<(usize, Prefix)> {
    let end = first + 2;
    if path.len() < end {
        return None;
    }

    let next_two = &path[first..end];
    if next_two == path_type::DotSlash {
        match_devicens(path, end)
    } else if next_two == path_type::QuestionSlash {
        type Method = fn(&[u8], usize) -> Option<(usize, Prefix)>;
        let match_funcs: [Method; 3] =
            [match_verbatimunc, match_verbatimdisk, match_verbatim];

        for f in match_funcs.iter() {
            let result = f(path, end);
            if result.is_some() {
                return result;
            }
        }

        None
    } else {
        match_unc(path, first)
    }
}

fn match_verbatim(path: &[u8], first: usize) -> Option<(usize, Prefix)> {
    let mut end = path.len();

    for (i, c) in path[first..end].iter().enumerate() {
        if SEPARATOR.contains(c) {
            end = i + first;
            break;
        }
    }

    let part = &path[first..end];
    if part == path_type::NonUNCPart {
        let strval = as_str(part);
        let val = OsStr::new(strval);
        Some((end, Prefix::Verbatim(val)))
    } else {
        None
    }
}

fn match_verbatimdisk(path: &[u8], first: usize) -> Option<(usize, Prefix)> {
    let end = first + 3;
    if path.len() < end {
        return None;
    }

    let part = &path[first..end];
    if part == path_type::DiskRoot {
        let letter = ascii_uppercase(path[first]);
        Some((end, Prefix::VerbatimDisk(letter)))
    } else {
        None
    }
}

// endpoint (from match_doubleslash)
fn match_unc(path: &[u8], first: usize) -> Option<(usize, Prefix)> {
    let end = path.len();

    let mut sep_index: Vec<usize> = Vec::with_capacity(2);
    for (i, c) in path[first..end].iter().enumerate() {
        if SEPARATOR.contains(c) {
            sep_index.push(i + first);
            if sep_index.len() == 2 {
                break;
            }
        }
    }

    let num_sep = sep_index.len();
    if num_sep == 0 {
        return None;
    }

    let last = if num_sep == 1 { end } else { sep_index[1] };

    let part = &path[first..last];
    if part == path_type::ServerShare {
        let server = &path[first..sep_index[0]];
        let share = &path[sep_index[0] + 1..last];

        let (server_val, share_val) = (as_osstr(server), as_osstr(share));
        let prefix = Prefix::UNC(server_val, share_val);
        Some((last, prefix))
    } else {
        None
    }
}

// endpoint (from match_doubleslash)
fn match_verbatimunc(path: &[u8], first: usize) -> Option<(usize, Prefix)> {
    let part_end = first + 4;
    if path.len() < part_end {
        return None;
    }

    let unc_part = &path[first..part_end];

    if unc_part != path_type::UNCRootPart {
        return None;
    }

    let result = match_unc(path, part_end);
    if let Some((p, Prefix::UNC(server, share))) = result {
        Some((p, Prefix::VerbatimUNC(server, share)))
    } else {
        result
    }
}

// Endpoint (from match_doubleslash)
fn match_devicens(path: &[u8], first: usize) -> Option<(usize, Prefix)> {
    let mut end = path.len();

    // Get all bytes until first separator
    for (i, c) in path[first..end].iter().enumerate() {
        if SEPARATOR.contains(c) {
            end = i + first;
            break;
        }
    }

    let part = &path[first..end];
    if part == path_type::DeviceNamespace {
        let prefix = Prefix::DeviceNS(as_osstr(part));
        Some((end, prefix))
    } else {
        None
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod test {
    use crate::windows::match_prefix::match_prefix;
    use crate::windows::path_type;
    use std::ffi::OsStr;
    use std::path::Prefix;

    use proptest::prelude::*;
    use proptest::{
        prop_assert, prop_assume, prop_compose, prop_oneof, proptest,
    };

    // Exclude restricted printable chars and any char with ascii code 0 - 31
    const COMP_REGEX: &str = r#"[^/\\<>:"|?*\x00-\x1F]+"#;
    const VALID_CHARS: &str =
        r#"[^/\\<>:"|?*\x00-\x1F]*[^./\\<>:"|?*\x00-\x1F ]+"#;
    const VALID_CHARS_NOEXT: &str =
        r#"[^./\\<>:"|?*\x00-\x1F]*[^./\\<>:"|?*\x00-\x1F ]+"#;

    mod noprefix {
        use super::*;

        #[test]
        fn empty_string() {
            let value = match_prefix(b"");
            assert_eq!(value, None);
        }

        #[test]
        fn doubleslash() {
            let value = match_prefix(br#"\\"#);
            assert_eq!(value, None);
        }

        #[test]
        fn empty_verbatim() {
            let value = match_prefix(br#"\\?\"#);
            assert_eq!(value, None);
        }

        // string without any separator characters is not a unc prefix
        #[test]
        fn unc_noseparator() {
            let value = match_prefix(br#"\\helloworld"#);
            assert_eq!(value, None);
        }

        // string with a server and separator but without a share is not a unc
        // prefix
        #[test]
        fn unc_not_servershare() {
            let value = match_prefix(br#"\\helloworld\"#);
            assert_eq!(value, None);
        }

        #[test]
        fn verbatimunc_no_separator() {
            let value = match_prefix(br#"\\?\UNC\helloworld"#);
            assert_eq!(value, None);
        }

        #[test]
        fn devicens_no_device() {
            let value = match_prefix(br#"\\.\"#);
            assert_eq!(value, None);
        }
    }

    mod verbatim {
        use super::*;

        #[test]
        fn simple() {
            // --------------------
            // GIVEN
            // --------------------
            // A byte string with a Prefix::Verbatim prefix

            let pathstr = br#"\\?\hello\world"#;

            // --------------------
            // WHEN
            // --------------------
            // match_prefix is called with the pathstr
            let value = match_prefix(pathstr);

            // --------------------
            // THEN
            // --------------------
            // The returned value is not None and
            // the returned value is a 2-tuple and
            // the 2-tuple's first element is a usize indicating
            //     the end index of the prefix and
            // the 2-tuple's second element is a Prefix::Verbatim
            //     and the Prefix::Verbatim contains the first component
            //     following \\?\
            let result = match value {
                None => false,
                Some((index, prefix)) => {
                    let res = &pathstr[..index] == br#"\\?\hello"#;
                    match prefix {
                        Prefix::Verbatim(comp) => {
                            res && comp == OsStr::new("hello")
                        }
                        _ => false,
                    }
                }
            };
            assert!(result);
        }

        proptest! {
            #[test]
            fn return_only_first_component(
                comp in prop::collection::vec(VALID_CHARS, 1..10),
                sep in prop_oneof!(Just("/"), Just(r#"\"#))
            ) {
                // --------------------
                // GIVEN
                // --------------------
                // a valid separator character and
                // a Verbatim path made up of 1 or more valid components
                let valid_comp = comp
                    .iter()
                    .all(|c| c[..].as_bytes() == path_type::NonUNCPart);
                prop_assume!(valid_comp);

                let sep = String::from(sep);
                let prefix = format!("{sep}{sep}?{sep}", sep = sep);
                let mut path = prefix.clone();
                path.push_str(&comp.join(&sep)[..]);

                let mut expected_path = prefix.clone();
                expected_path.push_str(&comp[0][..]);

                // --------------------
                // WHEN
                // --------------------
                // the match_prefix() function is called with the Verbatim
                // path
                let value = match_prefix(&path[..].as_bytes());

                // --------------------
                // THEN
                // --------------------
                // the returned value is not None and
                // the value is a 2-tuple and
                // the value's first element is the end index
                //     of the prefix and
                // the value's second element is a Prefix::Verbatim
                //     containing only the first path component
                let result = match value {
                    Some((end, p)) => {
                        let path_comp = OsStr::new(&comp[0][..]);
                        &path[..end] == &expected_path[..]
                            && p == Prefix::Verbatim(path_comp)
                    }
                    _ => false,
                };
                prop_assert!(result);
            }
        }
    }

    mod verbatimunc {
        use super::*;

        #[test]
        fn simple() {
            // --------------------
            // GIVEN
            // --------------------
            // A byte string with a Prefix::VerbatimUNC prefix

            let pathstr = br#"\\?\UNC\hello\again\world"#;

            // --------------------
            // WHEN
            // --------------------
            // match_prefix is called with the pathstr
            let value = match_prefix(pathstr);

            // --------------------
            // THEN
            // --------------------
            // The returned value is not None and
            // the returned value is a 2-tuple and
            // the 2-tuple's first element is a usize indicating
            //     the end index of the prefix and
            // the 2-tuple's second element is a Prefix::Verbatim
            //     and the Prefix::Verbatim contains the first component
            //     following \\?\
            let result = match value {
                None => false,
                Some((index, prefix)) => {
                    let res = &pathstr[..index] == br#"\\?\UNC\hello\again"#;
                    match prefix {
                        Prefix::VerbatimUNC(server, share) => {
                            res && server == OsStr::new("hello")
                                && share == OsStr::new("again")
                        }
                        _ => false,
                    }
                }
            };
            assert!(result);
        }

        proptest! {
            #[test]
            fn return_only_server_share(
                server in VALID_CHARS_NOEXT,
                share in VALID_CHARS_NOEXT,
                comp in prop::collection::vec(VALID_CHARS, 0..10),
                sep in prop_oneof!(Just("/"), Just(r#"\"#))
            ) {
                // --------------------
                // GIVEN
                // --------------------
                // a valid separator character and
                // a VerbatimUNC path made up of 1 or more valid components

                let valid_comp = comp
                    .iter()
                    .all(|c| c[..].as_bytes() == path_type::NonUNCPart);
                prop_assume!(valid_comp);

                let server_share = format!(
                    "{server}{sep}{share}",
                    sep = sep,
                    server = server,
                    share = share
                );
                prop_assume!(
                    server_share.as_bytes() == path_type::ServerShare
                );

                let sep = String::from(sep);
                let prefix = format!(
                    "{sep}{sep}?{sep}UNC{sep}{server}{sep}{share}",
                    sep = sep,
                    server = server,
                    share = share
                );
                let mut path = prefix.clone();
                if comp.len() > 0 {
                    path.push_str(&sep[..]);
                    path.push_str(&comp.join(&sep)[..]);
                }

                let expected_path = prefix.clone();

                // --------------------
                // WHEN
                // --------------------
                // the match_prefix() function is called with the Verbatim
                // path
                let value = match_prefix(&path[..].as_bytes());

                // --------------------
                // THEN
                // --------------------
                // the returned value is not None and
                // the value is a 2-tuple and
                // the value's first element is the end index
                //     of the prefix and
                // the value's second element is a Prefix::VerbatimUNC
                //     containing only the server and share path components
                let result = match value {
                    Some((end, p)) => {
                        let comp_server = OsStr::new(&server[..]);
                        let comp_share = OsStr::new(&share[..]);
                        let expected = Prefix::VerbatimUNC(
                            comp_server,
                            comp_share
                        );
                        &path[..end] == &expected_path[..]
                            && p == expected
                    }
                    _ => false,
                };
                prop_assert!(result);
            }
        }
    }

    mod verbatimdisk {
        use super::*;

        #[test]
        fn simple() {
            // --------------------
            // GIVEN
            // --------------------
            // A byte string with a Prefix::VerbatimDisk prefix

            let pathstr = br#"\\?\C:\hello\world"#;

            // --------------------
            // WHEN
            // --------------------
            // match_prefix is called with the pathstr
            let value = match_prefix(pathstr);

            // --------------------
            // THEN
            // --------------------
            // The returned value is not None and
            // the returned value is a 2-tuple and
            // the 2-tuple's first element is a usize indicating
            //     the end index of the prefix and
            // the 2-tuple's second element is a Prefix::VerbatimDisk
            //     and the Prefix::VerbatimDisk contains the disk letter
            //     following \\?\
            let result = match value {
                None => false,
                Some((index, prefix)) => {
                    let res = &pathstr[..index] == br#"\\?\C:\"#;
                    match prefix {
                        Prefix::VerbatimDisk(drive) => res && drive == b'C',
                        _ => false,
                    }
                }
            };
            assert!(result);
        }

        proptest! {
            #[test]
            fn return_only_drive(
                drive in r#"[a-zA-Z]"#,
                comp in prop::collection::vec(VALID_CHARS, 0..10),
                sep in prop_oneof!(Just("/"), Just(r#"\"#))
            ) {
                // --------------------
                // GIVEN
                // --------------------
                // a valid separator character and
                // a VerbatimDisk path made up of 0 or more valid components

                let valid_comp = comp
                    .iter()
                    .all(|c| c[..].as_bytes() == path_type::NonUNCPart);
                prop_assume!(valid_comp);
                let disk_upper = drive.to_uppercase();

                let sep = String::from(sep);
                let prefix = format!(
                    "{sep}{sep}?{sep}{disk}:{sep}",
                    sep = sep,
                    disk = drive
                );
                let mut path = prefix.clone();
                if comp.len() > 0 {
                    path.push_str(&comp.join(&sep)[..]);
                }

                let expected_path = prefix.clone();

                // --------------------
                // WHEN
                // --------------------
                // the match_prefix() function is called with the Verbatim
                // path
                let value = match_prefix(&path[..].as_bytes());

                // --------------------
                // THEN
                // --------------------
                // the returned value is not None and
                // the value is a 2-tuple and
                // the value's first element is the end index
                //     of the prefix and
                // the value's second element is a Prefix::VerbatimDisk
                //     containing only the disk letter path component
                let result = match value {
                    Some((end, p)) => {
                        let comp_disk = disk_upper.as_bytes()[0];
                        let expected = Prefix::VerbatimDisk(comp_disk);
                        &path[..end] == expected_path
                            && p == expected
                    }
                    _ => false,
                };
                prop_assert!(result);
            }
        }

    }

    mod devicens {
        use super::*;
        use crate::windows::RESERVED_NAMES;

        #[test]
        fn simple() {
            // --------------------
            // GIVEN
            // --------------------
            // A byte string with a Prefix::DeviceNS prefix

            let pathstr = br#"\\.\NUL"#;

            // --------------------
            // WHEN
            // --------------------
            // match_prefix is called with the pathstr
            let value = match_prefix(pathstr);

            // --------------------
            // THEN
            // --------------------
            // The returned value is not None and
            // the returned value is a 2-tuple and
            // the 2-tuple's first element is a usize indicating
            //     the end index of the prefix and
            // the 2-tuple's second element is a Prefix::DeviceNS
            //     and the Prefix::DeviceNS contains the device name
            //     following \\.\
            let result = match value {
                None => false,
                Some((index, prefix)) => {
                    let res = &pathstr[..index] == br#"\\.\NUL"#;
                    match prefix {
                        Prefix::DeviceNS(device) => {
                            res && device == OsStr::new("NUL")
                        }
                        _ => false,
                    }
                }
            };
            assert!(result);
        }

        prop_compose! {
            fn choose_device()(i in 0..RESERVED_NAMES.len()) -> String {
                RESERVED_NAMES.iter().nth(i).unwrap().clone()
            }
        }

        prop_compose! {
            fn choose_devicename()(s in COMP_REGEX) -> String {
                s
            }
        }

        fn choose_devicens() -> BoxedStrategy<String> {
            prop_oneof![choose_devicename(), choose_device()].boxed()
        }

        proptest! {
            #[test]
            fn return_only_device(
                device in choose_devicens(),
                comp in prop::collection::vec(COMP_REGEX, 0..10),
                sep in prop_oneof!(Just("/"), Just(r#"\"#)),
                mk_lower in prop::bool::ANY
            ) {
                // --------------------
                // GIVEN
                // --------------------
                // a valid separator character and
                // a valid DeviceNS path
                let mut device = device;
                if mk_lower {
                    device = device.to_lowercase();
                }

                let sep = String::from(sep);
                let prefix = format!(
                    "{sep}{sep}.{sep}{device}",
                    sep = sep,
                    device = device
                );
                let mut path = prefix.clone();
                if comp.len() > 0 {
                    path.push_str(&sep[..]);
                    path.push_str(&comp.join(&sep)[..]);
                }

                let expected_path = prefix.clone();

                // --------------------
                // WHEN
                // --------------------
                // the match_prefix() function is called with the DeviceNS
                // path
                let value = match_prefix(&path[..].as_bytes());

                // --------------------
                // THEN
                // --------------------
                // the returned value is not None and
                // the value is a 2-tuple and
                // the value's first element is the end index
                //     of the prefix and
                // the value's second element is a Prefix::DeviceNS
                //     containing only the device file name component
                let result = match value {
                    Some((end, p)) => {
                        let comp_device = OsStr::new(device.as_str());
                        let expected = Prefix::DeviceNS(comp_device);
                        &path[..end] == expected_path
                            && p == expected
                    }
                    _ => false,
                };
                prop_assert!(result);
            }
        }
    }

    mod unc {
        use super::*;

        #[test]
        fn simple() {
            // --------------------
            // GIVEN
            // --------------------
            // A byte string with a Prefix::UNC prefix

            let pathstr = br#"\\hello\world"#;

            // --------------------
            // WHEN
            // --------------------
            // match_prefix is called with the pathstr
            let value = match_prefix(pathstr);

            // --------------------
            // THEN
            // --------------------
            // The returned value is not None and
            // the returned value is a 2-tuple and
            // the 2-tuple's first element is a usize indicating
            //     the end index of the prefix and
            // the 2-tuple's second element is a Prefix::UNC
            //     and the Prefix::UNC contains the server and share
            //     components folloing \\
            let result = match value {
                None => false,
                Some((index, prefix)) => {
                    let res = &pathstr[..index] == br#"\\hello\world"#;
                    match prefix {
                        Prefix::UNC(server, share) => {
                            res && server == OsStr::new("hello")
                                && share == OsStr::new("world")
                        }
                        _ => false,
                    }
                }
            };
            assert!(result);
        }

        proptest! {
            #[test]
            fn return_only_server_share(
                server in VALID_CHARS,
                share in VALID_CHARS,
                comp in prop::collection::vec(VALID_CHARS, 0..10),
                sep in prop_oneof!(Just("/"), Just(r#"\"#))
            ) {
                // --------------------
                // GIVEN
                // --------------------
                // a valid separator character and
                // a UNC path made up of 0 or more valid components

                let valid_comp = comp
                    .iter()
                    .all(|c| c[..].as_bytes() == path_type::NonUNCPart);
                prop_assume!(valid_comp);

                let server_share = format!(
                    "{server}{sep}{share}",
                    sep = sep,
                    server = server,
                    share = share
                );
                prop_assume!(
                    server_share.as_bytes() == path_type::ServerShare
                );

                let sep = String::from(sep);
                let prefix = format!(
                    "{sep}{sep}{server}{sep}{share}",
                    sep = sep,
                    server = server,
                    share = share
                );
                let mut path = prefix.clone();
                if comp.len() > 0 {
                    path.push_str(&sep[..]);
                    path.push_str(&comp.join(&sep)[..]);
                }

                let expected_path = prefix.clone();

                // --------------------
                // WHEN
                // --------------------
                // the match_prefix() function is called with the Verbatim
                // path
                let value = match_prefix(&path[..].as_bytes());

                // --------------------
                // THEN
                // --------------------
                // the returned value is not None and
                // the value is a 2-tuple and
                // the value's first element is the end index
                //     of the prefix and
                // the value's second element is a Prefix::UNC
                //     containing only the server and share path components
                let result = match value {
                    Some((end, p)) => {
                        let comp_server = OsStr::new(&server[..]);
                        let comp_share = OsStr::new(&share[..]);
                        let expected = Prefix::UNC(
                            comp_server,
                            comp_share
                        );
                        &path[..end] == &expected_path[..]
                            && p == expected
                    }
                    _ => false,
                };
                prop_assert!(result);
            }
        }
    }

    mod disk {
        use super::*;

        #[test]
        fn simple() {
            // --------------------
            // GIVEN
            // --------------------
            // A byte string with a Prefix::Disk prefix

            let pathstr = br#"C:\hello\world"#;

            // --------------------
            // WHEN
            // --------------------
            // match_prefix is called with the pathstr
            let value = match_prefix(pathstr);

            // --------------------
            // THEN
            // --------------------
            // The returned value is not None and
            // the returned value is a 2-tuple and
            // the 2-tuple's first element is a usize indicating
            //     the end index of the prefix and
            // the 2-tuple's second element is a Prefix::Disk
            //     and the Prefix::Disk contains the disk letter
            let result = match value {
                None => false,
                Some((index, prefix)) => {
                    let res = &pathstr[..index] == br#"C:"#;
                    match prefix {
                        Prefix::Disk(drive) => res && drive == b'C',
                        _ => false,
                    }
                }
            };
            assert!(result);
        }

        proptest! {
            #[test]
            fn return_only_drive(
                drive in r#"[a-zA-Z]"#,
                comp in prop::collection::vec(VALID_CHARS, 0..10),
                sep in prop_oneof!(Just("/"), Just(r#"\"#)),
                include_root in prop::bool::ANY
            ) {
                // --------------------
                // GIVEN
                // --------------------
                // a valid separator character and
                // a Disk path made up of 0 or more valid components

                let valid_comp = comp
                    .iter()
                    .all(|c| c[..].as_bytes() == path_type::NonUNCPart);
                prop_assume!(valid_comp);
                let disk_upper = drive.to_uppercase();

                let sep = String::from(sep);
                let prefix = format!(
                    "{disk}:",
                    disk = drive
                );
                let mut path = prefix.clone();
                if include_root {
                    path.push_str(&sep[..]);
                }

                if comp.len() > 0 {
                    path.push_str(&comp.join(&sep)[..]);
                }

                let expected_path = prefix.clone();

                // --------------------
                // WHEN
                // --------------------
                // the match_prefix() function is called with the Disk
                // path
                let value = match_prefix(&path[..].as_bytes());

                // --------------------
                // THEN
                // --------------------
                // the returned value is not None and
                // the value is a 2-tuple and
                // the value's first element is the end index
                //     of the prefix and
                // the value's second element is a Prefix::Disk
                //     containing only the disk letter path component
                let result = match value {
                    Some((end, p)) => {
                        let comp_disk = disk_upper.as_bytes()[0];
                        let expected = Prefix::Disk(comp_disk);
                        &path[..end] == expected_path
                            && p == expected
                    }
                    _ => false,
                };
                prop_assert!(result);
            }
        }
    }
}

// ===========================================================================
//
// ===========================================================================
