use multiversx_sc::types::heap::Address;

use super::TxFunctionName;

#[derive(Clone, Debug)]
pub struct TxLog {
    pub address: Address,
    pub endpoint: TxFunctionName,
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}
