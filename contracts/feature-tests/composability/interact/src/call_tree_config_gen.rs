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

/// Scenario 2: a linear chain of `n` contracts, each calling the next via sync call.
///
/// Contract names are `"s2_{n-1}"`, `"s2_{n-2}"`, ..., `"s2_0"`.
/// The start call goes to `"s2_{n-1}"`, which calls `"s2_{n-2}"` synchronously, and so on,
/// until `"s2_0"` which has no further calls.
pub fn scenario_2(n: usize) -> CallTreeConfig {
    assert!(n >= 1, "chain length must be at least 1");

    let start = vec![StartCall {
        to: format!("s2_{}", n - 1),
        gas_limit: None,
        args: Vec::new(),
        payments: Vec::new(),
    }];

    let mut contracts = BTreeMap::new();
    for i in 0..n {
        let name = format!("s2_{i}");
        let calls = if i > 0 {
            vec![ProgrammedCallConfig {
                to: format!("s2_{}", i - 1),
                call_type: ProgrammedCallTypeConfig::Sync,
                gas_limit: None,
                payments: Vec::new(),
            }]
        } else {
            Vec::new()
        };
        contracts.insert(
            name,
            ContractConfig {
                address: None,
                calls,
            },
        );
    }

    CallTreeConfig {
        gateway: GatewayConfig::default(),
        start,
        contracts,
    }
}
