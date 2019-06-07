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
use super::match_prefix::ascii_uppercase;
use super::{RESERVED_NAMES, RESTRICTED_CHARS};
use crate::common::error;
use crate::common::string::as_osstr;

// ===========================================================================
// Globals
// ===========================================================================

pub(crate) const RESTRICTED_NAME_ERRMSG: &str = "reserved name used";

lazy_static! {
    static ref PATH_COMPONENT: regex_bytes::Regex =
        regex_bytes::Regex::new(r"[^\x00\x2f]*").unwrap();
    static ref DEVICE_REGEX: regex_bytes::Regex = {
        let regex = RESERVED_NAMES.iter().fold(String::new(), |mut s, name| {
            if !s.is_empty() {
                s.push_str("|");
            }

            let regex = name.bytes().fold("(?i-u)".to_owned(), |mut s, b| {
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
        let regex = format!("[^{}]+", regex);
        regex_bytes::Regex::new(regex.as_str()).unwrap()
    };
    static ref UNC_WORD: regex_bytes::Regex =
        { regex_bytes::Regex::new("(?i)UNC").unwrap() };
    static ref SEP_REGEX: regex_bytes::Regex = {
        let pattern = format!("[\\x{:02x}\\x{:02x}]", b'\\', b'/');
        regex_bytes::Regex::new(pattern.as_str()).unwrap()
    };
}

// ===========================================================================
// Types
// ===========================================================================

pub type PathComponent<'path> = Result<Component<'path>, error::ParseError>;

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

pub fn root<'a, I>(
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    separator().map(|sep| (Ok(Component::RootDir(as_osstr(sep))), sep.len()))
}

fn curdir<'a, I>() -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let sep = choice!(attempt(separator().map(|_| ())), attempt(eof()));
    range(&b"."[..])
        .skip(look_ahead(sep))
        .map(|part: &'a [u8]| (Ok(Component::CurDir), part.len()))
}

fn parentdir<'a, I>(
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let sep = choice!(attempt(separator().map(|_| ())), attempt(eof()));
    range(&b".."[..])
        .skip(look_ahead(sep))
        .map(|part: &'a [u8]| (Ok(Component::ParentDir), part.len()))
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
        if parts.is_empty() {
            value(None)
        } else if parts.len() == 1 {
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
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
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
            (Ok(comp), prefix.len())
        })
    })
}

fn prefix_verbatimunc<'a, I>(
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
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
            (Ok(comp), prefix.len())
        })
    })
}

fn prefix_verbatimdisk<'a, I>(
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
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
            (Ok(comp), prefix.len() - 1)
        })
    })
}

fn prefix_devicens<'a, I>(
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
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
            (Ok(comp), prefix.len())
        })
    })
}

fn prefix_unc<'a, I>(
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
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
            (Ok(comp), prefix.len())
        })
    })
}

fn prefix_disk<'a, I>(
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(letter().and(byte(b':'))).map(|disk: &'a [u8]| {
        let prefix = Prefix::Disk(ascii_uppercase(disk[0]));
        (Ok(Component::Prefix(PrefixComponent::new(disk, prefix))), 2)
    })
}

pub fn prefix<'a, I>(
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
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
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let sep = choice!(attempt(separator().map(|_| ())), attempt(eof()));
    nondevice_part().skip(sep).map(|comp| {
        if comp.is_empty() {
            (Ok(Component::CurDir), 0)
        } else {
            let len = comp.len();
            match comp {
                b"." => (Ok(Component::CurDir), len),
                b".." => (Ok(Component::ParentDir), len),
                _ => (Ok(Component::Normal(as_osstr(comp))), len),
            }
        }
    })
}

// ===========================================================================
//
// ===========================================================================
