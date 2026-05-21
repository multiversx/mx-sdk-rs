#!/bin/bash

SCRIPT_DIR="$(realpath "$(dirname "${BASH_SOURCE[0]}")")"
META_DIR="$(realpath "${SCRIPT_DIR}/..")"
ROOT="$(realpath "${SCRIPT_DIR}/../../..")"

NETWORK=devnet      # choose: devnet, testnet, mainnet, chain-simulator
TOOL_VARIANT=source # choose: sc-meta, source

# ── Tool selection ─────────────────────────────────────────────────────────────
case "${TOOL_VARIANT}" in
    sc-meta) BASE=(sc-meta)                                                ;;
    source)  BASE=(cargo run --manifest-path "${META_DIR}/Cargo.toml" --)  ;;
esac

TX_TOOL=("${BASE[@]}" tx)
DATA_TOOL=("${BASE[@]}" data)
RB_TOOL=("${BASE[@]}" reproducible-build)
# ──────────────────────────────────────────────────────────────────────────────

case "${NETWORK}" in
    devnet)           PROXY=https://devnet-gateway.multiversx.com;  CHAIN=D     ;;
    testnet)          PROXY=https://testnet-gateway.multiversx.com; CHAIN=T     ;;
    mainnet)          PROXY=https://gateway.multiversx.com;         CHAIN=1     ;;
    chain-simulator)  PROXY=http://localhost:8085;                   CHAIN=chain ;;
esac

VERIFIER_URL=https://devnet-play-api.multiversx.com
PEM="${ROOT}/sdk/core/src/test_wallets/mike.pem"

RB_CONTRACT_DIR="${ROOT}/contracts/test-reproducible-build/scripts"
BASIC_CONTRACT_NAME=basic-rb
RB_ADDER_DIR="${RB_CONTRACT_DIR}/${BASIC_CONTRACT_NAME}"
RB_ADDER_WASM="${RB_ADDER_DIR}/output/${BASIC_CONTRACT_NAME}.wasm"
RB_ADDER_PACKAGED_SRC="${RB_ADDER_DIR}/output-rb/${BASIC_CONTRACT_NAME}/${BASIC_CONTRACT_NAME}-0.0.0.source.json"
RB_ADDER_ADDRESS="erd1qqqqqqqqqqqqqpgq45afyyhufmsmem5p9tfay9huyw9xvhdla4sqlzwewf"
RB_ADDER_CODE_HASH=3d2cebbce167957486e66abbe9191afd64bdb8a46b2b39143699199313082e58
OUTFILE_DEPLOY="deploy-stripped-adder-${NETWORK}.interaction.json"

setup() {
    "${BASE[@]}" new --template adder --name "${BASIC_CONTRACT_NAME}" \
        --path "${RB_CONTRACT_DIR}"
    "${RB_TOOL[@]}" init-config --path "${RB_ADDER_DIR}"
}

deploy() {
    "${TX_TOOL[@]}" deploy \
        --bytecode "${RB_ADDER_WASM}" \
        --pem="${PEM}" \
        --gas-limit=50000000 \
        --arguments 0 \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_DEPLOY}" \
        || return

    RB_ADDER_ADDRESS=$("${DATA_TOOL[@]}" parse --file="${OUTFILE_DEPLOY}" --expression="data['contractAddress']" 2>/dev/null)
    "${DATA_TOOL[@]}" store --partition "${NETWORK}" --key="stripped-adder-address" --value="${RB_ADDER_ADDRESS}" 2>/dev/null || true

    echo ""
    echo "Deployed adder address: ${RB_ADDER_ADDRESS}"
}


build() {
    "${BASE[@]}" all build --path "${RB_ADDER_DIR}"
    "${RB_TOOL[@]}" build --project "${RB_ADDER_DIR}"
}

download() {
    "${RB_TOOL[@]}" download "${RB_ADDER_ADDRESS}" \
        --verifier-url "${VERIFIER_URL}" \
        --output "${ROOT}/contracts/test-reproducible-build/downloaded" \
        --overwrite
}

publish() {
    "${RB_TOOL[@]}" publish "${RB_ADDER_ADDRESS}" \
        --packaged-src "${RB_ADDER_PACKAGED_SRC}" \
        --docker-image multiversx/sdk-rust-contract-builder:v12.0.0-alpha \
        --verifier-url "${VERIFIER_URL}" \
        --pem "${PEM}" \
        -y
}

unpublish() {
    "${RB_TOOL[@]}" unpublish "${RB_ADDER_ADDRESS}" \
        --code-hash "${RB_ADDER_CODE_HASH}" \
        --verifier-url "${VERIFIER_URL}" \
        --pem "${PEM}" \
        -y
}
