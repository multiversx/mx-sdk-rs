use elrond_wasm::types::Address;

/// Holds the data for a Elrond standard digital token transaction
#[derive(Clone, Default, Debug)]
pub struct EsdtInstanceMetadata {
    pub name: Vec<u8>,
    pub creator: Option<Address>,
    pub royalties: u64,
    pub hash: Option<Vec<u8>>,
    pub uri: Option<Vec<u8>>,
    pub attributes: Vec<u8>,
}
