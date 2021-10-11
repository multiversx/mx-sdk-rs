use mandos::model::TxTransfer;
use num_traits::Zero;

use crate::BlockchainMock;

pub fn execute(state: &mut BlockchainMock, tx: &TxTransfer) {
    let sender_address = &tx.from.value.into();
    state.increase_nonce(sender_address);
    state
        .subtract_tx_payment(sender_address, &tx.value.value)
        .unwrap();
    let recipient_address = &tx.to.value.into();
    state.increase_balance(recipient_address, &tx.value.value);

    tx.esdt_value.iter().for_each(|esdt_transfer| {
        let esdt_value = esdt_transfer.esdt_value.value.clone();
        if !esdt_value.is_zero() {
            let esdt_token_identifier = esdt_transfer.esdt_token_identifier.value.clone();
            let nonce = esdt_transfer.nonce.value;
            state.substract_esdt_balance(
                sender_address,
                &esdt_token_identifier[..],
                nonce,
                &esdt_value,
            );
            state.increase_esdt_balance(
                recipient_address,
                &esdt_token_identifier[..],
                nonce,
                &esdt_value,
            );
        }
    })
}
