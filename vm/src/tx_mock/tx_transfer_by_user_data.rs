use crate::{
    tx_mock::TxInput,
    types::{VMAddress, H256},
};

use super::{CallType, TxFunctionName, TxTokenTransfer};

#[derive(Debug, Clone)]
pub struct TranferByUserData {
    pub from: VMAddress,
    pub to: VMAddress,
    pub token_transfers: Vec<TxTokenTransfer>,
    pub func_name: TxFunctionName,
    pub arguments: Vec<Vec<u8>>,
    pub tx_hash: H256,
}

pub fn transfer_by_user_tx_input(
    transfer_by_user_call: &TranferByUserData,
    call_type: CallType,
) -> TxInput {
    let mut egld_value = num_bigint::BigUint::default();
    let mut esdt_values = Vec::new();

    for transfer in &transfer_by_user_call.token_transfers {
        if transfer.token_identifier.is_empty() && transfer.nonce == 0 {
            egld_value += &transfer.value;
        } else {
            esdt_values.push(transfer.clone());
        }
    }
    TxInput {
        from: transfer_by_user_call.from.clone(),
        to: transfer_by_user_call.to.clone(),
        egld_value,
        esdt_values,
        func_name: transfer_by_user_call.func_name.clone(),
        args: transfer_by_user_call.arguments.clone(),
        call_type,
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: transfer_by_user_call.tx_hash.clone(),
        ..Default::default()
    }
}
