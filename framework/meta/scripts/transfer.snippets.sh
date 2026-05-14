#!/bin/bash

NETWORK=devnet      # choose: devnet, testnet, mainnet, chain-simulator
TOOL_VARIANT=source # choose: sc-meta, source, mxpy

# ── Tool selection ─────────────────────────────────────────────────────────────
case "${TOOL_VARIANT}" in
    sc-meta) BASE=(sc-meta)                                      ;;
    source)  BASE=(cargo run --manifest-path ../Cargo.toml --)   ;;
    mxpy)    BASE=(mxpy --verbose)                               ;;
esac

TX_TOOL=("${BASE[@]}" tx)
# ──────────────────────────────────────────────────────────────────────────────

ALICE="../../../sdk/core/src/test_wallets/alice.pem"

case "${NETWORK}" in
    devnet)           PROXY=https://devnet-gateway.multiversx.com;  CHAIN=D      ;;
    testnet)          PROXY=https://testnet-gateway.multiversx.com; CHAIN=T      ;;
    mainnet)          PROXY=https://gateway.multiversx.com;         CHAIN=1      ;;
    chain-simulator)  PROXY=http://localhost:8085;                  CHAIN=chain  ;;
esac

OUTFILE_TRANSFER="transfer-${NETWORK}.interaction.json"

transfer1() {
    RECEIVER=erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx
    EGLD_VALUE=10000000000000000 # 0.01 EGLD

    "${TX_TOOL[@]}" new \
        --pem="${ALICE}" \
        --receiver="${RECEIVER}" \
        --value="${EGLD_VALUE}" \
        --gas-limit=50000 \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --outfile="${OUTFILE_TRANSFER}" \
        --send \
        --wait-result \
        || return
}

transfer2() {
    RECEIVER=erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx
    USDC_AMOUNT=10000 # 0.01 USDC (6 decimals)
    EGLD_VALUE=10000000000000000 # 0.01 EGLD

    "${TX_TOOL[@]}" new \
        --pem="${ALICE}" \
        --receiver="${RECEIVER}" \
        --token-transfers \
            "USDC-350c4e" "${USDC_AMOUNT}" \
            "EGLD-000000" "${EGLD_VALUE}" \
        --gas-limit=800000 \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --outfile="${OUTFILE_TRANSFER}" \
        --send \
        --wait-result \
        || return
}

# transfer3 uses --payments (TOKEN-ID NONCE AMOUNT triples), which is sc-meta only and NOT supported by mxpy.
transfer3() {
    RECEIVER=erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx
    USDC_AMOUNT=10000 # 0.01 USDC (6 decimals), nonce 0 (fungible)
    EGLD_VALUE=10000000000000000 # 0.01 EGLD

    "${TX_TOOL[@]}" new \
        --pem="${ALICE}" \
        --receiver="${RECEIVER}" \
        --payments \
            "USDC-350c4e" 0 "${USDC_AMOUNT}" \
            "EGLD-000000" "${EGLD_VALUE}" \
        --gas-limit=800000 \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --outfile="${OUTFILE_TRANSFER}" \
        --send \
        --wait-result \
        || return
}
