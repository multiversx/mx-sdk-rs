use crate::scenario::model::Checkable;
use alloc::vec::Vec;
use multiversx_sc::types::heap::Address;

use super::TxFunctionName;

#[derive(Clone, Debug)]
pub struct TxLog {
    pub address: Address,
    pub endpoint: TxFunctionName,
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

impl TxLog {
    pub fn scenario_check(&self, check_log: &crate::scenario::model::CheckLog) -> bool {
        check_log.address.check(self.address.as_bytes())
            && check_log.endpoint.check(&self.endpoint)
            && check_log.topics.check(self.topics.as_slice())
            && check_log.data.check(self.data.as_slice())
    }
}
