use std::rc::Rc;

use elrond_wasm::types::H256;
use mandos::model::TxTransfer;

use crate::{
    sc_call::tx_esdt_transfers_from_mandos, tx_execution::sc_call, tx_mock::TxInput,
    world_mock::BlockchainMock, ContractMap,
};

pub fn execute(state: &mut Rc<BlockchainMock>, tx_transfer: &TxTransfer) {
    match tx_transfer.esdt_value.len() {
        0 => {
            let tx_input = TxInput {
                from: tx_transfer.from.value.clone().into(),
                to: tx_transfer.to.value.clone().into(),
                egld_value: tx_transfer.egld_value.value.clone(),
                esdt_values: Vec::new(),
                func_name: Vec::new(),
                args: Vec::new(),
                gas_limit: 0,
                gas_price: 0,
                tx_hash: H256::zero(),
            };
            sc_call(tx_input, state, &ContractMap::default()).unwrap();
        },
        1 => panic!("single ESDT transfer not yet implemented"),
        _ => panic!("multi ESDT transfer not yet implemented"),
    }

    // let sender_address = &tx.from.value.into();
    // state.increase_nonce(sender_address);
    // state
    //     .subtract_egld_balance(sender_address, &tx.egld_value.value)
    //     .unwrap();
    // let recipient_address = &tx.to.value.into();
    // state.increase_egld_balance(recipient_address, &tx.egld_value.value);

    // let tx_esdt = tx_esdt_transfers_from_mandos(tx.esdt_value.as_slice());
    // state.subtract_multi_esdt_balance(sender_address, tx_esdt.as_slice());
    // state.increase_multi_esdt_balance(recipient_address, tx_esdt.as_slice());
}
