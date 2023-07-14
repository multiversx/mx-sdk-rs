#!/bin/bash

cargo install multiversx-sc-meta

TARGET_DIR=$PWD/target

sc-meta all update --path ./contracts
