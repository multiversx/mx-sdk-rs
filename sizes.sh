#!/bin/sh

# bytecode sizes

stat --printf="adder %s\n" contracts/examples/adder/output/adder.wasm
stat --printf="crypto-bubbles %s\n" contracts/examples/crypto-bubbles/output/crypto-bubbles.wasm
stat --printf="crowdfunding-erc20 %s\n" contracts/examples/crowdfunding-erc20/output/crowdfunding-erc20.wasm
stat --printf="crowdfunding-esdt %s\n" contracts/examples/crowdfunding-esdt/output/crowdfunding-esdt.wasm
stat --printf="factorial %s\n" contracts/examples/factorial/output/factorial.wasm
stat --printf="lottery-erc20 %s\n" contracts/examples/lottery-erc20/output/lottery-erc20.wasm
stat --printf="lottery-esdt %s\n" contracts/examples/lottery-esdt/output/lottery-esdt.wasm
stat --printf="erc20 %s\n" contracts/examples/erc20/output/erc20.wasm
stat --printf="multisig %s\n" contracts/examples/multisig/output/multisig.wasm
stat --printf="basic-features %s\n" contracts/feature-tests/basic-features/output/basic-features.wasm
stat --printf="async-alice %s\n" contracts/feature-tests/composability/proxy-test-first/output/proxy-test-first.wasm
stat --printf="async-bob %s\n" contracts/feature-tests/composability/proxy-test-second/output/proxy-test-second.wasm
stat --printf="use-module %s\n" contracts/feature-tests/use-module/output/use-module.wasm
