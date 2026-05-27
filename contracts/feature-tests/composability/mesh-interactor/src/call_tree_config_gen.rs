use std::collections::BTreeMap;

use multiversx_sc::types::ShardId;

use crate::call_tree_config::{
    CallTreeLayout, ContractConfig, PaymentConfig, ProgrammedCallConfig, ProgrammedCallTypeConfig,
    StartCall,
};

const LAYOUTS: &str = "layouts";

// async_v1, 1 child
const ASYNC_V1_C1_SM_LAYOUT: &str = "layouts/async_v1_c1_smin.toml";
const ASYNC_V1_C1_PAY_SM_LAYOUT: &str = "layouts/async_v1_c1_smin_pay.toml";
const ASYNC_V1_C1_SX_LAYOUT: &str = "layouts/async_v1_c1_sall.toml";
const ASYNC_V1_C1_PAY_SX_LAYOUT: &str = "layouts/async_v1_c1_sall_pay.toml";

// async_v2, 1 child
const ASYNC_V2_C1_SM_LAYOUT: &str = "layouts/async_v2_c1_smin.toml";
const ASYNC_V2_C1_PAY_SM_LAYOUT: &str = "layouts/async_v2_c1_smin_pay.toml";
const ASYNC_V2_C1_SX_LAYOUT: &str = "layouts/async_v2_c1_sall.toml";
const ASYNC_V2_C1_PAY_SX_LAYOUT: &str = "layouts/async_v2_c1_sall_pay.toml";

// async_v2, 3 children
const ASYNC_V2_C3_LAYOUT: &str = "layouts/async_v2_c3.toml";
const ASYNC_V2_C3_PAY_LAYOUT: &str = "layouts/async_v2_c3_pay.toml";

// transf_exec, 1 child
const TRANSF_EXEC_C1_SM_LAYOUT: &str = "layouts/transf_exec_c1_smin.toml";
const TRANSF_EXEC_C1_SX_LAYOUT: &str = "layouts/transf_exec_c1_sall.toml";

// transf_exec, 3 children
const TRANSF_EXEC_C3_LAYOUT: &str = "layouts/transf_exec_c3.toml";

// sync chain
const SYNC_CHAIN_LAYOUT: &str = "layouts/sync_chain.toml";

fn save_layout(layout: CallTreeLayout, path: &str) {
    let mut layout = layout;
    layout.fill_gas_estimates();
    layout.save_to_file(path);
    println!("Layout saved to {path}");
}

