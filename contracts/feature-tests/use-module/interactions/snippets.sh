USER_PEM=""
USER_ADDRESS_BECH32=
USER_ADDDRESS_HEX=0x

RECEIVER_ADDRESS_BECH32=
RECEIVER_ADDRESS_HEX=0x

TOKEN_ID=""
TOKEN_ID_HEX=0x
TOKEN_AMOUNT_PER_TX=100

PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"

SC_WITH_ROLE_ADDRESS_BECH32=
SC_WITH_ROLE_ADDRESS_HEX=0x

SC_DEST_ADDRESS_BECH32=
SC_DEST_ADDRESS_HEX=0x

ESDT_SYSTEM_SC_ADDRESS=erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u
ESDT_TRANSFER_FUNC_NAME="ESDTTransfer"
ESDT_MULTI_TRANSFER_FUNC_NAME="MultiESDTNFTTransfer"

FORWARD_FUNC_NAME_ASCII="forwardPayments"
FORWARD_FUNC_NAME_HEX=0x666F72776172645061796D656E7473

deploySc() {
    erdpy --verbose contract deploy --project=${PROJECT} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --send --outfile="deploy-testnet.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

setSpecialRoleForSc() {
    erdpy --verbose contract call ${ESDT_SYSTEM_SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --function="setSpecialRole" \
    --arguments ${TOKEN_ID_HEX} 0x63cd6b5738bc0432c09ef200e2db8e5a59cc531c09cdb3dd5f6b631893b3437e 0x455344545472616E73666572526F6C65
}

transferSingleToUser() {
    erdpy --verbose contract call ${SC_WITH_ROLE_ADDRESS_BECH32} \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --function=${ESDT_TRANSFER_FUNC_NAME} \
    --arguments ${TOKEN_ID_HEX} ${TOKEN_AMOUNT_PER_TX} \
    ${FORWARD_FUNC_NAME_HEX} ${RECEIVER_ADDRESS_HEX} str:enjoy
}
