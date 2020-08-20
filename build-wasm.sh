#!/bin/sh

# helper script for checking that all contracts and debug projects compile

cd examples/adder/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
mv wasm/target/wasm32-unknown-unknown/release/adder_wasm.wasm output/adder.wasm
cd ../..

# cd crypto-bubbles
# RUSTFLAGS='-C link-arg=-s' \
# cargo build --bin crypto-bubbles --target=wasm32-unknown-unknown --release
# cd ..
# mv target/wasm32-unknown-unknown/release/crypto-bubbles.wasm crypto-bubbles.wasm
# # wasm-snip target/wasm32-unknown-unknown/release/crypto-bubbles.wasm -o crypto-bubbles.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd examples/factorial/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
mv wasm/target/wasm32-unknown-unknown/release/factorial_wasm.wasm output/factorial.wasm
cd ../..

cd test-contracts/basic-features/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
mv wasm/target/wasm32-unknown-unknown/release/basic_features_wasm.wasm output/features.wasm
cd ../..

cd test-contracts/async-alice/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
mv wasm/target/wasm32-unknown-unknown/release/async_bob_wasm.wasm output/bob.wasm
cd ../..

cd test-contracts/async-bob/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
mv wasm/target/wasm32-unknown-unknown/release/async_alice_wasm.wasm output/alice.wasm
cd ../..


# cd simple-coin
# RUSTFLAGS='-C link-arg=-s' \
# cargo build --bin simple-coin --target=wasm32-unknown-unknown --release
# cd ..
# mv target/wasm32-unknown-unknown/release/simple-coin.wasm simple-coin.wasm
# # wasm-snip target/wasm32-unknown-unknown/release/simple-coin.wasm -o simple-coin.wasm --snip-rust-fmt-code --snip-rust-panicking-code
