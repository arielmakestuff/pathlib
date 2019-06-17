// src/windows/parser.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::path::Prefix;

// Third-party imports
use combine::{
    attempt, choice, eof,
    error::{Info, ParseError},
    look_ahead, not_followed_by,
    parser::{
        byte::{byte, bytes, letter},
        range::{range, recognize},
        regex::find,
        Parser,
    },
    range::take_while,
    sep_by,
    stream::{FullRangeStream, RangeStream},
    token, unexpected_any, value,
};
use lazy_static::lazy_static;
use regex::bytes as regex_bytes;

// Local imports
use super::iter::{Component, PrefixComponent};
use super::{RESERVED_NAMES, RESTRICTED_CHARS};
use crate::common::string::{as_osstr, ascii_uppercase};

// ===========================================================================
// Globals
// ===========================================================================

pub(crate) const RESTRICTED_NAME_ERRMSG: &str = "reserved name used";

lazy_static! {
    static ref DEVICE_REGEX: regex_bytes::Regex = {
        let regex = RESERVED_NAMES.iter().fold(String::new(), |mut s, name| {
            if !s.is_empty() {
                s.push_str("|");
            }

            let regex = name.bytes().fold("(?i-u)^".to_owned(), |mut s, b| {
                s.push_str(format!("\\x{:02x}", b).as_str());
                s
            });
            s.push_str(regex.as_str());
            s
        });
        regex_bytes::Regex::new(regex.as_str()).unwrap()
    };
    static ref VALID_NAME_REGEX: regex_bytes::Regex = {
        let regex = RESTRICTED_CHARS.iter().fold(String::new(), |mut s, c| {
            s.push_str(format!("\\x{:02x}", c).as_str());
            s
        });
        let regex = format!("^[^{}]+", regex);
        regex_bytes::Regex::new(regex.as_str()).unwrap()
    };
    static ref UNC_WORD: regex_bytes::Regex =
        { regex_bytes::Regex::new("(?i)^UNC").unwrap() };
}

// ===========================================================================
// General parsers
// ===========================================================================

pub fn separator<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice!(attempt(range(&b"\\"[..])), attempt(range(&b"/"[..])))
}

pub fn path_sep<'a, I>() -> impl Parser<Input = I, Output = ()> + 'a
where
    I: 'a + RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice!(attempt(separator().map(|_| ())), attempt(eof()))
}

pub fn root<'a, I>() -> impl Parser<Input = I, Output = (Component<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    separator().map(|sep| (Component::RootDir(as_osstr(sep)), sep.len()))
}

fn curdir<'a, I>() -> impl Parser<Input = I, Output = (Component<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let sep = choice!(attempt(separator().map(|_| ())), attempt(eof()));
    range(&b"."[..])
        .skip(look_ahead(sep))
        .map(|part: &'a [u8]| (Component::CurDir, part.len()))
}

fn parentdir<'a, I>() -> impl Parser<Input = I, Output = (Component<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let sep = choice!(attempt(separator().map(|_| ())), attempt(eof()));
    range(&b".."[..])
        .skip(look_ahead(sep))
        .map(|part: &'a [u8]| (Component::ParentDir, part.len()))
}

// ===========================================================================
// Utility parsers
// ===========================================================================

fn double_slash<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    separator().then(|_| separator())
}

fn question_slash<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    bytes(b"?").then(|_| separator())
}

fn dot_slash<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    bytes(b".").then(|_| separator())
}

fn device<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    find(&*DEVICE_REGEX)
}

fn device_namespace<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    find(&*VALID_NAME_REGEX)
}

fn unc_part<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    find(&*UNC_WORD)
}

pub fn valid_part_char<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    take_while(|b: u8| !RESTRICTED_CHARS.contains(&b))
}

fn file_parts<'a, I>() -> impl Parser<Input = I, Output = Vec<&'a [u8]>>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    sep_by(
        take_while(|b: u8| b != b'.' && !RESTRICTED_CHARS.contains(&b)),
        token(b'.'),
    )
}

fn file_name<'a, I>(
) -> impl Parser<Input = I, Output = Option<(Vec<u8>, &'a [u8])>> + 'a
where
    I: 'a + RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    file_parts().then(|parts| {
        let parts_len = parts.len();
        if parts.is_empty() || (parts_len == 1 && parts[0].is_empty()) {
            value(None)
        } else if parts_len == 1 {
            let last = *parts.last().unwrap();
            value(Some((last.to_vec(), &[][..])))
        } else {
            let slice = &parts[..parts.len() - 1];
            let last = *parts.last().unwrap();
            let name = slice.join(&b'.');
            value(Some((name, last)))
        }
    })
}

