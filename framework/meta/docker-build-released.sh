#!/bin/bash

# Builds the Docker image using the published sc-meta from crates.io.
# For a local source build, use docker-build.sh.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

docker build \
  --platform linux/amd64 \
  -f "$SCRIPT_DIR/Dockerfile" \
  -t multiversx/sc-meta-reproducible-build:0.65.1 \
  "$SCRIPT_DIR"
