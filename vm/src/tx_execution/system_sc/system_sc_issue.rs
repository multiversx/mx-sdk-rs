use crate::tx_mock::{TxContext, TxResult};

pub fn issue(_tx_context: TxContext) -> (TxContext, TxResult) {
    panic!("System SC issue not implemented")
}

pub fn issue_semi_fungible(_tx_context: TxContext) -> (TxContext, TxResult) {
    panic!("System SC issue_semi_fungible not implemented")
}

pub fn issue_non_fungible(_tx_context: TxContext) -> (TxContext, TxResult) {
    panic!("System SC issue_non_fungible not implemented")
}
