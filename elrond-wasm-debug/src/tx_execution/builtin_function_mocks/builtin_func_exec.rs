use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

use super::{esdt_transfer_mock::execute_esdt_transfer, set_username_mock::execute_set_username};

pub const ESDT_TRANSFER_FUNC: &[u8] = b"ESDTTransfer";
pub const SET_USERNAME_FUNC: &[u8] = b"SetUserName";

pub fn execute_builtin_function_or_default(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    match tx_input.func_name.as_slice() {
        ESDT_TRANSFER_FUNC => execute_esdt_transfer(tx_input, tx_cache),
        SET_USERNAME_FUNC => execute_set_username(tx_input, tx_cache),
        _ => default_execution(tx_input, tx_cache),
    }
}
