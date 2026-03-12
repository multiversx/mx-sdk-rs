use std::collections::BTreeMap;

use multiversx_sc::types::ShardId;

use crate::call_tree_config::{
    CallTreeLayout, ContractConfig, ProgrammedCallConfig, ProgrammedCallTypeConfig, StartCall,
};

/// Scenario 1: async-v1, async-v2 and transfer-execute each tested in three shard layouts,
/// plus a sync call entirely within shard 2.
///
/// The three shard variants (sender → root → target) are:
/// - `_s222`:  2 → 2 → 2  (all in shard 2)
/// - `_s022`:  0 → 2 → 2  (sender in shard 0, root and target in shard 2)
/// - `_s012`:  0 → 1 → 2  (sender shard 0, root shard 1, target shard 2)
///
/// Call types covered: `async_v1` (legacy_async), `async_v2` (promise),
/// `transfer_execute`, and `sync` (shard 2 only).
pub fn async_sharded() -> CallTreeLayout {
    struct ShardVariant {
        suffix: &'static str,
        sender_shard: u32,
        root_shard: u32,
        target_shard: u32,
    }

    let variants = [
        ShardVariant {
            suffix: "s222",
            sender_shard: 2,
            root_shard: 2,
            target_shard: 2,
        },
        ShardVariant {
            suffix: "s022",
            sender_shard: 0,
            root_shard: 2,
            target_shard: 2,
        },
        ShardVariant {
            suffix: "s012",
            sender_shard: 0,
            root_shard: 1,
            target_shard: 2,
        },
    ];

    let async_call_types: &[(&str, ProgrammedCallTypeConfig)] = &[
        ("async_v1", ProgrammedCallTypeConfig::LegacyAsync),
        ("async_v2", ProgrammedCallTypeConfig::Promise),
        ("transf_exec", ProgrammedCallTypeConfig::TransfExec),
    ];

    let mut start = Vec::new();
    let mut contracts = BTreeMap::new();

    for (type_name, call_type) in async_call_types {
        for v in &variants {
            let root_name = format!("{}_root_{}", type_name, v.suffix);
            let target_name = format!("{}_target_{}", type_name, v.suffix);

            start.push(StartCall {
                to: root_name.clone(),
                shard: Some(v.sender_shard.into()),
                gas_limit: None,
                args: Vec::new(),
                payments: Vec::new(),
            });

            contracts.insert(
                root_name,
                ContractConfig {
                    shard: Some(v.root_shard.into()),
                    address: None,
                    calls: vec![ProgrammedCallConfig {
                        to: target_name.clone(),
                        call_type: call_type.clone(),
                        gas_limit: None,
                        payments: Vec::new(),
                    }],
                },
            );

            contracts.insert(
                target_name,
                ContractConfig {
                    shard: Some(v.target_shard.into()),
                    address: None,
                    calls: Vec::new(),
                },
            );
        }
    }

    // Sync call: all in shard 2
    start.push(StartCall {
        to: "sync_root".to_string(),
        shard: Some(2u32.into()),
        gas_limit: None,
        args: Vec::new(),
        payments: Vec::new(),
    });
    contracts.insert(
        "sync_root".to_string(),
        ContractConfig {
            shard: Some(2u32.into()),
            address: None,
            calls: vec![ProgrammedCallConfig {
                to: "sync_target".to_string(),
                call_type: ProgrammedCallTypeConfig::Sync,
                gas_limit: None,
                payments: Vec::new(),
            }],
        },
    );
    contracts.insert(
        "sync_target".to_string(),
        ContractConfig {
            shard: Some(2u32.into()),
            address: None,
            calls: Vec::new(),
        },
    );

    CallTreeLayout { start, contracts }
}

/// Scenario 2: a linear chain of `n` contracts, each calling the next via sync call.
///
/// Contract names are `"s2_{n-1}"`, `"s2_{n-2}"`, ..., `"s2_0"`.
/// The start call goes to `"s2_{n-1}"`, which calls `"s2_{n-2}"` synchronously, and so on,
/// until `"s2_0"` which has no further calls.
pub fn sync_chain(n: usize) -> CallTreeLayout {
    assert!(n >= 1, "chain length must be at least 1");

    let start = vec![StartCall {
        to: format!("s2_{}", n - 1),
        shard: Some(ShardId::from(2)),
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
                shard: Some(ShardId::from(2)),
                address: None,
                calls,
            },
        );
    }

    CallTreeLayout { start, contracts }
}
