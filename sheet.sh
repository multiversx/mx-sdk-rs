#!/bin/sh

# bytecode sizes


stat --printf="%s\t" contracts/examples/adder/output/adder.wasm
stat --printf="%s\t" contracts/examples/crypto-bubbles/output/crypto-bubbles.wasm
stat --printf="%s\t" contracts/examples/crowdfunding-egld/output/crowdfunding-egld.wasm
stat --printf="%s\t" contracts/examples/crowdfunding-erc20/output/crowdfunding-erc20.wasm
stat --printf="%s\t" contracts/examples/crowdfunding-esdt/output/crowdfunding-esdt.wasm

stat --printf="%s\t" contracts/examples/factorial/output/factorial.wasm

stat --printf="%s\t" contracts/examples/lottery-egld/output/lottery-egld.wasm
stat --printf="%s\t" contracts/examples/lottery-erc20/output/lottery-erc20.wasm
stat --printf="%s\t" contracts/examples/lottery-esdt/output/lottery-esdt.wasm

stat --printf="%s\t" contracts/examples/erc20/output/erc20.wasm

stat --printf="%s\t" contracts/feature-tests/basic-features/output/basic-features.wasm
stat --printf="%s\t" contracts/feature-tests/async/proxy-test-first/output/proxy-test-first.wasm
stat --printf="%s\t" contracts/feature-tests/async/proxy-test-second/output/proxy-test-second.wasm
stat --printf="%s\t" contracts/feature-tests/use-module/output/use-module.wasm
