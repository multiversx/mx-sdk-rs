#!/bin/bash

# Locally update the VM version

VM_TAG="1.4.51"

Echo "Before:"
erdpy config dump
erdpy config set dependencies.vmtools.tag $VM_TAG
Echo "After:"
erdpy config dump

erdpy deps install vmtools --overwrite
