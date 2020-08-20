#!/bin/sh

# cleans all wasm targets

cd examples/adder/wasm
cargo clean
cd ../../..

cd examples/factorial/wasm
cargo clean
cd ../../..

cd examples/simple-erc20/wasm
cargo clean
cd ../../..

cd test-contracts/basic-features/wasm
cargo clean
cd ../../..

cd test-contracts/async-alice/wasm
cargo clean
cd ../../..

cd test-contracts/async-bob/wasm
cargo clean
cd ../../..
