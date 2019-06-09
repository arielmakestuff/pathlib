// benches/compare_iter.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.
#![cfg(all(feature = "manual-iter", feature = "parser-iter"))]

// ===========================================================================
// Imports
// ===========================================================================

// --------------------
// Stdlib imports
// --------------------

// --------------------
// Third-party imports
// --------------------
use criterion::{Criterion, ParameterizedBenchmark};

// Macros
use criterion::{criterion_group, criterion_main};

// --------------------
// Local imports
// --------------------
use pathlib::path::{AsSystemStr, Path, SystemStr};
use pathlib::{
    unix::{
        iter::{manual::Iter, parser::Iter as ParserIter},
        UnixPath,
    },
    windows::{
        iter::{manual::Iter as WinIter, parser::Iter as WinParserIter},
        WindowsPath,
    },
};

// ===========================================================================
// Globals
// ===========================================================================

const UNIXPATH: &str = "/hello/world/./what//now/../ya/\x00/";
const WINDOWSPATH: &str = r#"\\?\UNC\server\share\hello\\yep.txt\.\h\nul.txt"#;

// ===========================================================================
// Unix benchmark
// ===========================================================================

struct ParserUnixPath<'path>(&'path UnixPath);

impl<'path> AsSystemStr for ParserUnixPath<'path> {
    fn as_sys_str(&self) -> &SystemStr {
        self.0.as_sys_str()
    }
}

impl<'path> Path<'path, ParserIter<'path>> for ParserUnixPath<'path> {}

fn simple_manual_unixiter() {
    let path = UnixPath::new(UNIXPATH);
    let _: Vec<_> = <&UnixPath as Path<Iter>>::iter(&path).collect();
}

fn simple_parser_unixiter() {
    let path = ParserUnixPath(UnixPath::new(UNIXPATH));
    let _: Vec<_> = <ParserUnixPath as Path<ParserIter>>::iter(&path).collect();
}

fn bench_unixiter(c: &mut Criterion) {
    c.bench(
        "unix_pathiter",
        ParameterizedBenchmark::new(
            "manual",
            |b, _| b.iter(|| simple_manual_unixiter()),
            vec![()],
        )
        .with_function("parser", |b, _| b.iter(|| simple_parser_unixiter())),
    );
}

// ===========================================================================
// Windows benchmark
// ===========================================================================

struct ParserWindowsPath<'path>(&'path WindowsPath);

impl<'path> AsSystemStr for ParserWindowsPath<'path> {
    fn as_sys_str(&self) -> &SystemStr {
        self.0.as_sys_str()
    }
}

impl<'path> Path<'path, WinParserIter<'path>> for ParserWindowsPath<'path> {}

fn simple_manual_winiter() {
    let path = WindowsPath::new(WINDOWSPATH);
    let _: Vec<_> = <&WindowsPath as Path<WinIter>>::iter(&path).collect();
}

fn simple_parser_winiter() {
    let path = ParserWindowsPath(WindowsPath::new(WINDOWSPATH));
    let _: Vec<_> =
        <ParserWindowsPath as Path<WinParserIter>>::iter(&path).collect();
}

fn bench_winiter(c: &mut Criterion) {
    c.bench(
        "win_pathiter",
        ParameterizedBenchmark::new(
            "manual",
            |b, _| b.iter(|| simple_manual_winiter()),
            vec![()],
        )
        .with_function("parser", |b, _| b.iter(|| simple_parser_winiter())),
    );
}

// ===========================================================================
// Main
// ===========================================================================

criterion_group!(benches, bench_unixiter, bench_winiter);
criterion_main!(benches);

// ===========================================================================
//
// ===========================================================================
