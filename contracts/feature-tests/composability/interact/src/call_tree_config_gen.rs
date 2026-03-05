use std::collections::BTreeMap;

use crate::call_tree_config::{
    CallTreeConfig, ContractConfig, GatewayConfig, ProgrammedCallConfig, ProgrammedCallTypeConfig,
    StartCall,
};

/// Scenario 1: three root forwarders (async-v1, async-v2, sync) each calling one target,
/// plus a direct call with no further children.
///
/// ```toml
/// [gateway]
/// uri = "https://testnet-gateway.multiversx.com"
/// chain_type = "real"
///
/// [[start]]
/// to = "async_v1_root"
///
/// [[start]]
/// to = "async_v2_root"
///
/// [[start]]
/// to = "sync_root"
///
/// [[start]]
/// to = "direct"
///
/// [contracts.async_v1_root]
///
/// [[contracts.async_v1_root.calls]]
/// to = "async_v1_target"
/// call_type = "legacy_async"
///
/// [contracts.async_v1_target]
///
/// [contracts.async_v2_root]
///
/// [[contracts.async_v2_root.calls]]
/// to = "async_v2_target"
/// call_type = "promise"
///
/// [contracts.async_v2_target]
///
/// [contracts.direct]
///
/// [contracts.sync_root]
///
/// [[contracts.sync_root.calls]]
/// to = "sync_target"
/// call_type = "sync"
///
/// [contracts.sync_target]
/// ```
pub fn scenario_1() -> CallTreeConfig {
    CallTreeConfig {
        gateway: GatewayConfig::default(),
        start: vec![
            StartCall {
                to: "async_v1_root".to_string(),
                gas_limit: None,
                args: Vec::new(),
                payments: Vec::new(),
            },
            StartCall {
                to: "async_v2_root".to_string(),
                gas_limit: None,
                args: Vec::new(),
                payments: Vec::new(),
            },
            StartCall {
                to: "sync_root".to_string(),
                gas_limit: None,
                args: Vec::new(),
                payments: Vec::new(),
            },
            StartCall {
                to: "direct".to_string(),
                gas_limit: None,
                args: Vec::new(),
                payments: Vec::new(),
            },
        ],
        contracts: BTreeMap::from([
            (
                "async_v1_root".to_string(),
                ContractConfig {
                    address: None,
                    calls: vec![ProgrammedCallConfig {
                        to: "async_v1_target".to_string(),
                        call_type: ProgrammedCallTypeConfig::LegacyAsync,
                        gas_limit: None,
                        payments: Vec::new(),
                    }],
                },
            ),
            (
                "async_v1_target".to_string(),
                ContractConfig {
                    address: None,
                    calls: Vec::new(),
                },
            ),
            (
                "async_v2_root".to_string(),
                ContractConfig {
                    address: None,
                    calls: vec![ProgrammedCallConfig {
                        to: "async_v2_target".to_string(),
                        call_type: ProgrammedCallTypeConfig::Promise,
                        gas_limit: None,
                        payments: Vec::new(),
                    }],
                },
            ),
            (
                "async_v2_target".to_string(),
                ContractConfig {
                    address: None,
                    calls: Vec::new(),
                },
            ),
            (
                "direct".to_string(),
                ContractConfig {
                    address: None,
                    calls: Vec::new(),
                },
            ),
            (
                "sync_root".to_string(),
                ContractConfig {
                    address: None,
                    calls: vec![ProgrammedCallConfig {
                        to: "sync_target".to_string(),
                        call_type: ProgrammedCallTypeConfig::Sync,
                        gas_limit: None,
                        payments: Vec::new(),
                    }],
                },
            ),
            (
                "sync_target".to_string(),
                ContractConfig {
                    address: None,
                    calls: Vec::new(),
                },
            ),
        ]),
    }
}
