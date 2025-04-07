use num_bigint::BigUint;
use num_traits::Zero;

use super::EsdtInstanceMetadata;

/// Holds the data for a MultiversX standard digital token transaction
#[derive(Clone, Default, Debug)]
pub struct EsdtInstance {
    pub nonce: u64,
    pub balance: BigUint,
    pub metadata: EsdtInstanceMetadata,
}

impl EsdtInstance {
    pub fn default(nonce: u64) -> Self {
        EsdtInstance {
            nonce,
            balance: BigUint::zero(),
            metadata: EsdtInstanceMetadata::default(),
        }
    }

    pub fn fungible(balance: BigUint) -> Self {
        EsdtInstance {
            nonce: 0,
            balance,
            metadata: EsdtInstanceMetadata::default(),
        }
    }

    pub fn is_empty_esdt(&self) -> bool {
        self.balance.is_zero()
    }
}
