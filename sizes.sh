#!/bin/sh

# bytecode sizes

echo "adder 1000"
echo "crypto-bubbles 2000"
stat --printf="crowdfunding-erc20 %s\n" contracts/examples/crowdfunding-erc20/output/crowdfunding-erc20.wasm
stat --printf="crowdfunding-esdt %s\n" contracts/examples/crowdfunding-esdt/output/crowdfunding-esdt.wasm
stat --printf="factorial %s\n" contracts/examples/factorial/output/factorial.wasm
stat --printf="lottery-erc20 %s\n" contracts/examples/lottery-erc20/output/lottery-erc20.wasm
stat --printf="lottery-esdt %s\n" contracts/examples/lottery-esdt/output/lottery-esdt.wasm
stat --printf="erc20 %s\n" contracts/examples/simple-erc20/output/simple-erc20.wasm
stat --printf="basic-features %s\n" contracts/feature-tests/basic-features/output/basic-features.wasm
stat --printf="async-alice %s\n" contracts/feature-tests/async/async-alice/output/async-alice.wasm
stat --printf="async-bob %s\n" contracts/feature-tests/async/async-bob/output/async-bob.wasm
stat --printf="use-module %s\n" contracts/feature-tests/use-module/output/use-module.wasm
