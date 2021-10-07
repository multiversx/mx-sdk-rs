use num_bigint::BigUint;
use num_traits::Zero;

/// Holds the data for a Elrond standard digital token transaction
#[derive(Clone, Default, Debug)]
pub struct EsdtInstance {
    pub nonce: u64,
    pub balance: BigUint,
    pub creator: Option<Vec<u8>>,
    pub royalties: u64,
    pub hash: Option<Vec<u8>>,
    pub uri: Option<Vec<u8>>,
    pub attributes: Vec<u8>,
}

impl EsdtInstance {
    pub fn default(nonce: u64) -> Self {
        EsdtInstance {
            nonce: nonce,
            balance: BigUint::zero(),
            creator: None,
            royalties: 0,
            hash: None,
            uri: None,
            attributes: Vec::new(),
        }
    }

    pub fn fungible(balance: BigUint) -> Self {
        EsdtInstance {
            nonce: 0,
            balance,
            creator: None,
            royalties: 0,
            hash: None,
            uri: None,
            attributes: Vec::new(),
        }
    }

    pub fn is_empty_esdt(&self) -> bool {
        self.balance.is_zero()
    }
}
