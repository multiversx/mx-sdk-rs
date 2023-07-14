use crate::types::VMAddress;

/// Holds the data for a MultiversX standard digital token transaction
#[derive(Clone, Default, Debug)]
pub struct EsdtInstanceMetadata {
    pub name: Vec<u8>,
    pub creator: Option<VMAddress>,
    pub royalties: u64,
    pub hash: Option<Vec<u8>>,
    pub uri: Vec<Vec<u8>>,
    pub attributes: Vec<u8>,
}
