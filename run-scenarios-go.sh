#!/bin/bash

### Use this to build all contracts and test them using the VM.

./build-wasm.sh

cargo test --features run-go-tests
