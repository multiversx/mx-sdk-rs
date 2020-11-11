#!/bin/sh

# helper script for checking that all contracts and debug projects compile

### EXAMPLES ###

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
erdpy --verbose contract build "contracts/examples/simple-erc20"
erdpy --verbose contract build "contracts/feature-tests/basic-features"
erdpy --verbose contract build "contracts/feature-tests/async/async-alice"
erdpy --verbose contract build "contracts/feature-tests/async/async-bob"
erdpy --verbose contract build "contracts/feature-tests/use-module"
