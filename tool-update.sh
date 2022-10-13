#!/bin/bash

# Locally update the VM version

VM_TAG="v1.5.1"

echo "Before:"
erdpy config dump
erdpy config set dependencies.vmtools.tag $VM_TAG
echo "After:"
erdpy config dump

erdpy deps install vmtools --overwrite

# Also update the Rust version

erdpy deps install rust --tag="nightly" --overwrite
