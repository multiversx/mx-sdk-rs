use multiversx_sc_snippets::imports::Bech32Address;
use serde::{Deserialize, Serialize};

/// Basic Features Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    bf_address_storage_bytes: Option<Bech32Address>,
    bf_address: Option<Bech32Address>,
    bf_address_crypto: Option<Bech32Address>,
}

impl State {
    /// Sets the contract address for basic-features-storage-bytes
    pub fn set_bf_address_storage_bytes(&mut self, address: Bech32Address) {
        self.bf_address_storage_bytes = Some(address);
    }

    /// Sets the contract address for basic-features
    pub fn set_bf_address(&mut self, address: Bech32Address) {
        self.bf_address = Some(address);
    }

    /// Returns basic-features-storage-bytes contract
    pub fn bf_storage_bytes_contract(&self) -> &Bech32Address {
        self.bf_address_storage_bytes
            .as_ref()
            .expect("basic-features-storage-bytes contract not yet deployed")
    }

    /// Returns basic-features contract
    pub fn bf_contract(&self) -> &Bech32Address {
        self.bf_address
            .as_ref()
            .expect("basic-features contract not yet deployed")
    }
}
