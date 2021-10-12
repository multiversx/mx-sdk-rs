use crate::{
    tx_mock::{TxInput, TxResult},
    world_mock::BlockchainMock,
};

use super::{esdt_transfer_mock::execute_esdt_transfer, set_username_mock::execute_set_username};

pub const ESDT_TRANSFER_FUNC: &[u8] = b"ESDTTransfer";
pub const SET_USERNAME_FUNC: &[u8] = b"SetUserName";

pub fn try_execute_builtin_function(
    tx_input: &TxInput,
    state: &mut BlockchainMock,
) -> Option<TxResult> {
    match tx_input.func_name.as_slice() {
        ESDT_TRANSFER_FUNC => Some(execute_esdt_transfer(tx_input, state)),
        SET_USERNAME_FUNC => Some(execute_set_username(tx_input, state)),
        _ => None,
    }
}
