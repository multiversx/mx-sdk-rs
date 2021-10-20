use std::rc::Rc;

use elrond_wasm::types::H256;
use mandos::model::TxTransfer;

use crate::{tx_execution::sc_call, tx_mock::TxInput, world_mock::BlockchainMock};

pub fn execute(state: &mut Rc<BlockchainMock>, tx_transfer: &TxTransfer) {
    match tx_transfer.esdt_value.len() {
        0 => {
            let tx_input = TxInput {
                from: tx_transfer.from.value.into(),
                to: tx_transfer.to.value.into(),
                egld_value: tx_transfer.egld_value.value.clone(),
                esdt_values: Vec::new(),
                func_name: Vec::new(),
                args: Vec::new(),
                gas_limit: 0,
                gas_price: 0,
                tx_hash: H256::zero(),
            };
            sc_call(tx_input, state, true).unwrap();
        },
        1 => panic!("single ESDT transfer not yet implemented"),
        _ => panic!("multi ESDT transfer not yet implemented"),
    }
}
