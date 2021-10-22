use std::rc::Rc;

use elrond_wasm::types::H256;
use mandos::model::TxTransfer;

use crate::{
    sc_call::tx_esdt_transfers_from_mandos, tx_execution::sc_call, tx_mock::TxInput,
    world_mock::BlockchainMock,
};

pub fn execute(state: &mut Rc<BlockchainMock>, tx_transfer: &TxTransfer) {
    let tx_input = TxInput {
        from: tx_transfer.from.value.into(),
        to: tx_transfer.to.value.into(),
        egld_value: tx_transfer.egld_value.value.clone(),
        esdt_values: tx_esdt_transfers_from_mandos(tx_transfer.esdt_value.as_slice()),
        func_name: Vec::new(),
        args: Vec::new(),
        gas_limit: tx_transfer.gas_limit.value,
        gas_price: tx_transfer.gas_price.value,
        tx_hash: H256::zero(),
    };
    sc_call(tx_input, state, true);
}
