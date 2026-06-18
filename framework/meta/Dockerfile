FROM ubuntu:24.04

# Toolchain versions — bump these together when upgrading.
ARG VERSION_RUST="1.95.0"
# Set VERSION_SC_META=local to install sc-meta from the local source tree instead of
# crates.io. When building locally, the build context must be the workspace root:
#   docker build --build-arg VERSION_SC_META=local -f framework/meta/Dockerfile .
ARG VERSION_SC_META="0.66.2"
ARG VERSION_WASM_OPT="0.116.1"
ARG TARGETPLATFORM

# Install system dependencies
RUN apt-get update --fix-missing && apt-get install -y --no-install-recommends \
    ca-certificates \
    wget \
    build-essential \
    git \
    pkg-config \
    libssl-dev 

# Install Rust
RUN wget -O rustup.sh https://sh.rustup.rs && \
    chmod +x rustup.sh && \
    CARGO_HOME=/rust RUSTUP_HOME=/rust ./rustup.sh \
        --verbose \
        --default-toolchain ${VERSION_RUST} \
        --profile minimal \
        -y && \
    rm rustup.sh && \
    chmod -R 777 /rust && \
    rm -rf /rust/registry

# Install sc-meta: from crates.io when VERSION_SC_META is a version string,
# or from the local source tree when VERSION_SC_META=local.
# The bind mount is only accessed in the local branch; no source is baked
# into the image in either case.
RUN --mount=type=bind,source=.,target=/mx-sdk-rs,readonly \
    if [ "${VERSION_SC_META}" = "local" ]; then \
        PATH="/rust/bin:${PATH}" CARGO_HOME=/rust RUSTUP_HOME=/rust CARGO_TARGET_DIR=/tmp/sc-meta-build-target \
        cargo install --path /mx-sdk-rs/framework/meta --locked; \
    else \
        PATH="/rust/bin:${PATH}" CARGO_HOME=/rust RUSTUP_HOME=/rust \
        cargo install multiversx-sc-meta --version ${VERSION_SC_META} --locked; \
    fi && \
    rm -rf /rust/registry /tmp/sc-meta-build-target

# Install wasm32 target
RUN PATH="/rust/bin:${PATH}" CARGO_HOME=/rust RUSTUP_HOME=/rust \
    sc-meta install wasm32

# Install wasm-opt
RUN PATH="/rust/bin:${PATH}" CARGO_HOME=/rust RUSTUP_HOME=/rust \
    cargo install wasm-opt --version ${VERSION_WASM_OPT} --locked && \
    rm -rf /rust/registry

ENV PATH="/rust/bin:${PATH}"
ENV CARGO_HOME="/rust"
ENV RUSTUP_HOME="/rust"

# Use system git for Cargo fetches to avoid libgit2 memory spikes on large indexes.
# See https://github.com/rust-lang/cargo/issues/10583
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# Exposed to sc-meta at runtime for embedding into .source.json build metadata.
ENV BUILD_METADATA_VERSION_RUST=${VERSION_RUST}
ENV BUILD_METADATA_VERSION_SC_META=${VERSION_SC_META}
ENV BUILD_METADATA_VERSION_WASM_OPT=${VERSION_WASM_OPT}
ENV BUILD_METADATA_TARGETPLATFORM=${TARGETPLATFORM}

# /project  — mounted read-only by the caller (the contract source tree)
# /output   — mounted read-write by the caller (build artifacts land here)
# /rust/cargo-target-dir — optionally mounted for caching between runs

# Additional arguments forwarded at "docker run" time:
#   --project /project        (required; set automatically by docker-build)
#   --contract <name>         (optional)
#   --no-wasm-opt             (optional)
#   --build-root <path>       (optional; defaults to /tmp/sc-build inside the container)
ENTRYPOINT ["sc-meta", "reproducible-build", "local-build", \
    "--output", "/output", \
    "--target-dir", "/rust/cargo-target-dir"]

LABEL rust=${VERSION_RUST}
LABEL sc_meta=${VERSION_SC_META}
LABEL wasm_opt=${VERSION_WASM_OPT}
