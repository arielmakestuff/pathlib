// benches/compare_stdpath.rs
// Copyright (C) 2019 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// --------------------
// Stdlib imports
// --------------------
use std::path::Path as StdPath;

// --------------------
// Third-party imports
// --------------------
use criterion::{Criterion, ParameterizedBenchmark};

// Macros
use criterion::{criterion_group, criterion_main};

// --------------------
// Local imports
// --------------------
use pathlib::path::Path;

// ===========================================================================
// Unix benchmark
// ===========================================================================

#[cfg(unix)]
mod pathiter {
    use super::*;
    use pathlib::unix::UnixPath;

    fn simple_std_iter(path: &str) {
        let path = StdPath::new(path);
        let _: Vec<_> = path.components().collect();
    }

    fn simple_pathlib_iter(path: &str) {
        let path = UnixPath::new(path);
        let _: Vec<_> = path.iter().collect();
    }

    pub(super) fn bench_path(c: &mut Criterion) {
        c.bench(
            "compare_iter",
            ParameterizedBenchmark::new(
                "std",
                |b, p| b.iter(|| simple_std_iter(p)),
                vec![
                    "/hello/world/what/about/now",
                    // "/hello/world/./what//now/../ya/./../",
                ],
            )
            .with_function("pathlib", |b, p| b.iter(|| simple_pathlib_iter(p))),
        );
    }
}

#[cfg(windows)]
mod pathiter {
    use super::*;
    use pathlib::windows::WindowsPath;

    fn simple_std_iter(path: &str) {
        let path = StdPath::new(path);
        let _: Vec<_> = path.components().collect();
    }

    fn simple_pathlib_iter(path: &str) {
        let path = WindowsPath::new(path);
        let _: Vec<_> = path.iter().collect();
    }

    pub(super) fn bench_path(c: &mut Criterion) {
        c.bench(
            "compare_iter",
            ParameterizedBenchmark::new(
                "std",
                |b, p| b.iter(|| simple_std_iter(p)),
                vec![
                    r#"\\?\UNC\server\share\hello\world\what\about\now"#,
                    // r#"\\?\UNC\server\share\hello\\yep.txt\.\h"#,
                ],
            )
            .with_function("pathlib", |b, p| b.iter(|| simple_pathlib_iter(p))),
        );
    }
}

// ===========================================================================
// Main
// ===========================================================================

use pathiter::bench_path;

criterion_group!(benches, bench_path);
criterion_main!(benches);

// ===========================================================================
//
// ===========================================================================
