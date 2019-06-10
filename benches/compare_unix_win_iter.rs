// benches/compare_unix_win_iter.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

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
use pathlib::{path::Path, unix::UnixPath, windows::WindowsPath};

// ===========================================================================
// Benchmark
// ===========================================================================

fn unix_iter(path: &str) {
    let path = UnixPath::new(path);
    let _: Vec<_> = path.iter().collect();
}

fn windows_iter(path: &str) {
    let path = WindowsPath::new(path);
    let _: Vec<_> = path.iter().collect();
}

fn bench_path(c: &mut Criterion) {
    c.bench(
        "compare_unix_win_iter",
        ParameterizedBenchmark::new(
            "unix",
            |b, (p, _)| b.iter(|| unix_iter(p)),
            vec![(
                "/hello/world/what/about/now",
                "c:/hello/world/what/about/now",
            )],
        )
        .with_function("windows", |b, (_, p)| b.iter(|| windows_iter(p))),
    );
}

// ===========================================================================
// Main
// ===========================================================================

criterion_group!(benches, bench_path);
criterion_main!(benches);

// ===========================================================================
//
// ===========================================================================
