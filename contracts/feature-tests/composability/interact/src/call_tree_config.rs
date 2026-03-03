use std::{
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

/// Equivalent of `QueuedCallType` from the forwarder-queue proxy, for TOML config.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallType {
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

/// Equivalent of `QueuedCall` from the forwarder-queue proxy, for TOML config.
///
/// `to` is the `index` of the target contract in `CallTreeConfig::contracts`.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChildCall {
    pub to: usize,
    pub call_type: CallType,
    pub gas_limit: u64,
    pub endpoint_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payments: Vec<PaymentConfig>,
}

/// An initial call that triggers the chain reaction.
///
/// Unlike [`ChildCall`], there is no `call_type` — start calls are plain
/// user transactions, not queued smart-contract calls.
///
/// `to` is the `index` of the target contract in `CallTreeConfig::contracts`.
#[derive(Debug, Serialize, Deserialize)]
pub struct StartCall {
    pub to: usize,
    pub gas_limit: u64,
    pub endpoint_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payments: Vec<PaymentConfig>,
}

/// Discriminates between a forwarder-queue contract and a vault.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContractKind {
    Forwarder,
    Vault,
}

/// Serializable description of a single contract node in the call tree.
///
/// `index` is an explicit identifier used in child references.
/// `children` holds `index` values of child contracts and is only
/// meaningful for `Forwarder` nodes; it is omitted when empty.
#[derive(Debug, Serialize, Deserialize)]
pub struct ContractConfig {
    pub index: usize,
    pub name: String,
    pub kind: ContractKind,
    /// Bech32 address; populated after deployment and saved back to the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<ChildCall>,
}

/// Serializable description of the whole call tree.
///
/// `contracts[0]` is always the root (must be a `Forwarder`).
/// Children are referenced by index into this same list.
///
/// Example TOML for a root forwarder → fwd1 → vault1 tree:
/// ```toml
/// [[contracts]]
/// index = 0
/// name = "root"
/// kind = "forwarder"
///
/// [[contracts.children]]
/// to = 1
/// call_type = "legacy_async"
/// gas_limit = 10000000
/// endpoint_name = "forward_queued_calls"
///
/// [[contracts]]
/// index = 1
/// name = "fwd1"
/// kind = "forwarder"
///
/// [[contracts.children]]
/// to = 2
/// call_type = "legacy_async"
/// gas_limit = 10000000
/// endpoint_name = "accept_funds"
///
/// [[contracts]]
/// index = 2
/// name = "vault1"
/// kind = "vault"
/// ```
pub const CALL_TREE_FILE: &str = "call_tree.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct CallTreeConfig {
    pub gateway: GatewayConfig,
    /// Initial transactions that trigger the chain reaction.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub start: Vec<StartCall>,
    pub contracts: Vec<ContractConfig>,
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
