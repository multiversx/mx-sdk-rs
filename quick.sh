#!/bin/sh

export PATH=/home/andreim/erdpy-redirect:$PATH


# rm -f contracts/examples/adder/output/adder.wasm
# rm -f contracts/examples/adder/output/adder-dbg.c
# rm -f contracts/examples/adder/output/adder-dbg.wat

# erdpy contract build "contracts/examples/adder" 
# erdpy contract build "contracts/examples/adder" --wasm-symbols --wasm-name "adder-dbg.wasm"
# /home/andreim/elrond/operations/wasm/wabt/bin/wasm2wat \
#     contracts/examples/adder/output/adder-dbg.wasm \
#     -o \
#     contracts/examples/adder/output/adder-dbg.wat

# /home/andreim/elrond/operations/wasm/wabt/bin/wasm2c \
#     contracts/examples/adder/output/adder-dbg.wasm \
#     -o \
#     contracts/examples/adder/output/adder-dbg.c

# erdpy contract build "contracts/feature-tests/basic-features"
# erdpy contract build "contracts/feature-tests/basic-features" --wasm-symbols --wasm-name "basic-features-dbg.wasm"
# /home/andreim/elrond/operations/wasm/wabt/bin/wasm2wat \
#     contracts/feature-tests/basic-features/output/basic-features-dbg.wasm \
#     -o \
#     contracts/feature-tests/basic-features/output/basic-features-dbg.wat

erdpy contract build "contracts/examples/erc20" 
erdpy contract build "contracts/examples/erc20" --wasm-symbols --wasm-name "erc20-dbg.wasm"
/home/andreim/elrond/operations/wasm/wabt/bin/wasm2wat \
    contracts/examples/erc20/output/erc20-dbg.wasm \
    -o \
    contracts/examples/erc20/output/erc20-dbg.wat

# erdpy contract build "contracts/feature-tests/composability/forwarder"
# erdpy contract build "contracts/feature-tests/composability/forwarder-raw"
# erdpy contract build "contracts/feature-tests/composability/recursive-caller"
# erdpy contract build "contracts/feature-tests/composability/vault"
# erdpy contract build "contracts/feature-tests/composability/proxy-test-first"
# erdpy contract build "contracts/feature-tests/composability/proxy-test-second"

# erdpy contract build "contracts/feature-tests/composability/forwarder" --wasm-symbols --wasm-name "forwarder-dbg.wasm"
# erdpy contract build "contracts/feature-tests/composability/recursive-caller" --wasm-symbols --wasm-name "composability/recursive-caller-dbg.wasm"

# erdpy contract build "contracts/examples/multisig"
# erdpy contract build "contracts/examples/multisig" --wasm-symbols --wasm-name "multisig-dbg.wasm"

# erdpy contract build "contracts/feature-tests/panic-message-features" --wasm-symbols --wasm-name "panic-message-features-dbg.wasm"

# mandos-test "/home/andreim/elrond/smartcontract/elrond-wasm-rs/contracts/feature-tests/composability/mandos"