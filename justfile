TARGET_DIR = `cargo metadata --no-deps --format-version 1 | jq '.target_directory'`

cov:
    RUSTC=bin/rustc-proptest-fix cargo tarpaulin -v -o Html

kcov:
    #!/usr/bin/bash
    mkdir -p {{TARGET_DIR}}/debug {{TARGET_DIR}}/cov
    rm -rf {{TARGET_DIR}}/debug/pathlib-* {{TARGET_DIR}}/cov/*
    cargo test --no-run
    EXEC_FILE=$(ls {{TARGET_DIR}}/debug/pathlib-* | sed '/[.]d$/d')
    kcov --include-path src --exclude-path src/test {{TARGET_DIR}}/cov $EXEC_FILE
