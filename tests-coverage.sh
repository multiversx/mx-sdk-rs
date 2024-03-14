#!/bin/sh
RUSTFLAGS="-C instrument-coverage" \
    cargo test --tests

PROFRAW_FILES=$(find . -name "default_*.profraw")
llvm-profdata merge -sparse $PROFRAW_FILES -o tests.profdata
find . -name "default_*.profraw" -delete

llvm-cov export \
    $( \
      for file in \
        $( \
          RUSTFLAGS="-C instrument-coverage" \
            cargo test --tests --no-run --message-format=json \
              | jq -r "select(.profile.test == true) | .filenames[]" \
              | grep -v dSYM - \
        ); \
      do \
        printf "%s %s " -object $file; \
      done \
    ) \
  --ignore-filename-regex='/.cargo/registry' \
  --ignore-filename-regex='rustc/' \
  --ignore-filename-regex='meta/src' \
  --ignore-filename-regex='wasm-adapter' \
  --ignore-filename-regex='benchmarks/' \
  --ignore-filename-regex='tests/' \
  --instr-profile=tests.profdata --summary-only --format=text > tests.coverage
rm ./tests.profdata

cargo run --bin sc-meta test-coverage-render --input ./tests.coverage --output ./coverage.md
rm ./tests.coverage