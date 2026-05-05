#!/bin/bash

# Builds the Docker image using the local workspace source tree.
# For the released (crates.io) image, use docker-build-released.sh.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "$SCRIPT_DIR"

WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

docker build \
  --platform linux/amd64 \
  --progress=plain \
  --no-cache \
  --build-arg VERSION_SC_META=local \
  -f "$SCRIPT_DIR/Dockerfile" \
  -t multiversx/sdk-rust-contract-builder:v12.0.0-alpha \
  "$WORKSPACE_ROOT"


docker push multiversx/sdk-rust-contract-builder:v12.0.0-alpha
