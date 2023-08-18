#!/bin/sh

## Creates a zip files with all the .wasm and .abi.json outputs from examples.
## Used in generating an output artefact for each elrond-wasm release.

ZIP_OUTPUT="examples-wasm.zip"
TARGET_DIR=$PWD/target
cargo install multiversx-sc-meta

# start fresh
rm -f $ZIP_OUTPUT

sc-meta all build --target-dir-wasm $TARGET_DIR --path ./contracts/examples || return 1

SMART_CONTRACT_JSONS=$(find contracts/examples -name "multiversx.json")
for smart_contract_json in $SMART_CONTRACT_JSONS
do
    smart_contract_folder=$(dirname $smart_contract_json)

    # add to zip
    zip -ur --junk-paths $ZIP_OUTPUT $smart_contract_folder/output
done