pub fn generate_layouts(n: usize) {
    std::fs::create_dir_all(LAYOUTS).expect("failed to create layouts/ directory");

    // async_v1, 1 child (legacy async only supports a single child)
    save_layout(async_v1_c1(), ASYNC_V1_C1_SM_LAYOUT);
    save_layout(async_v1_c1_pay(), ASYNC_V1_C1_PAY_SM_LAYOUT);
    save_layout(async_v1_c1_all_shards(), ASYNC_V1_C1_SX_LAYOUT);
    save_layout(async_v1_c1_pay_all_shards(), ASYNC_V1_C1_PAY_SX_LAYOUT);

    // async_v2, 1 child
    save_layout(async_v2_c1(), ASYNC_V2_C1_SM_LAYOUT);
    save_layout(async_v2_c1_pay(), ASYNC_V2_C1_PAY_SM_LAYOUT);
    save_layout(async_v2_c1_all_shards(), ASYNC_V2_C1_SX_LAYOUT);
    save_layout(async_v2_c1_pay_all_shards(), ASYNC_V2_C1_PAY_SX_LAYOUT);

    // async_v2, 3 children (one per shard)
    save_layout(async_v2_c3(), ASYNC_V2_C3_LAYOUT);
    save_layout(async_v2_c3_pay(), ASYNC_V2_C3_PAY_LAYOUT);

    // transf_exec, 1 child
    save_layout(transf_exec_c1(), TRANSF_EXEC_C1_SM_LAYOUT);
    save_layout(transf_exec_c1_all_shards(), TRANSF_EXEC_C1_SX_LAYOUT);

    // transf_exec, 3 children (one per shard)
    save_layout(transf_exec_c3(), TRANSF_EXEC_C3_LAYOUT);

    // sync chain
    save_layout(sync_chain(n), SYNC_CHAIN_LAYOUT);
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

/// Shard variant for c3 scenarios: one root shard, children spread across all 3 shards.
struct C3ShardVariant {
    suffix: String,
    sender_shard: u32,
    root_shard: u32,
    // children are always placed on shards 0, 1, 2
}

impl C3ShardVariant {
    fn new(sender_shard: u32, root_shard: u32) -> Self {
        C3ShardVariant {
            suffix: format!("s{sender_shard}{root_shard}"),
            sender_shard,
            root_shard,
        }
    }
}

/// One variant per root shard (sender = root).
fn c3_shard_variants() -> Vec<C3ShardVariant> {
    vec![
        C3ShardVariant::new(0, 0),
        C3ShardVariant::new(1, 1),
        C3ShardVariant::new(2, 2),
    ]
}

fn egld_payment() -> PaymentConfig {
    PaymentConfig {
        token_id: "EGLD-000000".to_string(),
        nonce: 0,
        amount: "12000000".to_string(),
    }
}

fn egld_start_payment() -> PaymentConfig {
    PaymentConfig {
        token_id: "EGLD-000000".to_string(),
        nonce: 0,
        amount: "23000000".to_string(),
    }
}

fn egld_return_payment() -> PaymentConfig {
    PaymentConfig {
        token_id: "EGLD-000000".to_string(),
        nonce: 0,
        amount: "1".to_string(),
    }
}

/// Generic async scenario: one root contract calls `num_children` children via `call_type`.
///
/// Contract names follow the pattern `{label}_root_{suffix}` for the root and
/// `{label}_target_{i}_{suffix}` for each child (index `i` in `0..num_children`).
/// All children are deployed on `target_shard`.
///
/// `label` should encode both the call type and child count, e.g. `"async_v2_c3"`.
fn async_impl(
    label: &str,
    call_type: ProgrammedCallTypeConfig,
    num_children: usize,
    variants: Vec<ShardVariant>,
    start_payments: Vec<PaymentConfig>,
    call_payments: Vec<PaymentConfig>,
    target_returns: Vec<PaymentConfig>,
) -> CallTreeLayout {
    assert!(num_children >= 1, "num_children must be at least 1");

    let mut start = Vec::new();
    let mut contracts = BTreeMap::new();

    for v in &variants {
        let root_name = format!("{label}_root_{}", v.suffix);

        start.push(StartCall::new(
            root_name.clone(),
            v.sender_shard,
            start_payments.clone(),
        ));

        let calls = (0..num_children)
            .map(|i| {
                let target_name = format!("{label}_target_{i}_{}", v.suffix);
                ProgrammedCallConfig {
                    to: target_name,
                    call_type: call_type.clone(),
                    gas_limit: None,
                    payments: call_payments.clone(),
                }
            })
            .collect();

        contracts.insert(
            root_name,
            ContractConfig {
                shard: Some(v.root_shard.into()),
                payable: None,
                address: None,
                calls,
                returns: Vec::new(),
            },
        );

        for i in 0..num_children {
            let target_name = format!("{label}_target_{i}_{}", v.suffix);
            contracts.insert(
                target_name,
                ContractConfig {
                    shard: Some(v.target_shard.into()),
                    payable: None,
                    address: None,
                    calls: Vec::new(),
                    returns: target_returns.clone(),
                },
            );
        }
    }

    CallTreeLayout { start, contracts }
}

// --- async_v1, 1 child ---
// async_v1 (legacy_async) only supports a single child.

/// Async-v1, 1 child, minimal shard layouts, no payments.
pub fn async_v1_c1() -> CallTreeLayout {
    async_impl(
        "async_v1_c1",
        ProgrammedCallTypeConfig::AsyncV1,
        1,
        three_shard_variants(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
}

/// Async-v1, 1 child, minimal shard layouts, with EGLD payment.
pub fn async_v1_c1_pay() -> CallTreeLayout {
    async_impl(
        "async_v1_c1",
        ProgrammedCallTypeConfig::AsyncV1,
        1,
        three_shard_variants(),
        vec![egld_start_payment()],
        vec![egld_payment()],
        vec![egld_return_payment()],
    )
}

/// Async-v1, 1 child, all 27 shard permutations, no payments.
pub fn async_v1_c1_all_shards() -> CallTreeLayout {
    async_impl(
        "async_v1_c1",
        ProgrammedCallTypeConfig::AsyncV1,
        1,
        all_shard_variants(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
}

/// Async-v1, 1 child, all 27 shard permutations, with EGLD payment.
pub fn async_v1_c1_pay_all_shards() -> CallTreeLayout {
    async_impl(
        "async_v1_c1",
        ProgrammedCallTypeConfig::AsyncV1,
        1,
        all_shard_variants(),
        vec![egld_start_payment()],
        vec![egld_payment()],
        vec![egld_return_payment()],
    )
}

// --- async_v2, 1 child ---

/// Async-v2, 1 child, minimal shard layouts, no payments.
pub fn async_v2_c1() -> CallTreeLayout {
    async_impl(
        "async_v2_c1",
        ProgrammedCallTypeConfig::AsyncV2,
        1,
        three_shard_variants(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
}

/// Async-v2, 1 child, minimal shard layouts, with EGLD payment.
pub fn async_v2_c1_pay() -> CallTreeLayout {
    async_impl(
        "async_v2_c1",
        ProgrammedCallTypeConfig::AsyncV2,
        1,
        three_shard_variants(),
        vec![egld_start_payment()],
        vec![egld_payment()],
        vec![egld_return_payment()],
    )
}

/// Async-v2, 1 child, all 27 shard permutations, no payments.
pub fn async_v2_c1_all_shards() -> CallTreeLayout {
    async_impl(
        "async_v2_c1",
        ProgrammedCallTypeConfig::AsyncV2,
        1,
        all_shard_variants(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
}

/// Async-v2, 1 child, all 27 shard permutations, with EGLD payment.
pub fn async_v2_c1_pay_all_shards() -> CallTreeLayout {
    async_impl(
        "async_v2_c1",
        ProgrammedCallTypeConfig::AsyncV2,
        1,
        all_shard_variants(),
        vec![egld_start_payment()],
        vec![egld_payment()],
        vec![egld_return_payment()],
    )
}

// --- async_v2, 3 children ---
// Each root contract calls one child per shard (shards 0, 1, 2).

fn async_c3_impl(
    label: &str,
    call_type: ProgrammedCallTypeConfig,
    variants: Vec<C3ShardVariant>,
    start_payments: Vec<PaymentConfig>,
    call_payments: Vec<PaymentConfig>,
    target_returns: Vec<PaymentConfig>,
) -> CallTreeLayout {
    let mut start = Vec::new();
    let mut contracts = BTreeMap::new();

    for v in &variants {
        let root_name = format!("{label}_root_{}", v.suffix);

        start.push(StartCall::new(
            root_name.clone(),
            v.sender_shard,
            start_payments.clone(),
        ));

        let calls = (0..3usize)
            .map(|i| {
                let target_name = format!("{label}_target_s{i}_{}", v.suffix);
                ProgrammedCallConfig {
                    to: target_name,
                    call_type: call_type.clone(),
                    gas_limit: None,
                    payments: call_payments.clone(),
                }
            })
            .collect();

        contracts.insert(
            root_name,
            ContractConfig {
                shard: Some(v.root_shard.into()),
                payable: None,
                address: None,
                calls,
                returns: Vec::new(),
            },
        );

        for i in 0..3usize {
            let target_name = format!("{label}_target_s{i}_{}", v.suffix);
            contracts.insert(
                target_name,
                ContractConfig {
                    shard: Some(ShardId::from(i as u32)),
                    payable: None,
                    address: None,
                    calls: Vec::new(),
                    returns: target_returns.clone(),
                },
            );
        }
    }

    CallTreeLayout { start, contracts }
}

/// Async-v2, 3 children (one per shard), no payments.
pub fn async_v2_c3() -> CallTreeLayout {
    async_c3_impl(
        "async_v2_c3",
        ProgrammedCallTypeConfig::AsyncV2,
        c3_shard_variants(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
}

/// Async-v2, 3 children (one per shard), with EGLD payment.
pub fn async_v2_c3_pay() -> CallTreeLayout {
    async_c3_impl(
        "async_v2_c3",
        ProgrammedCallTypeConfig::AsyncV2,
        c3_shard_variants(),
        vec![egld_start_payment()],
        vec![egld_payment()],
        vec![egld_return_payment()],
    )
}

// --- transf_exec ---
// Transfer-execute does not forward payments, but supports multiple children.

fn transf_exec_impl(
    label: &str,
    num_children: usize,
    variants: Vec<ShardVariant>,
) -> CallTreeLayout {
    assert!(num_children >= 1, "num_children must be at least 1");

    let mut start = Vec::new();
    let mut contracts = BTreeMap::new();

    for v in &variants {
        let root_name = format!("{label}_root_{}", v.suffix);

        start.push(StartCall::new(
            root_name.clone(),
            v.sender_shard,
            Vec::new(),
        ));

        let calls = (0..num_children)
            .map(|i| {
                let target_name = format!("{label}_target_{i}_{}", v.suffix);
                ProgrammedCallConfig {
                    to: target_name,
                    call_type: ProgrammedCallTypeConfig::TransfExec,
                    gas_limit: None,
                    payments: Vec::new(),
                }
            })
            .collect();

        contracts.insert(
            root_name,
            ContractConfig {
                shard: Some(v.root_shard.into()),
                payable: Some(true),
                address: None,
                calls,
                returns: Vec::new(),
            },
        );

        for i in 0..num_children {
            let target_name = format!("{label}_target_{i}_{}", v.suffix);
            contracts.insert(
                target_name,
                ContractConfig {
                    shard: Some(v.target_shard.into()),
                    payable: None,
                    address: None,
                    calls: Vec::new(),
                    returns: Vec::new(),
                },
            );
        }
    }

    CallTreeLayout { start, contracts }
}

/// Transfer-execute, 1 child, minimal shard layouts.
pub fn transf_exec_c1() -> CallTreeLayout {
    transf_exec_impl("transf_exec_c1", 1, three_shard_variants())
}

/// Transfer-execute, 1 child, all 27 shard permutations.
pub fn transf_exec_c1_all_shards() -> CallTreeLayout {
    transf_exec_impl("transf_exec_c1", 1, all_shard_variants())
}

/// Transfer-execute, 3 children (one per shard).
pub fn transf_exec_c3() -> CallTreeLayout {
    transf_exec_c3_impl("transf_exec_c3", c3_shard_variants())
}

fn transf_exec_c3_impl(label: &str, variants: Vec<C3ShardVariant>) -> CallTreeLayout {
    let mut start = Vec::new();
    let mut contracts = BTreeMap::new();

    for v in &variants {
        let root_name = format!("{label}_root_{}", v.suffix);

        start.push(StartCall::new(
            root_name.clone(),
            v.sender_shard,
            Vec::new(),
        ));

        let calls = (0..3usize)
            .map(|i| {
                let target_name = format!("{label}_target_s{i}_{}", v.suffix);
                ProgrammedCallConfig {
                    to: target_name,
                    call_type: ProgrammedCallTypeConfig::TransfExec,
                    gas_limit: None,
                    payments: Vec::new(),
                }
            })
            .collect();

        contracts.insert(
            root_name,
            ContractConfig {
                shard: Some(v.root_shard.into()),
                payable: Some(true),
                address: None,
                calls,
                returns: Vec::new(),
            },
        );

        for i in 0..3usize {
            let target_name = format!("{label}_target_s{i}_{}", v.suffix);
            contracts.insert(
                target_name,
                ContractConfig {
                    shard: Some(ShardId::from(i as u32)),
                    payable: None,
                    address: None,
                    calls: Vec::new(),
                    returns: Vec::new(),
                },
            );
        }
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
                returns: Vec::new(),
            },
        );
    }

    CallTreeLayout { start, contracts }
}
