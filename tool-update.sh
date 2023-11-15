#!/bin/bash

# Locally update the VM version

VM_TAG="v1.5.19"

echo "Before:"
mxpy config dump
mxpy config set dependencies.vmtools.tag $VM_TAG
echo "After:"
mxpy config dump

mxpy deps install vmtools --overwrite
