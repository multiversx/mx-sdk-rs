#!/bin/sh

# helper script for checking that all contracts and debug projects compile

erdpy --verbose contract build "contracts/benchmarks/str-repeat" || return 1
erdpy --verbose contract build "contracts/benchmarks/send-tx-repeat" || return 1
erdpy --verbose contract build "contracts/examples/adder" || return 1
erdpy --verbose contract build "contracts/examples/crypto-bubbles" || return 1
erdpy --verbose contract build "contracts/examples/crypto-kitties/kitty-ownership" || return 1
erdpy --verbose contract build "contracts/examples/crypto-kitties/kitty-genetic-alg" || return 1
erdpy --verbose contract build "contracts/examples/crypto-kitties/kitty-auction" || return 1
erdpy --verbose contract build "contracts/examples/crowdfunding-egld" || return 1
erdpy --verbose contract build "contracts/examples/crowdfunding-erc20" || return 1
erdpy --verbose contract build "contracts/examples/crowdfunding-esdt" || return 1
erdpy --verbose contract build "contracts/examples/erc1155" || return 1
erdpy --verbose contract build "contracts/examples/erc1155-user-mock" || return 1
erdpy --verbose contract build "contracts/examples/factorial" || return 1
erdpy --verbose contract build "contracts/examples/lottery-egld" || return 1
erdpy --verbose contract build "contracts/examples/lottery-erc20" || return 1
erdpy --verbose contract build "contracts/examples/lottery-esdt" || return 1
erdpy --verbose contract build "contracts/examples/multisig" || return 1
erdpy --verbose contract build "contracts/examples/non-fungible-tokens" || return 1
erdpy --verbose contract build "contracts/examples/ping-pong-egld" || return 1
erdpy --verbose contract build "contracts/examples/simple-erc20" || return 1
erdpy --verbose contract build "contracts/feature-tests/abi-tester" || return 1
erdpy --verbose contract build "contracts/feature-tests/async/async-alice" || return 1
erdpy --verbose contract build "contracts/feature-tests/async/async-bob" || return 1
erdpy --verbose contract build "contracts/feature-tests/async/forwarder" || return 1
erdpy --verbose contract build "contracts/feature-tests/async/forwarder-raw" || return 1
erdpy --verbose contract build "contracts/feature-tests/async/vault" || return 1
erdpy --verbose contract build "contracts/feature-tests/basic-features" || return 1
erdpy --verbose contract build "contracts/feature-tests/esdt-contract-pair/first-contract" || return 1
erdpy --verbose contract build "contracts/feature-tests/esdt-contract-pair/second-contract" || return 1
erdpy --verbose contract build "contracts/feature-tests/deploy-two-contracts" || return 1
erdpy --verbose contract build "contracts/feature-tests/use-module" || return 1
