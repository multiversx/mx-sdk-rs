FIRST_USER_PEM=""
SECOND_USER_PEM=""

FORWARDER_ADDRESS_BECH32=
FORWARDER_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${FORWARDER_ADDRESS_BECH32})"

VAULT_ADDRESS_BECH32=
VAULT_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${VAULT_ADDRESS_BECH32})"

ECHO_ARGS_FUNC_NAME=echo_args_async

PROXY=https://testnet-api.multiversx.com
CHAIN_ID=T

deployForwarder() {
    mxpy --verbose contract deploy \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --recall-nonce \
        --pem=${FIRST_USER_PEM} \
        --bytecode="output/forwarder.wasm" \
        --gas-limit=200000000 \
        --send \
        --outfile="deploy-testnet.interaction.json" \
         || return
}

deployVault() {
    mxpy --verbose contract deploy \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --recall-nonce \
        --pem=${SECOND_USER_PEM} \
        --bytecode="../vault/output/vault.wasm" \
        --gas-limit=200000000 \
        --send \
        --outfile="deploy-testnet.interaction.json" \
         || return
}

echoNoArgs() {
    mxpy --verbose contract call ${FORWARDER_ADDRESS_BECH32} \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send \
    --recall-nonce --pem=${FIRST_USER_PEM} \
    --gas-limit=50000000 \
    --function=${ECHO_ARGS_FUNC_NAME} \
    --arguments ${VAULT_ADDRESS_HEX}
}

echoOneArgument() {
    mxpy --verbose contract call ${FORWARDER_ADDRESS_BECH32} \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send \
    --recall-nonce --pem=${FIRST_USER_PEM} \
    --gas-limit=25000000 \
    --function=${ECHO_ARGS_FUNC_NAME} \
    --arguments ${VAULT_ADDRESS_HEX} str:hello
}
