#!/bin/sh

# cleans all wasm targets

erdpy --verbose contract clean "contracts/benchmarks/str-repeat"
erdpy --verbose contract clean "contracts/benchmarks/send-tx-repeat"
erdpy --verbose contract clean "contracts/examples/adder"
erdpy --verbose contract clean "contracts/examples/crypto-bubbles"
erdpy --verbose contract clean "contracts/examples/crypto-kitties/kitty-ownership"
erdpy --verbose contract clean "contracts/examples/crypto-kitties/kitty-genetic-alg"
erdpy --verbose contract clean "contracts/examples/crypto-kitties/kitty-auction"
erdpy --verbose contract clean "contracts/examples/crowdfunding-egld"
erdpy --verbose contract clean "contracts/examples/crowdfunding-erc20"
erdpy --verbose contract clean "contracts/examples/crowdfunding-esdt"
erdpy --verbose contract clean "contracts/examples/erc1155"
erdpy --verbose contract clean "contracts/examples/erc1155-user-mock"
erdpy --verbose contract clean "contracts/examples/factorial"
erdpy --verbose contract clean "contracts/examples/lottery-egld"
erdpy --verbose contract clean "contracts/examples/lottery-erc20"
erdpy --verbose contract clean "contracts/examples/lottery-esdt"
erdpy --verbose contract clean "contracts/examples/multisig"
erdpy --verbose contract clean "contracts/examples/non-fungible-tokens"
erdpy --verbose contract clean "contracts/examples/ping-pong-egld"
erdpy --verbose contract clean "contracts/examples/simple-erc20"
erdpy --verbose contract clean "contracts/feature-tests/abi-tester"
erdpy --verbose contract clean "contracts/feature-tests/async/async-alice"
erdpy --verbose contract clean "contracts/feature-tests/async/async-bob"
erdpy --verbose contract clean "contracts/feature-tests/async/forwarder"
erdpy --verbose contract clean "contracts/feature-tests/async/forwarder-raw"
erdpy --verbose contract clean "contracts/feature-tests/async/vault"
erdpy --verbose contract clean "contracts/feature-tests/basic-features"
erdpy --verbose contract clean "contracts/feature-tests/esdt-contract-pair/first-contract"
erdpy --verbose contract clean "contracts/feature-tests/esdt-contract-pair/second-contract"
erdpy --verbose contract clean "contracts/feature-tests/deploy-two-contracts"
erdpy --verbose contract clean "contracts/feature-tests/use-module"


# not wasm, but worth cleaning from time to time

cargo clean
cd elrond-wasm-node
cargo clean
cd ..
cd elrond-wasm-output
cargo clean
cd ..
