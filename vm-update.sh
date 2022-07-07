#!/bin/bash

# Locally update the VM version

VM_TAG="v1.4.53"

echo "Before:"
erdpy config dump
erdpy config set dependencies.vmtools.tag $VM_TAG
echo "After:"
erdpy config dump

erdpy deps install vmtools --overwrite
