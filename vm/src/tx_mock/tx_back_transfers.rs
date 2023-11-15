use num_bigint::BigUint;

use super::{TxResult, TxTokenTransfer};

#[derive(Default)]
pub struct BackTransfers {
    pub call_value: BigUint,
    pub esdt_transfers: Vec<TxTokenTransfer>,
}

impl BackTransfers {
    pub fn empty() -> Self {
        BackTransfers::default()
    }

    pub fn append_from_result(&mut self, _result: &TxResult) {
        todo!()
    }
}