fn nondevice_part<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let sep = choice!(attempt(separator().map(|_| ())), attempt(eof()));
    let part = valid_part_char().skip(look_ahead(sep));

    part.then(|part: &'a [u8]| {
        if part.is_empty() {
            return value(part).left();
        }

        let mut parser = choice!(attempt(parentdir()), attempt(curdir()));
        let res = parser.easy_parse(part);
        if res.is_ok() {
            value(part).left()
        } else {
            let last = *part.last().unwrap();
            match last {
                b' ' | b'.' => {
                    return unexpected_any(Info::Range(part))
                        .message("last character is invalid")
                        .right();
                }
                _ => {}
            }
            let ret = value(part).left();

            // This should always succeed since it has already been successfully
            // parsed
            let mut parser = file_name();
            let file_name = parser.easy_parse(part).unwrap();
            let file_name = file_name.0.unwrap();

            // Fail if the file name matches a reserved name
            let mut parser = device();
            let file_device = parser.parse(&file_name.0[..]);
            match file_device {
                Ok(_) => unexpected_any(Info::Range(part))
                    .message(RESTRICTED_NAME_ERRMSG)
                    .right(),
                Err(_) => ret,
            }
        }
    })
}

fn nonunc_part<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let nomatch = choice!(
        attempt(unc_part().map(|_| 0)),
        attempt(parentdir().map(|_| 0)),
        attempt(curdir().map(|_| 0))
    );
    not_followed_by(nomatch).with(nondevice_part())
}

fn server_share<'a, I>() -> impl Parser<Input = I, Output = (&'a [u8], &'a [u8])>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    nondevice_part().skip(separator()).and(nondevice_part())
}

fn verbatim_start<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(double_slash().and(question_slash()))
}

fn verbatim_unc_start<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(verbatim_start().with(unc_part()).skip(separator()))
}

// ===========================================================================
// Component parsers
// ===========================================================================

fn prefix_verbatim<'a, I>(
) -> impl Parser<Input = I, Output = (Component<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let parser = || verbatim_start().with(nonunc_part());

    look_ahead(recognize(parser())).then(move |prefix| {
        parser().map(move |part| {
            let prefix_kind = Prefix::Verbatim(as_osstr(part));
            let comp =
                Component::Prefix(PrefixComponent::new(prefix, prefix_kind));
            (comp, prefix.len())
        })
    })
}

fn prefix_verbatimunc<'a, I>(
) -> impl Parser<Input = I, Output = (Component<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let parser = || verbatim_unc_start().with(server_share());
    look_ahead(recognize(parser())).then(move |prefix| {
        parser().map(move |(server, share)| {
            let prefix_kind =
                Prefix::VerbatimUNC(as_osstr(server), as_osstr(share));
            let comp =
                Component::Prefix(PrefixComponent::new(prefix, prefix_kind));
            (comp, prefix.len())
        })
    })
}

fn prefix_verbatimdisk<'a, I>(
) -> impl Parser<Input = I, Output = (Component<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let parser = |consume_root| {
        verbatim_start().with(letter().skip(byte(b':')).then(move |l| {
            let ret = value(l);
            if consume_root {
                ret.skip(separator()).left()
            } else {
                ret.skip(look_ahead(separator())).right()
            }
        }))
    };

    look_ahead(recognize(parser(true))).then(move |prefix| {
        parser(false).map(move |disk| {
            let prefix_kind = Prefix::VerbatimDisk(disk);
            let comp =
                Component::Prefix(PrefixComponent::new(prefix, prefix_kind));
            (comp, prefix.len() - 1)
        })
    })
}

fn prefix_devicens<'a, I>(
) -> impl Parser<Input = I, Output = (Component<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let parser = || {
        double_slash()
            .with(dot_slash())
            .with(choice!(attempt(device()), attempt(device_namespace())))
    };

    look_ahead(recognize(parser())).then(move |prefix| {
        parser().map(move |device| {
            let prefix_kind = Prefix::DeviceNS(as_osstr(device));
            let comp =
                Component::Prefix(PrefixComponent::new(prefix, prefix_kind));
            (comp, prefix.len())
        })
    })
}

fn prefix_unc<'a, I>() -> impl Parser<Input = I, Output = (Component<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let parser = || double_slash().with(server_share());
    look_ahead(recognize(parser())).then(move |prefix| {
        parser().map(move |(server, share)| {
            let (server, share) = (as_osstr(server), as_osstr(share));
            let prefix_kind = Prefix::UNC(server, share);
            let comp =
                Component::Prefix(PrefixComponent::new(prefix, prefix_kind));
            (comp, prefix.len())
        })
    })
}

