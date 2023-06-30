use crate::types::VMAddress;

use super::TxFunctionName;

#[derive(Clone, Debug)]
pub struct TxLog {
    pub address: VMAddress,
    pub endpoint: TxFunctionName,
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}
