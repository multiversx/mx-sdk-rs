#!/bin/sh

# helper script for checking that all contracts and debug projects compile

### BUILD ALL CONTRACTS ###

erdpy --verbose contract build "contracts/benchmarks/str-repeat" || return 1
erdpy --verbose contract build "contracts/examples/adder" || return 1
erdpy --verbose contract build "contracts/examples/crowdfunding-egld" || return 1
erdpy --verbose contract build "contracts/examples/crowdfunding-erc20" || return 1
erdpy --verbose contract build "contracts/examples/crowdfunding-esdt" || return 1
erdpy --verbose contract build "contracts/examples/crypto-bubbles" || return 1
erdpy --verbose contract build "contracts/examples/factorial" || return 1
erdpy --verbose contract build "contracts/examples/lottery-egld" || return 1
erdpy --verbose contract build "contracts/examples/lottery-erc20" || return 1
erdpy --verbose contract build "contracts/examples/lottery-esdt" || return 1
erdpy --verbose contract build "contracts/examples/multisig" || return 1
erdpy --verbose contract build "contracts/examples/non-fungible-tokens" || return 1
erdpy --verbose contract build "contracts/examples/ping-pong-egld" || return 1
erdpy --verbose contract build "contracts/examples/simple-erc20" || return 1
erdpy --verbose contract build "contracts/feature-tests/abi-tester" || return 1
erdpy --verbose contract build "contracts/feature-tests/basic-features" || return 1
erdpy --verbose contract build "contracts/feature-tests/async/async-alice" || return 1
erdpy --verbose contract build "contracts/feature-tests/async/async-bob" || return 1
erdpy --verbose contract build "contracts/feature-tests/use-module" || return 1

### CREATE ALL ABIs ###

./abi.sh "contracts/examples/adder"
./abi.sh "contracts/examples/crowdfunding-egld"
./abi.sh "contracts/examples/crowdfunding-erc20"
./abi.sh "contracts/examples/crowdfunding-esdt"
./abi.sh "contracts/examples/crypto-bubbles"
./abi.sh "contracts/examples/factorial"
./abi.sh "contracts/examples/lottery-egld"
./abi.sh "contracts/examples/lottery-erc20"
./abi.sh "contracts/examples/lottery-esdt"
./abi.sh "contracts/examples/multisig"
./abi.sh "contracts/examples/non-fungible-tokens"
./abi.sh "contracts/examples/ping-pong-egld"
./abi.sh "contracts/examples/simple-erc20"
./abi.sh "contracts/feature-tests/abi-tester"
./abi.sh "contracts/feature-tests/basic-features"
./abi.sh "contracts/feature-tests/use-module"