fn prefix_disk<'a, I>(
) -> impl Parser<Input = I, Output = (Component<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(letter().and(byte(b':'))).map(|disk: &'a [u8]| {
        let prefix = Prefix::Disk(ascii_uppercase(disk[0]));
        (Component::Prefix(PrefixComponent::new(disk, prefix)), 2)
    })
}

pub fn prefix<'a, I>() -> impl Parser<Input = I, Output = (Component<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice!(
        attempt(prefix_verbatimunc()),
        attempt(prefix_verbatimdisk()),
        attempt(prefix_verbatim()),
        attempt(prefix_devicens()),
        attempt(prefix_unc()),
        attempt(prefix_disk())
    )
}

pub fn component<'a, I>(
) -> impl Parser<Input = I, Output = (Component<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let sep = choice!(attempt(separator().map(|_| ())), attempt(eof()));
    nondevice_part().skip(sep).map(|comp| {
        if comp.is_empty() {
            (Component::CurDir, 0)
        } else {
            let len = comp.len();
            match comp {
                b"." => (Component::CurDir, len),
                b".." => (Component::ParentDir, len),
                _ => (Component::Normal(as_osstr(comp)), len),
            }
        }
    })
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod test {
    use combine::Parser;

    mod file_name {
        use super::*;
        use crate::windows::parser::file_name;

        #[test]
        fn empty_filename() {
            let name = b"";
            let parse_result = file_name().parse(&name[..]);
            let result = match parse_result {
                Err(_) => false,
                Ok((cur, _)) => cur.is_none(),
            };
            assert!(result);
        }
    }

    mod prefix_verbatimunc {
        use super::*;
        use crate::windows::parser::prefix_verbatimunc;

        #[test]
        fn simple_parse() {
            let path = b"//?/UNC/server/share";
            let parse_result = prefix_verbatimunc().parse(&path[..]);
            let result = match parse_result {
                Err(_) => false,
                Ok(_) => true,
            };
            assert!(result);
        }
    }

    mod prefix_devicens {
        use super::*;
        use crate::windows::iter::{Component, PrefixComponent};
        use crate::windows::parser::prefix_devicens;
        use std::ffi::OsStr;
        use std::path::Prefix;

        #[test]
        fn simple_parse() {
            let path = b"//./COM4";
            let parse_result = prefix_devicens().parse(&path[..]);
            let result = match parse_result {
                Err(_) => false,
                Ok((cur, rest)) => {
                    let eof = rest.is_empty();
                    let prefix_kind = Prefix::DeviceNS(OsStr::new("COM4"));
                    let prefix_comp =
                        PrefixComponent::new(&path[..], prefix_kind.clone());
                    let (comp, len) = cur;
                    let expected_len = len == path.len();
                    match comp {
                        Component::Prefix(c) => {
                            c == prefix_comp && eof && expected_len
                        }
                        _ => false,
                    }
                }
            };
            assert!(result);
        }
    }

    mod prefix_unc {
        use super::*;
        use crate::windows::{
            iter::{Component, PrefixComponent},
            parser::prefix_unc,
        };
        use std::ffi::OsStr;
        use std::path::Prefix;

        #[test]
        fn simple_parse() {
            let path = b"//server/share";
            let parse_result = prefix_unc().parse(&path[..]);
            let result = match parse_result {
                Err(_) => false,
                Ok((cur, rest)) => {
                    let prefix_kind =
                        Prefix::UNC(OsStr::new("server"), OsStr::new("share"));
                    let prefix_comp =
                        PrefixComponent::new(&path[..], prefix_kind.clone());
                    let (comp, len) = cur;
                    match comp {
                        Component::Prefix(c) => {
                            c == prefix_comp
                                && rest.is_empty()
                                && len == path.len()
                        }
                        _ => unreachable!(),
                    }
                }
            };
            assert!(result);
        }
    }

    mod prefix {
        use super::*;
        use crate::windows::parser::prefix;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn simple_parse(path in prop_oneof!(
                    Just("//?/UNC/server/share"),
                    Just("//?/C:/"),
                    Just("//?/hello"),
                    Just("//./COM4"),
                    Just("//server/share"),
                    Just("C:"),
                    Just("C:/")
                )
            )
            {
                let path_str = path.to_owned();
                let path = path_str.as_bytes();
                let parse_result = prefix().parse(&path[..]);
                assert!(parse_result.is_ok());
            }
        }
    }
}

// ===========================================================================
//
// ===========================================================================
