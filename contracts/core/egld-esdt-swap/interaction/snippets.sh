ALICE="/home/elrond/elrond-sdk/erdpy/testnet/wallets/users/alice.pem"
BOB="/home/elrond/elrond-sdk/erdpy/testnet/wallets/users/bob.pem"
ADDRESS=$(erdpy data load --key=address-testnet-egld-esdt-swap)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-testnet)
PROXY=https://testnet-gateway.elrond.com
CHAIN_ID=T

ESDT_SYSTEM_SC_ADDRESS=erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u

deploy() {
    ######################################################################
    ############################ Update after issue ######################
    ######################################################################
    local WRAPPED_EGLD_TOKEN_ID=0x

    erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
    --gas-limit=100000000 \
    --arguments ${WRAPPED_EGLD_TOKEN_ID} \
    --send --outfile="deploy-testnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-testnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-testnet-egld-esdt-swap --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${ALICE} \
    --arguments ${WRAPPED_EGLD_TOKEN_ID} --gas-limit=100000000 --outfile="upgrade.json" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

issueWrappedEgld() {
    local TOKEN_DISPLAY_NAME=0x5772617070656445676c64  # "WrappedEgld"
    local TOKEN_TICKER=0x5745474c44  # "WEGLD"
    local INITIAL_SUPPLY=0x01 # 1
    local NR_DECIMALS=0x12 # 18
    local CAN_ADD_SPECIAL_ROLES=0x63616e4164645370656369616c526f6c6573 # "canAddSpecialRoles"
    local TRUE=0x74727565 # "true"

    erdpy --verbose contract call ${ESDT_SYSTEM_SC_ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=60000000 --value=5000000000000000000 --function="issue" \
    --arguments ${TOKEN_DISPLAY_NAME} ${TOKEN_TICKER} ${INITIAL_SUPPLY} ${NR_DECIMALS} ${CAN_ADD_SPECIAL_ROLES} ${TRUE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setLocalRoles() {
    local LOCAL_MINT_ROLE=0x45534454526f6c654c6f63616c4d696e74 # "ESDTRoleLocalMint"
    local LOCAL_BURN_ROLE=0x45534454526f6c654c6f63616c4275726e # "ESDTRoleLocalBurn"
    local ADDRESS_HEX = $(erdpy wallet bech32 --decode ${ADDRESS})

    erdpy --verbose contract call ${ESDT_SYSTEM_SC_ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=60000000 --function="setSpecialRole" \
    --arguments ${WRAPPED_EGLD_TOKEN_ID} ${ADDRESS_HEX} ${LOCAL_MINT_ROLE} ${LOCAL_BURN_ROLE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

wrapEgldBob() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} \
    --gas-limit=10000000 --value=1000 --function="wrapEgld" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

unwrapEgldBob() {
    local UNWRAP_EGLD_ENDPOINT=0x756e7772617045676c64 # "unwrapEgld"
    local UNWRAP_AMOUNT=0x05

    getWrappedEgldTokenIdentifier
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} \
    --gas-limit=10000000 --function="ESDTTransfer" \
    --arguments ${TOKEN_IDENTIFIER} ${UNWRAP_AMOUNT} ${UNWRAP_EGLD_ENDPOINT} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# views

getWrappedEgldTokenIdentifier() {
    local QUERY_OUTPUT=$(erdpy --verbose contract query ${ADDRESS} --function="getWrappedEgldTokenId" --proxy=${PROXY})
    TOKEN_IDENTIFIER=0x$(jq -r '.[0] .hex' <<< "${QUERY_OUTPUT}")
    echo "Wrapped eGLD token identifier: ${TOKEN_IDENTIFIER}"
}

getLockedEgldBalance() {
    erdpy --verbose contract query ${ADDRESS} --function="getLockedEgldBalance" --proxy=${PROXY}
}
