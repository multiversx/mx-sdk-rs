use std::{
    collections::BTreeMap,
    io::{Read, Write},
    path::Path,
};

use multiversx_sc::types::ShardId;
use serde::{Deserialize, Serialize};

/// Top-level config file path.
pub const CONFIG_FILE: &str = "config.toml";

/// Default call tree layout file path (used when `config.toml` is absent).
pub const CALL_TREE_FILE: &str = "call_tree.toml";

/// Call tree state file (deployed addresses), always `current.toml`.
pub const STATE_FILE: &str = "current.toml";

/// Chain type for the gateway connection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChainType {
    Real,
    Simulator,
}

/// Gateway connection settings.
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

/// Top-level interactor config, stored in `config.toml`.
///
/// Contains the gateway settings and the path to the call tree layout file.
#[derive(Debug, Serialize, Deserialize)]
pub struct InteractConfig {
    pub gateway: GatewayConfig,
    /// Path to the call tree layout TOML (defaults to `"call_tree.toml"`).
    #[serde(default = "default_call_tree_path")]
    pub call_tree_path: String,
}

fn default_call_tree_path() -> String {
    CALL_TREE_FILE.to_string()
}

impl Default for InteractConfig {
    fn default() -> Self {
        InteractConfig {
            gateway: GatewayConfig::default(),
            call_tree_path: CALL_TREE_FILE.to_string(),
        }
    }
}

impl InteractConfig {
    pub fn load_from_file(path: &str) -> Self {
        if !Path::new(path).exists() {
            return Self::default();
        }
        let mut file = std::fs::File::open(path).expect("failed to open config file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read config file");
        toml::from_str(&content).expect("failed to deserialize config")
    }

    #[allow(dead_code)]
    pub fn save_to_file(&self, path: &str) {
        let toml_str = toml::to_string_pretty(self).expect("failed to serialize config");
        let mut file = std::fs::File::create(path).expect("failed to create config file");
        file.write_all(toml_str.as_bytes())
            .expect("failed to write config file");
    }
}

/// Equivalent of `ProgrammedCallType` from the forwarder-net proxy, for TOML config.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgrammedCallTypeConfig {
    #[default]
    Sync,
    LegacyAsync,
    TransfExec,
    Promise,
}

/// A token payment attached to a call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentConfig {
    /// `"EGLD"` or an ESDT token identifier.
    pub token_id: String,
    /// 0 for fungible / EGLD.
    pub nonce: u64,
    /// Amount as a decimal string.
    pub amount: String,
}

/// Equivalent of `ProgrammedCall` from the forwarder-net proxy, for TOML config.
///
/// `to` is the name (map key) of the target contract in `CallTreeLayout::contracts`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgrammedCallConfig {
    pub to: String,
    pub call_type: ProgrammedCallTypeConfig,
    /// Estimated gas; filled by the `estimate-gas` command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payments: Vec<PaymentConfig>,
}

/// An initial call that triggers the chain reaction.
///
/// Unlike [`ProgrammedCallConfig`], there is no `call_type` — start calls are plain
/// user transactions, not queued smart-contract calls.
///
/// `to` is the name (map key) of the target contract in `CallTreeLayout::contracts`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartCall {
    pub to: String,
    /// Shard of the wallet used to send this transaction. Defaults to shard 0 when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard: Option<ShardId>,
    /// Estimated gas; filled by the `estimate-gas` command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payments: Vec<PaymentConfig>,
}

/// Serializable description of a single contract node in the call tree.
///
/// The contract name is the key in `CallTreeLayout::contracts`.
/// `address` is `None` in the layout file and `Some(bech32)` in the state file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractConfig {
    /// Shard of the wallet that deploys and manages this contract. Defaults to shard 0 when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard: Option<ShardId>,
    /// Whether the contract accepts EGLD/ESDT payments. Defaults to `false` when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payable: Option<bool>,
    /// Deployed bech32 address. `None` in the layout; `Some(...)` in the state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub calls: Vec<ProgrammedCallConfig>,
}

/// Serializable description of the whole call tree layout (no addresses).
///
/// `contracts` is a map from contract name to its config.
/// Children and start calls reference contracts by name.
///
/// Example TOML for a root forwarder → leaf tree:
/// ```toml
/// [[start]]
/// to = "root"
///
/// [contracts.root]
/// [[contracts.root.calls]]
/// to = "leaf"
/// call_type = "legacy_async"
/// gas_limit = 10000000
///
/// [contracts.leaf]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallTreeLayout {
    /// Initial transactions that trigger the chain reaction.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub start: Vec<StartCall>,
    /// Map from contract name to its layout configuration.
    pub contracts: BTreeMap<String, ContractConfig>,
}

impl CallTreeLayout {
    /// Serialize the call tree layout to a TOML file.
    pub fn save_to_file(&self, path: &str) {
        const HEADER: &str = "# Code generated by mesh-interactor. DO NOT EDIT.\n\n";
        let toml_str = toml::to_string_pretty(self).expect("failed to serialize call tree layout");
        let mut file = std::fs::File::create(path).expect("failed to create call tree layout file");
        file.write_all(HEADER.as_bytes())
            .expect("failed to write call tree layout file");
        file.write_all(toml_str.as_bytes())
            .expect("failed to write call tree layout file");
    }

    /// Load a call tree layout from a TOML file.
    pub fn load_from_file(path: &str) -> Self {
        let mut file = std::fs::File::open(path).expect("failed to open call tree layout file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read call tree layout file");
        toml::from_str(&content).expect("failed to deserialize call tree layout")
    }
}
