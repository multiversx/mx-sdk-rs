#!/bin/bash

# Builds the Docker image using the local workspace source tree.
# For the released (crates.io) image, use docker-build-released.sh.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "$SCRIPT_DIR"

WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

docker build \
  --platform linux/amd64 \
  -f "$SCRIPT_DIR/Dockerfile.local" \
  -t multiversx/sc-meta-reproducible-build:local \
  "$WORKSPACE_ROOT"