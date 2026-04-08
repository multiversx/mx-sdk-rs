#!/bin/sh

# In case you don't have it installed, run
# cargo +stable install cargo-llvm-cov

cargo llvm-cov --html --output-dir tools/coverage
