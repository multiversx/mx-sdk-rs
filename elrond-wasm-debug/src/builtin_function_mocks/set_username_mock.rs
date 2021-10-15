use std::rc::Rc;

use crate::{
    tx_mock::{TxInput, TxResult},
    world_mock::BlockchainMock,
};

pub fn execute_set_username(tx_input: &TxInput, state: &mut Rc<BlockchainMock>) -> TxResult {
    assert_eq!(tx_input.args.len(), 1, "SetUserName expects 1 argument");
    if Rc::get_mut(state)
        .unwrap()
        .try_set_username(&tx_input.to, tx_input.args[0].as_slice())
    {
        TxResult::empty()
    } else {
        TxResult {
            result_status: 10,
            result_message: b"username already set".to_vec(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
        }
    }
}
