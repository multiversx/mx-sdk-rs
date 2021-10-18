PEM_PATH="/home/elrond/Downloads/devnetWalletKey.pem"
FORWARDER_CODE_PATH="output/forwarder-raw.wasm"
VAULT_CODE_PATH="../vault/output/vault.wasm"

PROXY=https://testnet-gateway.elrond.com
CHAIN_ID=T

OWN_ADDRESS_BECH32=erd1yyfyrzu7wu5lh8jqegtj5klc0y6z8n8tjzlsw2zd00tu9pwl082sfv6x8c

# Update after deploy
FORWARDER_ADDRESS_BECH32=erd1qqqqqqqqqqqqqpgq39ug7svr8vyarzk9jtmq4wx5pd0jpwz0082slwmzdf
VAULT_ADDRESS_BECH32=erd1qqqqqqqqqqqqqpgq0tmcfcqcjsglnlafq6a0my3fsuewqyjx082svt8qag

ESDT_SYSTEM_SC_ADDRESS=erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u
ESDT_ISSUE_COST=50000000000000000

# Update after issue
TOKEN_ID=0x5346542d356366346237

deployForwarder() {
    erdpy --verbose contract deploy --bytecode=${FORWARDER_CODE_PATH} --recall-nonce --pem=${PEM_PATH} \
        --gas-limit=100000000 \
        --send --outfile="deploy.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID}

    TRANSACTION=$(erdpy data parse --file="deploy.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy.interaction.json" --expression="data['emitted_tx']['address']")
    
    echo ""
    echo "Forwarder Smart contract address: ${ADDRESS}"
}

deployVault() {
    erdpy --verbose contract deploy --bytecode=${VAULT_CODE_PATH} --recall-nonce --pem=${PEM_PATH} \
        --gas-limit=100000000 \
        --send --outfile="deploy.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID}

    TRANSACTION=$(erdpy data parse --file="deploy.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy.interaction.json" --expression="data['emitted_tx']['address']")
    
    echo ""
    echo "Vault Smart contract address: ${ADDRESS}"
}

