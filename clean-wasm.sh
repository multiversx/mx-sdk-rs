#!/bin/sh

# cleans all wasm targets

cargo install multiversx-sc-meta

sc-meta all clean --path ./contracts
