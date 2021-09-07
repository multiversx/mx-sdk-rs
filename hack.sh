#!/bin/sh


# cd contracts/feature-tests/basic-features/wasm
# cargo build --target=wasm32-unknown-unknown --release
# cd ..
# mkdir -p output
# cp wasm/target/wasm32-unknown-unknown/release/basic_features_wasm.wasm output/features-dbg.wasm
# cd ../../..

# twiggy top -n 1000 contracts/feature-tests/basic-features/output/features-dbg.wasm > tiggy.txt
# twiggy paths contracts/feature-tests/basic-features/output/features-dbg.wasm > tpaths.txt
# twiggy monos contracts/feature-tests/basic-features/output/features-dbg.wasm > tmonos.txt



# cd contracts/examples/factorial/wasm
# cargo build --target=wasm32-unknown-unknown --release
# cd ..
# mkdir -p output
# cp wasm/target/wasm32-unknown-unknown/release/factorial_wasm.wasm output/factorial-dbg.wasm
# cd ../../..



cd contracts/examples/erc20/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/erc20_wasm.wasm output/erc20-dbg.wasm
twiggy top -n 1000 output/erc20-dbg.wasm > erc-twiggy.txt
twiggy paths output/erc20-dbg.wasm > erc-paths.txt
twiggy monos output/erc20-dbg.wasm > erc-monos.txt
twiggy top -n 1000 output/erc20.wasm > erc-size.txt
cd ../../..
