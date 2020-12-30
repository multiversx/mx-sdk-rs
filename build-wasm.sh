#!/bin/sh

# helper script for checking that all contracts and debug projects compile

### BUILD ALL CONTRACTS ###

erdpy --verbose contract build "contracts/benchmarks/str-repeat"
erdpy --verbose contract build "contracts/examples/adder"
erdpy --verbose contract build "contracts/examples/crowdfunding-egld"
erdpy --verbose contract build "contracts/examples/crowdfunding-erc20"
erdpy --verbose contract build "contracts/examples/crowdfunding-esdt"
erdpy --verbose contract build "contracts/examples/crypto-bubbles"
erdpy --verbose contract build "contracts/examples/factorial"
erdpy --verbose contract build "contracts/examples/lottery-egld"
erdpy --verbose contract build "contracts/examples/lottery-erc20"
erdpy --verbose contract build "contracts/examples/lottery-esdt"
erdpy --verbose contract build "contracts/examples/multisig"
erdpy --verbose contract build "contracts/examples/non-fungible-tokens"
erdpy --verbose contract build "contracts/examples/simple-erc20"
erdpy --verbose contract build "contracts/feature-tests/abi-tester"
erdpy --verbose contract build "contracts/feature-tests/basic-features"
erdpy --verbose contract build "contracts/feature-tests/async/async-alice"
erdpy --verbose contract build "contracts/feature-tests/async/async-bob"
erdpy --verbose contract build "contracts/feature-tests/use-module"

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
./abi.sh "contracts/examples/simple-erc20"
./abi.sh "contracts/feature-tests/abi-tester"
./abi.sh "contracts/feature-tests/basic-features"
./abi.sh "contracts/feature-tests/use-module"
