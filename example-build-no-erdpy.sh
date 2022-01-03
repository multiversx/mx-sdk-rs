#!/bin/sh

# alternative to erdpy

export RUSTFLAGS=${RUSTFLAGS-'-C link-arg=-s'}

cd contracts/examples/adder/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/adder_wasm.wasm output/adder.wasm
cd ..
