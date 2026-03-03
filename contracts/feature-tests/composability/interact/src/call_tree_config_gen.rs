use crate::call_tree_config::{
    CallTreeConfig, CallType, ChildCall, ContractConfig, GatewayConfig, StartCall,
};

/// Scenario 1: a single root forwarder that calls `accept_funds` on one vault.
///
/// ```toml
/// [gateway]
/// uri = "https://testnet-gateway.multiversx.com"
/// chain_type = "real"
///
/// [[contracts]]
/// index = 0
/// name = "root"
/// kind = "forwarder"
///
/// [[contracts.children]]
/// to = 1
/// call_type = "legacy_async"
/// gas_limit = 10000000
/// endpoint_name = "accept_funds"
///
/// [[contracts]]
/// index = 1
/// name = "vault"
/// kind = "vault"
/// ```
pub fn scenario_1() -> CallTreeConfig {
    CallTreeConfig {
        gateway: GatewayConfig::default(),
        start: vec![StartCall {
            to: 0,
            gas_limit: 70_000_000,
            args: Vec::new(),
            payments: Vec::new(),
        }],
        contracts: vec![
            ContractConfig {
                index: 0,
                name: "root".to_string(),
                address: None,
                children: vec![ChildCall {
                    to: 1,
                    call_type: CallType::LegacyAsync,
                    gas_limit: 10_000_000,
                    args: Vec::new(),
                    payments: Vec::new(),
                }],
            },
            ContractConfig {
                index: 1,
                name: "leaf".to_string(),
                address: None,
                children: Vec::new(),
            },
        ],
    }
}
