use std::{
    collections::BTreeMap,
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

/// Chain type for the gateway connection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChainType {
    Real,
    Simulator,
}

/// Gateway connection settings, stored in the `[gateway]` section of `call_tree.toml`.
#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayConfig {
    pub uri: String,
    pub chain_type: ChainType,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        GatewayConfig {
            uri: "https://testnet-gateway.multiversx.com".to_string(),
            chain_type: ChainType::Real,
        }
    }
}

impl GatewayConfig {
    pub fn use_chain_simulator(&self) -> bool {
        self.chain_type == ChainType::Simulator
    }
}

/// Equivalent of `ProgrammedCallType` from the forwarder-queue proxy, for TOML config.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgrammedCallTypeConfig {
    #[default]
    Sync,
    LegacyAsync,
    TransferExecute,
    Promise,
}

/// A token payment attached to a call.
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentConfig {
    /// `"EGLD"` or an ESDT token identifier.
    pub token_id: String,
    /// 0 for fungible / EGLD.
    pub nonce: u64,
    /// Amount as a decimal string.
    pub amount: String,
}

/// Equivalent of `ProgrammedCall` from the forwarder-queue proxy, for TOML config.
///
/// `to` is the name (map key) of the target contract in `CallTreeConfig::contracts`.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgrammedCallConfig {
    pub to: String,
    pub call_type: ProgrammedCallTypeConfig,
    pub gas_limit: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payments: Vec<PaymentConfig>,
}

/// An initial call that triggers the chain reaction.
///
/// Unlike [`ProgrammedCallConfig`], there is no `call_type` — start calls are plain
/// user transactions, not queued smart-contract calls.
///
/// `to` is the name (map key) of the target contract in `CallTreeConfig::contracts`.
#[derive(Debug, Serialize, Deserialize)]
pub struct StartCall {
    pub to: String,
    pub gas_limit: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payments: Vec<PaymentConfig>,
}

/// Serializable description of a single contract node in the call tree.
///
/// The contract name is the key in `CallTreeConfig::contracts`.
/// `index` is the on-chain numeric ID passed to `init`.
/// `children` is omitted when empty.
#[derive(Debug, Serialize, Deserialize)]
pub struct ContractConfig {
    /// Bech32 address; populated after deployment and saved back to the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<ProgrammedCallConfig>,
}

/// Serializable description of the whole call tree.
///
/// `contracts` is a map from contract name to its config.
/// Children and start calls reference contracts by name.
///
/// Example TOML for a root forwarder → leaf tree:
/// ```toml
/// [contracts.root]
/// index = 0
///
/// [[contracts.root.children]]
/// to = "leaf"
/// call_type = "legacy_async"
/// gas_limit = 10000000
///
/// [contracts.leaf]
/// index = 1
/// ```
pub const CALL_TREE_FILE: &str = "call_tree.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct CallTreeConfig {
    pub gateway: GatewayConfig,
    /// Initial transactions that trigger the chain reaction.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub start: Vec<StartCall>,
    /// Map from contract name to its configuration.
    pub contracts: BTreeMap<String, ContractConfig>,
}

impl CallTreeConfig {
    /// Serialize the call tree to a TOML file.
    pub fn save_to_file(&self, path: &str) {
        let toml_str = toml::to_string_pretty(self).expect("failed to serialize call tree");
        let mut file = std::fs::File::create(path).expect("failed to create call tree file");
        file.write_all(toml_str.as_bytes())
            .expect("failed to write call tree file");
    }

    /// Load a call tree from a TOML file.
    pub fn load_from_file(path: &str) -> Self {
        let mut file = std::fs::File::open(path).expect("failed to open call tree file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read call tree file");
        toml::from_str(&content).expect("failed to deserialize call tree")
    }

    /// Returns true if the given path exists.
    #[allow(dead_code)]
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }
}
