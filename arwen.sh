#!/bin/sh

# copies wasm & mandos files to the Arwen test folder
# expects 1 argument: the path to the Arwen repo root

ARWEN_PATH=$1

# building all contracts takes a lot of time, here are just the ones for Arwen:
erdpy --verbose contract build ./contracts/examples/adder || return 1
erdpy --verbose contract build ./contracts/examples/crowdfunding-esdt || return 1
erdpy --verbose contract build ./contracts/examples/ping-pong-egld || return 1
erdpy --verbose contract build ./contracts/examples/multisig || return 1
erdpy --verbose contract build ./contracts/examples/egld-esdt-swap || return 1
erdpy --verbose contract build ./contracts/examples/erc20 || return 1
erdpy --verbose contract build ./contracts/feature-tests/basic-features || return 1
erdpy --verbose contract build ./contracts/feature-tests/composability/forwarder || return 1
erdpy --verbose contract build ./contracts/feature-tests/composability/forwarder-raw || return 1
erdpy --verbose contract build ./contracts/feature-tests/composability/proxy-test-first || return 1
erdpy --verbose contract build ./contracts/feature-tests/composability/proxy-test-second || return 1
erdpy --verbose contract build ./contracts/feature-tests/composability/recursive-caller || return 1
erdpy --verbose contract build ./contracts/feature-tests/composability/vault || return 1
erdpy --verbose contract build ./contracts/feature-tests/payable-features || return 1

# if you still want to build all:
# ./build-wasm.sh


# copying the files to arwen here:
cp contracts/examples/adder/output/adder.wasm \
   $ARWEN_PATH/test/adder/output/adder.wasm
cp -R contracts/examples/adder/mandos \
   $ARWEN_PATH/test/adder

cp contracts/examples/crowdfunding-esdt/output/crowdfunding-esdt.wasm \
   $ARWEN_PATH/test/crowdfunding-esdt/output/crowdfunding-esdt.wasm
cp -R contracts/examples/crowdfunding-esdt/mandos \
   $ARWEN_PATH/test/crowdfunding-esdt

cp contracts/examples/ping-pong-egld/output/ping-pong-egld.wasm \
   $ARWEN_PATH/test/ping-pong-egld/output/ping-pong-egld.wasm
cp -R contracts/examples/ping-pong-egld/mandos \
   $ARWEN_PATH/test/ping-pong-egld

cp contracts/examples/multisig/output/multisig.wasm \
   $ARWEN_PATH/test/multisig/output/multisig.wasm
cp -R contracts/examples/multisig/mandos \
   $ARWEN_PATH/test/multisig
cp -R contracts/examples/multisig/test-contracts \
   $ARWEN_PATH/test/multisig

cp -R contracts/examples/egld-esdt-swap/output/egld-esdt-swap.wasm \
   $ARWEN_PATH/test/egld-esdt-swap/output/egld-esdt-swap.wasm
cp -R contracts/examples/egld-esdt-swap/mandos \
   $ARWEN_PATH/test/egld-esdt-swap

cp -R contracts/examples/erc20/output/erc20.wasm \
   $ARWEN_PATH/test/erc20-rust/output/erc20.wasm
cp -R contracts/examples/erc20/mandos \
   $ARWEN_PATH/test/erc20-rust

cp -R contracts/feature-tests/basic-features/output/basic-features.wasm \
   $ARWEN_PATH/test/features/basic-features/output/basic-features.wasm
cp -R contracts/feature-tests/basic-features/mandos \
   $ARWEN_PATH/test/features/basic-features

cp -R contracts/feature-tests/payable-features/output/payable-features.wasm \
   $ARWEN_PATH/test/features/payable-features/output/payable-features.wasm
cp -R contracts/feature-tests/payable-features/mandos \
   $ARWEN_PATH/test/features/payable-features

cp -R contracts/feature-tests/composability/forwarder/output/forwarder.wasm \
   $ARWEN_PATH/test/features/composability/forwarder/output/forwarder.wasm
cp -R contracts/feature-tests/composability/forwarder-raw/output/forwarder-raw.wasm \
   $ARWEN_PATH/test/features/composability/forwarder-raw/output/forwarder-raw.wasm
cp -R contracts/feature-tests/composability/proxy-test-first/output/proxy-test-first.wasm \
   $ARWEN_PATH/test/features/composability/proxy-test-first/output/proxy-test-first.wasm
cp -R contracts/feature-tests/composability/proxy-test-second/output/proxy-test-second.wasm \
   $ARWEN_PATH/test/features/composability/proxy-test-second/output/proxy-test-second.wasm
cp -R contracts/feature-tests/composability/recursive-caller/output/recursive-caller.wasm \
   $ARWEN_PATH/test/features/composability/recursive-caller/output/recursive-caller.wasm
cp -R contracts/feature-tests/composability/vault/output/vault.wasm \
   $ARWEN_PATH/test/features/composability/vault/output/vault.wasm
cp -R contracts/feature-tests/composability/mandos \
   $ARWEN_PATH/test/features/composability
