use mandos::TxTransfer;

use crate::BlockchainMock;

pub fn execute(state: &mut BlockchainMock, tx: &TxTransfer) {
    let sender_address = &tx.from.value.into();
    state.increase_nonce(sender_address);
    state
        .subtract_tx_payment(sender_address, &tx.value.value)
        .unwrap();
    let recipient_address = &tx.to.value.into();
    state.increase_balance(recipient_address, &tx.value.value);
    let esdt_token_identifier = tx.esdt_token_identifier.value.clone();
    let nonce = tx.nonce.value.clone();
    let esdt_value = tx.esdt_value.value.clone();

    if !esdt_token_identifier.is_empty() && esdt_value > 0u32.into() {
        state.substract_esdt_balance(
            sender_address,
            &esdt_token_identifier[..],
            nonce.clone(),
            &esdt_value,
        );
        state.increase_esdt_balance(
            recipient_address,
            &esdt_token_identifier[..],
            nonce,
            &esdt_value,
        );
    }
}
