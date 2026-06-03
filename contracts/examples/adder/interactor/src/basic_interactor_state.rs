use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};

/// Adder Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub adder_address: Option<Bech32Address>,
}

impl State {
    /// Returns the adder contract
    pub fn current_adder_address(&self) -> &Bech32Address {
        self.adder_address
            .as_ref()
            .expect("no known adder contract, deploy first")
    }
}
