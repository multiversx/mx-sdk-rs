#!/bin/bash

# copies wasm & scenarios files to the Arwen test folder
# expects 1 argument: the path to the Arwen repo root

VM_REPO_PATH=${1:?"Missing VM repo path!"}

build_and_copy() {
   contract_path=$1
   contract_name=${contract_path##*/}
   vm_contract_path=$2

   # mxpy --verbose contract build --skip-eei-checks $contract_path || return 1
   mkdir -p $vm_contract_path/output
   rm -rf $vm_contract_path/mandos
   cp $contract_path/output/*.wasm \
      $vm_contract_path/output
   cp -R $contract_path/scenarios \
      $vm_contract_path
}

# building all contracts takes a lot of time, only the ones for the wasm-vm tests are built below
# if you still want to build all:
# ./build-wasm.sh

build_and_copy ./contracts/core/wegld-swap $VM_REPO_PATH/test/wegld-swap
build_and_copy ./contracts/examples/adder $VM_REPO_PATH/test/adder
build_and_copy ./contracts/examples/crowdfunding-esdt $VM_REPO_PATH/test/crowdfunding-esdt
build_and_copy ./contracts/examples/digital-cash $VM_REPO_PATH/test/digital-cash
build_and_copy ./contracts/examples/factorial $VM_REPO_PATH/test/factorial
build_and_copy ./contracts/examples/ping-pong-egld $VM_REPO_PATH/test/ping-pong-egld
build_and_copy ./contracts/examples/multisig $VM_REPO_PATH/test/multisig
build_and_copy ./contracts/feature-tests/alloc-features $VM_REPO_PATH/test/features/alloc-features
build_and_copy ./contracts/feature-tests/basic-features $VM_REPO_PATH/test/features/basic-features
build_and_copy ./contracts/feature-tests/big-float-features $VM_REPO_PATH/test/features/big-float-features
build_and_copy ./contracts/feature-tests/erc-style-contracts/erc20 $VM_REPO_PATH/test/erc20-rust
build_and_copy ./contracts/feature-tests/formatted-message-features $VM_REPO_PATH/test/features/formatted-message-features
build_and_copy ./contracts/feature-tests/payable-features $VM_REPO_PATH/test/features/payable-features
build_and_copy ./contracts/feature-tests/esdt-system-sc-mock $VM_REPO_PATH/test/features/esdt-system-sc-mock

build_and_copy_composability() {
   contract=$1
   contract_with_underscores="${contract//-/_}"

   # with managed-ei
   mxpy --verbose contract build --skip-eei-checks ./contracts/feature-tests/composability/$contract || return 1
   cp -R contracts/feature-tests/composability/$contract/output/${contract}.wasm \
      $VM_REPO_PATH/test/features/composability/$contract/output/${contract}.wasm

   # without managed-ei
   export RUSTFLAGS=${RUSTFLAGS-'-C link-arg=-s'}
   cd contracts/feature-tests/composability/$contract/wasm-no-managed-ei
   cargo build --target=wasm32-unknown-unknown --release
   cd ..
   mkdir -p output
   cp \
      wasm-no-managed-ei/target/wasm32-unknown-unknown/release/${contract_with_underscores}_wasm.wasm \
      output/${contract}-unmanaged.wasm
   cd ../../../..

   cp -R contracts/feature-tests/composability/$contract/output/${contract}-unmanaged.wasm \
      $VM_REPO_PATH/test/features/composability/$contract/output/${contract}-unmanaged.wasm
}

build_and_copy_composability forwarder
build_and_copy_composability forwarder-raw
build_and_copy_composability proxy-test-first
build_and_copy_composability proxy-test-second
build_and_copy_composability recursive-caller
build_and_copy_composability promises-features
mxpy --verbose contract build --skip-eei-checks ./contracts/feature-tests/composability/vault || return 1
cp -R contracts/feature-tests/composability/vault/output/vault.wasm \
   $VM_REPO_PATH/test/features/composability/vault/output/vault.wasm

rm -f $VM_REPO_PATH/test/features/composability/mandos/*
rm -f $VM_REPO_PATH/test/features/composability/scenarios/*
cp -R contracts/feature-tests/composability/scenarios \
   $VM_REPO_PATH/test/features/composability
cp -R contracts/feature-tests/composability/scenarios-promises \
   $VM_REPO_PATH/test/features/composability

mkdir -p $VM_REPO_PATH/test/features/composability/scenarios-legacy
rm -f $VM_REPO_PATH/test/features/composability/mandos-legacy/*
rm -f $VM_REPO_PATH/test/features/composability/scenarios-legacy/*
mmv -c 'contracts/feature-tests/composability/scenarios/*.scen.json' \
   $VM_REPO_PATH/test/features/composability/scenarios-legacy/l_'#1.scen.json'

sed -i 's/forwarder.wasm/forwarder-unmanaged.wasm/g' $VM_REPO_PATH/test/features/composability/scenarios-legacy/*
sed -i 's/forwarder-raw.wasm/forwarder-raw-unmanaged.wasm/g' $VM_REPO_PATH/test/features/composability/scenarios-legacy/*
sed -i 's/proxy-test-first.wasm/proxy-test-first-unmanaged.wasm/g' $VM_REPO_PATH/test/features/composability/scenarios-legacy/*
sed -i 's/proxy-test-second.wasm/proxy-test-second-unmanaged.wasm/g' $VM_REPO_PATH/test/features/composability/scenarios-legacy/*
sed -i 's/recursive-caller.wasm/recursive-caller-unmanaged.wasm/g' $VM_REPO_PATH/test/features/composability/scenarios-legacy/*
sed -i 's/proxy_test_init.scen.json/l_proxy_test_init.scen.json/g' $VM_REPO_PATH/test/features/composability/scenarios-legacy/*
