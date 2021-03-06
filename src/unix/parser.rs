// src/unix/parser.rs
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
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports
use combine::{
    choice,
    easy::Errors,
    eof,
    error::ParseError,
    look_ahead, not_followed_by,
    parser::{byte::byte, range::range, regex::find, Parser},
    stream::{FullRangeStream, PointerOffset, RangeStream},
    value,
};
use lazy_static::lazy_static;
use regex::bytes;

// Local imports
use super::{iter::Component, UnixErrorKind};
use crate::common::{error, string::as_osstr};

// ===========================================================================
// Globals
// ===========================================================================

lazy_static! {
    static ref PATH_COMPONENT: bytes::Regex =
        bytes::Regex::new(r"[^\x00\x2f]*").unwrap();
    static ref SIMPLE_COMPONENT: bytes::Regex =
        bytes::Regex::new(r"[^\x2f]*").unwrap();
}

// ===========================================================================
// Types
// ===========================================================================

pub type PathComponent<'path> = Result<Component<'path>, error::ParseError>;

// ===========================================================================
// Error Handling
// ===========================================================================

fn simple_component<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    find(&*SIMPLE_COMPONENT)
}

pub fn into_error<I, R>(
    path: &[u8],
    start: usize,
    parse_error: Errors<I, R, PointerOffset>,
) -> error::ParseError {
    let kind = error::ParseErrorKind::Unix(UnixErrorKind::InvalidCharacter);
    let path_comp = &path[start..];

    let err = parse_error.map_position(|p| p.translate_position(path_comp));
    let err_position = err.position;
    let msg = format!(
        "Parse error at position {}: found null character",
        start + err_position
    );

    // the returned tuple is (found, rest) where found is the part of the input
    // that matches and the rest is the remaining part of the input that's
    // unparsed
    let rest = simple_component()
        .parse(path_comp)
        .expect("should not fail")
        .0;
    let comp = [&path[..start], rest].concat();
    let end = start + rest.len();

    error::ParseError::new(
        kind,
        as_osstr(&comp[..]).into(),
        as_osstr(path).into(),
        start,
        end,
        msg,
    )
}

// ===========================================================================
// Parser
// ===========================================================================

fn null_byte<'a, I>() -> impl Parser<Input = I, Output = u8>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    byte(b'\x00')
}

fn sep_byte<'a, I>() -> impl Parser<Input = I, Output = u8>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    byte(b'/')
}

pub fn root<'a, I>(
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    sep_byte().map(|_| (Ok(Component::RootDir), 1))
}

fn curdir<'a, I>() -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    range(&b"."[..])
        .then(|_| eof().or(sep_byte().map(|_| ())))
        .map(|_| (Ok(Component::CurDir), 1))
}

fn parentdir<'a, I>(
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    range(&b".."[..])
        .then(|_| eof().or(sep_byte().map(|_| ())))
        .map(|_| (Ok(Component::ParentDir), 2))
}

pub fn component<'a, I>(
) -> impl Parser<Input = I, Output = (PathComponent<'a>, usize)>
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let comp = find(&*PATH_COMPONENT)
        .then(|comp: &'a [u8]| not_followed_by(null_byte()).with(value(comp)))
        .map(|comp| {
            if comp.is_empty() {
                (Ok(Component::CurDir), 0)
            } else {
                (Ok(Component::Normal(as_osstr(comp))), comp.len())
            }
        });
    let comp_option = (look_ahead(parentdir()), look_ahead(curdir()), comp);
    choice(comp_option)
}

// ===========================================================================
//
// ===========================================================================
