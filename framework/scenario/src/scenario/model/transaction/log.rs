use multiversx_sc::types::Address;

#[derive(Debug, Clone)]
pub struct Log {
    pub address: Address,
    pub endpoint: String,
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<Vec<u8>>,
}
