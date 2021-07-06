#!/bin/sh

export PATH=$HOME/elrondsdk/arwentools:$PATH
cargo test --features elrond-wasm-debug/arwen-tests

