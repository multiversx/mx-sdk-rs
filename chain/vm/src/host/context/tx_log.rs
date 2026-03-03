use crate::types::Address;

use super::TxFunctionName;

#[derive(Clone, Debug)]
pub struct TxLog {
    pub address: Address,
    pub endpoint: TxFunctionName,
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<Vec<u8>>,
}
