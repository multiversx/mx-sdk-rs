BOB="/home/elrond/MySandbox/testnet/wallets/users/bob.pem"
ADDRESS=$(erdpy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-testnet)
PROXY=https://testnet-api.elrond.com

DEV_TOKEN_IDENTIFIER=0x4445562d346237666238 # DEV-4b7fb8
ACCEPT_ESDT_FUNCTION=0x61636365707445736474 # acceptEsdt
ALICE_ADDRESS=0x0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1 # erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th

deploy() {
    erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${BOB} --gas-limit=50000000 --send --outfile="deploy-testnet.interaction.json" --proxy=${PROXY} --chain=T || return

    TRANSACTION=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-testnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

acceptEsdt() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} --gas-limit=5000000 --function="ESDTTransfer" --arguments ${DEV_TOKEN_IDENTIFIER} 0x0A ${ACCEPT_ESDT_FUNCTION} --send --proxy=${PROXY} --chain=T
}

transferSingle() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} --gas-limit=5000000 --function="transferEsdtOnce" --arguments ${ALICE_ADDRESS} ${DEV_TOKEN_IDENTIFIER} 0x01 --send --proxy=${PROXY} --chain=T
}

transferMultiple() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} --gas-limit=5000000 --function="transferEsdtMultiple" --arguments ${ALICE_ADDRESS} ${DEV_TOKEN_IDENTIFIER} 0x01 0x03 --send --proxy=${PROXY} --chain=T
}
