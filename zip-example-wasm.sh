#!/bin/sh

## Creates a zip files with all the .wasm and .abi.json outputs from examples.
## Used in generating an output artefact for each elrond-wasm release.

ZIP_OUTPUT="examples-wasm.zip"

# start fresh
rm -f $ZIP_OUTPUT

set -e
SMART_CONTRACT_JSONS=$(find contracts/examples -name "multiversx.json")
for smart_contract_json in $SMART_CONTRACT_JSONS
do
    smart_contract_folder=$(dirname $smart_contract_json)
    echo ""
    # build example wasm + ABI
    rm -rf $smart_contract_folder/output
    (set -x; mxpy --verbose contract build $smart_contract_folder)

    # add to zip
    zip -ur --junk-paths $ZIP_OUTPUT $smart_contract_folder/output
done
