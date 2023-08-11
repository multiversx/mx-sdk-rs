#!/bin/bash

# copies wasm & scenarios files to the Arwen test folder
# expects 1 argument: the path to the Arwen repo root

VM_REPO_PATH=${1:?"Missing VM repo path!"}
TARGET_DIR=$PWD/target

build_and_copy() {
   contract_path=$1
   contract_name=${contract_path##*/}
   vm_contract_path=$2

   sc-meta all build --target-dir $TARGET_DIR --path $contract_path || return 1

   mkdir -p $vm_contract_path/output
   cp $contract_path/output/*.wasm \
      $vm_contract_path/output
}

build_and_copy_with_scenarios() {
   contract_path=$1
   contract_name=${contract_path##*/}
   vm_contract_path=$2

   sc-meta all build --target-dir $TARGET_DIR --path $contract_path || return 1
   mkdir -p $vm_contract_path/output
   rm -rf $vm_contract_path/scenarios
   cp $contract_path/output/*.wasm \
      $vm_contract_path/output
   cp -R $contract_path/scenarios \
      $vm_contract_path
}

# building all contracts takes a lot of time, only the ones for the wasm-vm tests are built below
# if you still want to build all:
# ./build-wasm.sh

build_and_copy_with_scenarios ./contracts/core/wegld-swap $VM_REPO_PATH/test/wegld-swap
build_and_copy_with_scenarios ./contracts/examples/adder $VM_REPO_PATH/test/adder
build_and_copy_with_scenarios ./contracts/examples/crowdfunding-esdt $VM_REPO_PATH/test/crowdfunding-esdt
build_and_copy_with_scenarios ./contracts/examples/digital-cash $VM_REPO_PATH/test/digital-cash
build_and_copy_with_scenarios ./contracts/examples/factorial $VM_REPO_PATH/test/factorial
build_and_copy_with_scenarios ./contracts/examples/ping-pong-egld $VM_REPO_PATH/test/ping-pong-egld
build_and_copy_with_scenarios ./contracts/examples/multisig $VM_REPO_PATH/test/multisig
build_and_copy_with_scenarios ./contracts/feature-tests/alloc-features $VM_REPO_PATH/test/features/alloc-features
build_and_copy_with_scenarios ./contracts/feature-tests/basic-features $VM_REPO_PATH/test/features/basic-features
build_and_copy_with_scenarios ./contracts/feature-tests/big-float-features $VM_REPO_PATH/test/features/big-float-features
build_and_copy_with_scenarios ./contracts/feature-tests/erc-style-contracts/erc20 $VM_REPO_PATH/test/erc20-rust
build_and_copy_with_scenarios ./contracts/feature-tests/formatted-message-features $VM_REPO_PATH/test/features/formatted-message-features
build_and_copy_with_scenarios ./contracts/feature-tests/payable-features $VM_REPO_PATH/test/features/payable-features
build_and_copy_with_scenarios ./contracts/feature-tests/esdt-system-sc-mock $VM_REPO_PATH/test/features/esdt-system-sc-mock

build_and_copy_composability() {
   contract=$1
   contract_with_underscores="${contract//-/_}"

   sc-meta all build --target-dir $TARGET_DIR --path ./contracts/feature-tests/composability/$contract || return 1
   cp -R contracts/feature-tests/composability/$contract/output/${contract}.wasm \
      $VM_REPO_PATH/test/features/composability/$contract/output/${contract}.wasm
}

build_and_copy ./contracts/feature-tests/composability/forwarder         $VM_REPO_PATH/test/features/composability/forwarder
build_and_copy ./contracts/feature-tests/composability/forwarder-raw     $VM_REPO_PATH/test/features/composability/forwarder-raw
build_and_copy ./contracts/feature-tests/composability/proxy-test-first  $VM_REPO_PATH/test/features/composability/proxy-test-first
build_and_copy ./contracts/feature-tests/composability/proxy-test-second $VM_REPO_PATH/test/features/composability/proxy-test-second
build_and_copy ./contracts/feature-tests/composability/recursive-caller  $VM_REPO_PATH/test/features/composability/recursive-caller
build_and_copy ./contracts/feature-tests/composability/promises-features $VM_REPO_PATH/test/features/composability/promises-features
build_and_copy ./contracts/feature-tests/composability/vault             $VM_REPO_PATH/test/features/composability/vault

rm -f $VM_REPO_PATH/test/features/composability/scenarios/*
cp -R contracts/feature-tests/composability/scenarios \
   $VM_REPO_PATH/test/features/composability
cp -R contracts/feature-tests/composability/scenarios-promises \
   $VM_REPO_PATH/test/features/composability
