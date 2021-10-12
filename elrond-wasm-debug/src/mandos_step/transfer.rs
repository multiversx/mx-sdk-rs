use mandos::model::TxTransfer;

use crate::{sc_call::tx_esdt_transfers_from_mandos, BlockchainMock};

pub fn execute(state: &mut BlockchainMock, tx: &TxTransfer) {
    let sender_address = &tx.from.value.into();
    state.increase_nonce(sender_address);
    state
        .subtract_tx_payment(sender_address, &tx.egld_value.value)
        .unwrap();
    let recipient_address = &tx.to.value.into();
    state.increase_balance(recipient_address, &tx.egld_value.value);

    let tx_esdt = tx_esdt_transfers_from_mandos(tx.esdt_value.as_slice());
    state.subtract_multi_esdt_balance(sender_address, tx_esdt.as_slice());
    state.increase_multi_esdt_balance(recipient_address, tx_esdt.as_slice());
}
