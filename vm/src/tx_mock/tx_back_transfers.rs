use num_bigint::BigUint;

use crate::{tx_execution::BuiltinFunctionContainer, types::VMAddress};

use super::{async_call_tx_input, CallType, TxResult, TxTokenTransfer};

#[derive(Default)]
pub struct BackTransfers {
    pub call_value: BigUint,
    pub esdt_transfers: Vec<TxTokenTransfer>,
}

impl BackTransfers {
    pub fn empty() -> Self {
        BackTransfers::default()
    }

    pub fn new_from_result(
        &mut self,
        own_address: &VMAddress,
        result: &TxResult,
        builtin_functions: &BuiltinFunctionContainer,
    ) {
        let mut bt = BackTransfers::default();

        for call in &result.all_calls {
            // TODO: refactor, check type

            if call.endpoint_name.is_empty() {
                bt.call_value += &call.call_value;
                continue;
            }

            let tx_input = async_call_tx_input(call, CallType::BackTransfer);
            let mut token_transfers = builtin_functions.extract_token_transfers(&tx_input);
            if &token_transfers.real_recipient == own_address {
                bt.esdt_transfers.append(&mut token_transfers.transfers);
            }
        }

        self.call_value = bt.call_value;
        self.esdt_transfers = bt.esdt_transfers;
    }
    
}