issueToken() {
    local TOKEN_DISPLAY_NAME=0x4d79536674  # "MySft"
    local TOKEN_TICKER=0x534654  # "SFT"
    local CAN_TRANSFER_ROLE=0x63616e5472616e736665724e4654437265617465526f6c65 # "canTransferNFTCreateRole"
    local TRUE=0x74727565 # "true"

    erdpy --verbose contract call ${ESDT_SYSTEM_SC_ADDRESS} --recall-nonce --pem=${PEM_PATH} \
    --gas-limit=60000000 --value=${ESDT_ISSUE_COST} --function="issueSemiFungible" \
    --arguments ${TOKEN_DISPLAY_NAME} ${TOKEN_TICKER} ${CAN_TRANSFER_ROLE} ${TRUE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setLocalRolesSelf() {
    local OWN_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${OWN_ADDRESS_BECH32})
    local NFT_CREATE_ROLE=0x45534454526f6c654e4654437265617465 # "ESDTRoleNFTCreate"
    local NFT_BURN_ROLE=0x45534454526f6c654e46544275726e # "ESDTRoleNFTBurn"
    local NFT_ADD_QUANTITY_ROLE=0x45534454526f6c654e46544164645175616e74697479 # "ESDTRoleNFTAddQuantity"

    erdpy --verbose contract call ${ESDT_SYSTEM_SC_ADDRESS} --recall-nonce --pem=${PEM_PATH} \
    --gas-limit=60000000 --function="setSpecialRole" \
    --arguments ${TOKEN_ID} ${OWN_ADDRESS_HEX} ${NFT_CREATE_ROLE} ${NFT_BURN_ROLE} ${NFT_ADD_QUANTITY_ROLE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

transferNftCreateRoleToVault() {
    local OWN_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${OWN_ADDRESS_BECH32})
    local VAULT_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${VAULT_ADDRESS_BECH32})

    erdpy --verbose contract call ${ESDT_SYSTEM_SC_ADDRESS} --recall-nonce --pem=${PEM_PATH} \
    --gas-limit=60000000 --function="transferNFTCreateRole" \
    --arguments ${TOKEN_ID} ${OWN_ADDRESS_HEX} ${VAULT_ADDRESS_HEX} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setLocalRolesVault() {
    local VAULT_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${VAULT_ADDRESS_BECH32})
    local NFT_BURN_ROLE=0x45534454526f6c654e46544275726e # "ESDTRoleNFTBurn"
    local NFT_ADD_QUANTITY_ROLE=0x45534454526f6c654e46544164645175616e74697479 # "ESDTRoleNFTAddQuantity"

    erdpy --verbose contract call ${ESDT_SYSTEM_SC_ADDRESS} --recall-nonce --pem=${PEM_PATH} \
    --gas-limit=60000000 --function="setSpecialRole" \
    --arguments ${TOKEN_ID} ${VAULT_ADDRESS_HEX} ${NFT_BURN_ROLE} ${NFT_ADD_QUANTITY_ROLE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

createSft() {
    local INITIAL_QUANTITY=0x03e8 # 5000

    erdpy --verbose contract call ${OWN_ADDRESS_BECH32} --recall-nonce --pem=${PEM_PATH} \
    --gas-limit=10000000 --function="ESDTNFTCreate" \
    --arguments ${TOKEN_ID} ${INITIAL_QUANTITY} 0x 0x 0x 0x 0x \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

sendSftToVault() {
    local VAULT_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${VAULT_ADDRESS_BECH32})
    local DEPOSIT_FUNC_NAME_VAULT=0x6a7573745f6163636570745f66756e6473 # "just_accept_funds"

    local SFT_NONCE=0x01 # 1
    local AMOUNT=0x03e8 # 1000

    erdpy --verbose contract call ${OWN_ADDRESS_BECH32} --recall-nonce --pem=${PEM_PATH} \
    --gas-limit=100000000 --function="ESDTNFTTransfer" \
    --arguments ${TOKEN_ID} ${SFT_NONCE} ${AMOUNT} ${VAULT_ADDRESS_HEX} ${DEPOSIT_FUNC_NAME_VAULT} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

############################################################################
############################################################################
############################################################################

sendSftToForwarder() {
    local FORWARDER_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${FORWARDER_ADDRESS_BECH32})
    local DEPOSIT_FUNC_NAME_FORWARDER=0x6465706f736974 # "deposit"

    local SFT_NONCE=0x01 # 1
    local AMOUNT=0x03e8 # 1000

    erdpy --verbose contract call ${OWN_ADDRESS_BECH32} --recall-nonce --pem=${PEM_PATH} \
    --gas-limit=100000000 --function="ESDTNFTTransfer" \
    --arguments ${TOKEN_ID} ${SFT_NONCE} ${AMOUNT} ${FORWARDER_ADDRESS_HEX} ${DEPOSIT_FUNC_NAME_FORWARDER} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# Test scenario 4
transferFromVaultToForwarderOnCallback() {
    local VAULT_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${VAULT_ADDRESS_BECH32})
    local TOKEN_NONCE=0x01 # 1
    local AMOUNT=0x32 # 50

    erdpy --verbose contract call ${FORWARDER_ADDRESS_BECH32} --recall-nonce --pem=${PEM_PATH} \
    --gas-limit=100000000 --function="forward_async_retrieve_multi_transfer_funds" \
    --arguments ${VAULT_ADDRESS_HEX} ${TOKEN_ID} ${TOKEN_NONCE} ${AMOUNT} ${TOKEN_ID} ${TOKEN_NONCE} ${AMOUNT} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# Test scenario 5
transferFromForwarderToVaultAndBackToForwarder() {
    local VAULT_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${VAULT_ADDRESS_BECH32})
    local TOKEN_NONCE=0x01 # 1
    local AMOUNT=0x32 # 50

    erdpy --verbose contract call ${FORWARDER_ADDRESS_BECH32} --recall-nonce --pem=${PEM_PATH} \
    --gas-limit=100000000 --function="forwarder_async_send_and_retrieve_multi_transfer_funds" \
    --arguments ${VAULT_ADDRESS_HEX} ${TOKEN_ID} ${TOKEN_NONCE} ${AMOUNT} ${TOKEN_ID} ${TOKEN_NONCE} ${AMOUNT} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# Change the PEM_PATH, FORWARDER_CODE_PATH, VAULT_CODE_PATH and OWN_ADDRESS_BECH32 variables accordingly

# Init steps for both scenarios:

# deployForwarder -> Put its address in the FORWARDER_ADDRESS_BECH32 variable
# deployVault -> Put its address in the VAULT_ADDRESS_BECH32 variable
# issueToken -> Put its token ID in the TOKEN_ID variable
# setLocalRolesSelf
# createSft

# Scenario 4
# sendSftToVault
# transferFromVaultToForwarderOnCallback

# Scenario 5
# sendSftToForwarder
# transferNftCreateRoleToVault
# setLocalRolesVault
# transferFromForwarderToVaultAndBackToForwarder