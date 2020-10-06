#!/bin/sh

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests"
export RUSTDOCFLAGS="-Cpanic=abort"

cargo build
cargo test

grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/

## For playing around with lcov later:
# grcov ./target/debug/ -s . -t lcov --llvm --branch --ignore-not-existing -o ./target/debug/lcov.info
