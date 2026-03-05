use std::collections::BTreeMap;

use crate::call_tree_config::{
    CallTreeConfig, CallType, ChildCall, ContractConfig, GatewayConfig, StartCall,
};

/// Scenario 1: a single root forwarder that calls bump on one leaf.
///
/// ```toml
/// [gateway]
/// uri = "https://testnet-gateway.multiversx.com"
/// chain_type = "real"
///
/// [[start]]
/// to = "root"
/// gas_limit = 70000000
///
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
pub fn scenario_1() -> CallTreeConfig {
    CallTreeConfig {
        gateway: GatewayConfig::default(),
        start: vec![StartCall {
            to: "root".to_string(),
            gas_limit: 70_000_000,
            args: Vec::new(),
            payments: Vec::new(),
        }],
        contracts: BTreeMap::from([
            (
                "root".to_string(),
                ContractConfig {
                    address: None,
                    children: vec![ChildCall {
                        to: "leaf".to_string(),
                        call_type: CallType::LegacyAsync,
                        gas_limit: 10_000_000,
                        payments: Vec::new(),
                    }],
                },
            ),
            (
                "leaf".to_string(),
                ContractConfig {
                    address: None,
                    children: Vec::new(),
                },
            ),
        ]),
    }
}
