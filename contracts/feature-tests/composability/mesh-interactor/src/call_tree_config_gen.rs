use std::collections::BTreeMap;

use multiversx_sc::types::ShardId;

use crate::call_tree_config::{
    CallTreeLayout, ContractConfig, PaymentConfig, ProgrammedCallConfig, ProgrammedCallTypeConfig,
    StartCall,
};

const LAYOUTS: &str = "layouts";
const ASYNC_LAYOUT: &str = "layouts/async.toml";
const ASYNC_PAY_LAYOUT: &str = "layouts/async_pay1.toml";
const TRANSF_EXEC_SHARDED_LAYOUT: &str = "layouts/transf_exec_sharded.toml";
const SYNC_CHAIN_LAYOUT: &str = "layouts/sync_chain.toml";
const ASYNC_ALL_SHARDS_LAYOUT: &str = "layouts/async_all_shards.toml";
const ASYNC_PAY_ALL_SHARDS_LAYOUT: &str = "layouts/async_pay_all_shards.toml";
const TRANSF_EXEC_ALL_SHARDS_LAYOUT: &str = "layouts/transf_exec_all_shards.toml";

pub fn generate_layouts(n: usize) {
    std::fs::create_dir_all(LAYOUTS).expect("failed to create layouts/ directory");

    let mut async_sharded = async_no_payment();
    async_sharded.fill_gas_estimates();
    async_sharded.save_to_file(ASYNC_LAYOUT);
    println!("Async sharded layout saved to {ASYNC_LAYOUT}");

    let mut async_pay = async_with_payments();
    async_pay.fill_gas_estimates();
    async_pay.save_to_file(ASYNC_PAY_LAYOUT);
    println!("Async sharded layout with payments saved to {ASYNC_PAY_LAYOUT}");

    let mut transf_exec_sharded = transf_exec();
    transf_exec_sharded.fill_gas_estimates();
    transf_exec_sharded.save_to_file(TRANSF_EXEC_SHARDED_LAYOUT);
    println!("Transfer-execute sharded layout saved to {TRANSF_EXEC_SHARDED_LAYOUT}");

    let mut sync_chain = sync_chain(n);
    sync_chain.fill_gas_estimates();
    sync_chain.save_to_file(SYNC_CHAIN_LAYOUT);
    println!("Sync chain layout (n={n}) saved to {SYNC_CHAIN_LAYOUT}");

    let mut async_all_shards = async_no_payment_all_shards();
    async_all_shards.fill_gas_estimates();
    async_all_shards.save_to_file(ASYNC_ALL_SHARDS_LAYOUT);
    println!("Async all-shards layout saved to {ASYNC_ALL_SHARDS_LAYOUT}");

    let mut async_pay_all_shards = async_with_payments_all_shards();
    async_pay_all_shards.fill_gas_estimates();
    async_pay_all_shards.save_to_file(ASYNC_PAY_ALL_SHARDS_LAYOUT);
    println!("Async all-shards layout with payments saved to {ASYNC_PAY_ALL_SHARDS_LAYOUT}");

    let mut transf_exec_all_shards = transf_exec_all_shards();
    transf_exec_all_shards.fill_gas_estimates();
    transf_exec_all_shards.save_to_file(TRANSF_EXEC_ALL_SHARDS_LAYOUT);
    println!("Transfer-execute all-shards layout saved to {TRANSF_EXEC_ALL_SHARDS_LAYOUT}");
}

struct ShardVariant {
    suffix: String,
    sender_shard: u32,
    root_shard: u32,
    target_shard: u32,
}

impl ShardVariant {
    fn new(sender_shard: u32, root_shard: u32, target_shard: u32) -> Self {
        ShardVariant {
            suffix: format!("s{sender_shard}{root_shard}{target_shard}"),
            sender_shard,
            root_shard,
            target_shard,
        }
    }
}

fn three_shard_variants() -> Vec<ShardVariant> {
    vec![
        ShardVariant::new(2, 2, 2),
        ShardVariant::new(0, 2, 2),
        ShardVariant::new(0, 1, 2),
        ShardVariant::new(0, 1, 0),
    ]
}

/// All 27 permutations of (sender_shard, root_shard, target_shard) for shards 0, 1, 2.
fn all_shard_variants() -> Vec<ShardVariant> {
    let mut variants = Vec::new();
    for sender in 0u32..=2 {
        for root in 0u32..=2 {
            for target in 0u32..=2 {
                variants.push(ShardVariant::new(sender, root, target));
            }
        }
    }
    variants
}

