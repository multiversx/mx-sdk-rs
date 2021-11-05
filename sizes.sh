#!/bin/sh

# bytecode sizes

stat --printf="examples/adder %s\n" contracts/examples/adder/output/adder.wasm
stat --printf="examples/crypto-bubbles %s\n" contracts/examples/crypto-bubbles/output/crypto-bubbles.wasm
stat --printf="examples/crowdfunding-erc20 %s\n" contracts/examples/crowdfunding-erc20/output/crowdfunding-erc20.wasm
stat --printf="examples/crowdfunding-esdt %s\n" contracts/examples/crowdfunding-esdt/output/crowdfunding-esdt.wasm
stat --printf="examples/factorial %s\n" contracts/examples/factorial/output/factorial.wasm
stat --printf="examples/lottery-erc20 %s\n" contracts/examples/lottery-erc20/output/lottery-erc20.wasm
stat --printf="examples/lottery-esdt %s\n" contracts/examples/lottery-esdt/output/lottery-esdt.wasm
stat --printf="examples/erc20 %s\n" contracts/examples/erc20/output/erc20.wasm
stat --printf="examples/multisig %s\n" contracts/examples/multisig/output/multisig.wasm

stat --printf="tests/basic-features %s\n" contracts/feature-tests/basic-features/output/basic-features.wasm
stat --printf="tests/forwarder %s\n" contracts/feature-tests/composability/forwarder/output/forwarder.wasm
stat --printf="tests/forwarder-raw %s\n" contracts/feature-tests/composability/forwarder-raw/output/forwarder-raw.wasm
stat --printf="tests/vault %s\n" contracts/feature-tests/composability/vault/output/vault.wasm
stat --printf="tests/proxy-test-first %s\n" contracts/feature-tests/composability/proxy-test-first/output/proxy-test-first.wasm
stat --printf="tests/proxy-test-second %s\n" contracts/feature-tests/composability/proxy-test-second/output/proxy-test-second.wasm
stat --printf="tests/payable-features %s\n" contracts/feature-tests/payable-features/output/payable-features.wasm
stat --printf="tests/use-module %s\n" contracts/feature-tests/use-module/output/use-module.wasm
