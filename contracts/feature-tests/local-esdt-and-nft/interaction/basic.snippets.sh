ALICE="/home/elrond/elrond-sdk/erdpy/testnet/wallets/users/alice.pem"
ADDRESS=$(erdpy data load --key=address-testnet)
ADDRESS_DECODED=$(erdpy wallet bech32 --decode ${ADDRESS})
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-testnet)
PROXY=http://localhost:7950
CHAIN_ID=local-testnet

TOKEN_DISPLAY_NAME=0x46756e6769626c65546f6b656e # "FungibleToken"
TOKEN_TICKER=0x46554e47544f4b # "FUNGTOK"

# Manually update after issue
TOKEN_IDENTIFIER=0x46554e47544f4b2d623639646635

deploy() {
    erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} --gas-limit=100000000 --send --outfile="deploy-testnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-testnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --project==${PROJECT} --recall-nonce --pem=${ALICE} --gas-limit=75000000 --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

# SC calls

issueFungibleToken() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=80000000 --value=5000000000000000000 --function="issueFungibleToken" --arguments ${TOKEN_DISPLAY_NAME} ${TOKEN_TICKER} 0x03E8 --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

localMint() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=60000000 --function="localMint" --arguments ${TOKEN_IDENTIFIER} 0x64 --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

localBurn() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=60000000 --function="localBurn" --arguments ${TOKEN_IDENTIFIER} 0x64 --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# 0x01 = localmint roles, 0x02 = localburn role
setLocalRolesFungible() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=500000000 --function="setLocalRoles" --arguments 0x${ADDRESS_DECODED} ${TOKEN_IDENTIFIER} 0x01 0x02 --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

removeLocalRolesFungible() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=500000000 --function="unsetLocalRoles" --arguments 0x${ADDRESS_DECODED} ${TOKEN_IDENTIFIER} 0x01 0x02 --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# Queries

getFungibleEsdtBalance() {
    erdpy --verbose contract query ${ADDRESS} --function="getFungibleEsdtBalance" --arguments ${TOKEN_IDENTIFIER} --proxy=${PROXY}
}

getLastIssuedToken() {
    erdpy --verbose contract query ${ADDRESS} --function="lastIssuedToken" --proxy=${PROXY}
}

getLastError() {
    erdpy --verbose contract query ${ADDRESS} --function="lastErrorMessage" --proxy=${PROXY}
}