/// Scenario 1: async-v1 and async-v2, parametric over a set of shard variants.
///
/// Call types covered: `async_v1` (legacy_async) and `async_v2` (promise).
fn async_impl(
    variants: Vec<ShardVariant>,
    start_payments: Vec<PaymentConfig>,
    call_payments: Vec<PaymentConfig>,
) -> CallTreeLayout {
    let async_call_types: &[(&str, ProgrammedCallTypeConfig)] = &[
        ("async_v1", ProgrammedCallTypeConfig::AsyncV1),
        ("async_v2", ProgrammedCallTypeConfig::AsyncV2),
    ];

    let mut start = Vec::new();
    let mut contracts = BTreeMap::new();

    for (type_name, call_type) in async_call_types {
        for v in &variants {
            let root_name = format!("{}_root_{}", type_name, v.suffix);
            let target_name = format!("{}_target_{}", type_name, v.suffix);

            start.push(StartCall::new(
                root_name.clone(),
                v.sender_shard,
                start_payments.clone(),
            ));

            contracts.insert(
                root_name,
                ContractConfig {
                    shard: Some(v.root_shard.into()),
                    payable: None,
                    address: None,
                    calls: vec![ProgrammedCallConfig {
                        to: target_name.clone(),
                        call_type: call_type.clone(),
                        gas_limit: None,
                        payments: call_payments.clone(),
                    }],
                },
            );

            contracts.insert(
                target_name,
                ContractConfig {
                    shard: Some(v.target_shard.into()),
                    payable: None,
                    address: None,
                    calls: Vec::new(),
                },
            );
        }
    }

    CallTreeLayout { start, contracts }
}

/// Async-v1 and async-v2, minimal shard layouts, no payments.
pub fn async_no_payment() -> CallTreeLayout {
    async_impl(three_shard_variants(), Vec::new(), Vec::new())
}

/// Async-v1 and async-v2, minimal shard layouts, with an EGLD payment forwarded on each async call.
pub fn async_with_payments() -> CallTreeLayout {
    async_impl(
        three_shard_variants(),
        vec![PaymentConfig {
            token_id: "EGLD-000000".to_string(),
            nonce: 0,
            amount: "23000000".to_string(),
        }],
        vec![PaymentConfig {
            token_id: "EGLD-000000".to_string(),
            nonce: 0,
            amount: "12000000".to_string(),
        }],
    )
}

/// Async-v1 and async-v2, all 27 shard permutations, no payments.
pub fn async_no_payment_all_shards() -> CallTreeLayout {
    async_impl(all_shard_variants(), Vec::new(), Vec::new())
}

/// Async-v1 and async-v2, all 27 shard permutations, with an EGLD payment forwarded on each async call.
pub fn async_with_payments_all_shards() -> CallTreeLayout {
    async_impl(
        all_shard_variants(),
        vec![PaymentConfig {
            token_id: "EGLD-000000".to_string(),
            nonce: 0,
            amount: "23000000".to_string(),
        }],
        vec![PaymentConfig {
            token_id: "EGLD-000000".to_string(),
            nonce: 0,
            amount: "12000000".to_string(),
        }],
    )
}

/// Transfer-execute tested in minimal shard layouts.
pub fn transf_exec() -> CallTreeLayout {
    transf_exec_impl(three_shard_variants())
}

/// Transfer-execute tested in all 27 shard permutations.
pub fn transf_exec_all_shards() -> CallTreeLayout {
    transf_exec_impl(all_shard_variants())
}

fn transf_exec_impl(variants: Vec<ShardVariant>) -> CallTreeLayout {
    let mut start = Vec::new();
    let mut contracts = BTreeMap::new();

    for v in &variants {
        let root_name = format!("transf_exec_root_{}", v.suffix);
        let target_name = format!("transf_exec_target_{}", v.suffix);

        start.push(StartCall::new(
            root_name.clone(),
            v.sender_shard,
            Vec::new(),
        ));

        contracts.insert(
            root_name,
            ContractConfig {
                shard: Some(v.root_shard.into()),
                payable: Some(true),
                address: None,
                calls: vec![ProgrammedCallConfig {
                    to: target_name.clone(),
                    call_type: ProgrammedCallTypeConfig::TransfExec,
                    gas_limit: None,
                    payments: Vec::new(),
                }],
            },
        );

        contracts.insert(
            target_name,
            ContractConfig {
                shard: Some(v.target_shard.into()),
                payable: None,
                address: None,
                calls: Vec::new(),
            },
        );
    }

    CallTreeLayout { start, contracts }
}

/// Scenario 2: a linear chain of `n` contracts, each calling the next via sync call.
///
/// Contract names are `"s2_{n-1}"`, `"s2_{n-2}"`, ..., `"s2_0"`.
/// The start call goes to `"s2_{n-1}"`, which calls `"s2_{n-2}"` synchronously, and so on,
/// until `"s2_0"` which has no further calls.
pub fn sync_chain(n: usize) -> CallTreeLayout {
    assert!(n >= 1, "chain length must be at least 1");

    let start = vec![StartCall::new(format!("s2_{}", n - 1), 2, Vec::new())];

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
                payable: None,
                address: None,
                calls,
            },
        );
    }

    CallTreeLayout { start, contracts }
}
