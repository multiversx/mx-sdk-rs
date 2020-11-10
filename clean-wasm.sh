#!/bin/sh

# cleans all wasm targets

cd contracts/examples/adder/wasm
cargo clean
cd ../../../..

cd contracts/examples/crypto-bubbles/wasm
cargo clean
cd ../../../..

cd contracts/examples/factorial/wasm
cargo clean
cd ../../../..

cd contracts/examples/simple-erc20/wasm
cargo clean
cd ../../../..

cd contracts/feature-tests/basic-features/wasm
cargo clean
cd ../../../..

cd contracts/feature-tests/async/async-alice/wasm
cargo clean
cd ../../../../..

cd contracts/feature-tests/async/async-bob/wasm
cargo clean
cd ../../../../..
