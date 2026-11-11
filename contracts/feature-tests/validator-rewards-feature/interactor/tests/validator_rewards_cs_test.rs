use multiversx_sc_snippets::imports::*;
use serial_test::serial;
use validator_rewards_interactor::{Config, State, ValidatorRewardsInteract};

/// 1 EGLD.
const ONE_EGLD: u64 = 1_000_000_000_000_000_000;

async fn cs_interactor() -> ValidatorRewardsInteract {
    let config = Config {
        connection: ConnectionConfig::chain_simulator(),
        owner: WalletConfig::from_test_wallet("heidi"),
    };
    let interactor = Interactor::empty()
        .with_current_dir(env!("CARGO_MANIFEST_DIR"))
        .with_config(&config)
        .await;
    ValidatorRewardsInteract {
        interactor,
        config,
        state: AutoSave::<State>::no_save_default(),
    }
}

fn parse_balance(balance: &str) -> RustBigUint {
    balance.parse().expect("unexpected balance value")
}

/// Injects validator rewards into a deployed contract that already has storage, and checks
/// that the contract sees them through its blockchain API (`getCumulatedValidatorRewards`),
/// can act on them, and is otherwise left exactly as it was.
///
/// Rewards are read back through contract endpoints, not through the interactor's storage
/// helper, so the test does not depend on the storage key the injection itself writes.
#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn inject_validator_reward_into_contract_cs_test() {
    const NOTE: &[u8] = b"data that must survive the reward injection";

    let mut interact = cs_interactor().await;
    let owner = interact.config.owner.address();
    interact
        .interactor
        .send_user_funds(&owner)
        .await
        .expect("failed to fund the owner");

    interact.deploy().await;
    let contract = interact.contract_address();

    // Give the contract storage that has nothing to do with rewards.
    interact.store_note(NOTE).await;

    assert_eq!(interact.cumulated_rewards().await, RustBigUint::default());
    let metadata_before = interact.code_metadata().await;

    // Snapshots are taken right around the injection, with no transaction in between:
    // transactions accrue developer rewards to the owner, which would blur the comparison.
    let account_before = interact.interactor.get_account(contract.as_address()).await;
    let storage_before = interact
        .interactor
        .get_account_storage(contract.as_address())
        .await;
    assert!(!account_before.code.is_empty(), "contract has no code");
    assert!(!storage_before.is_empty(), "contract has no storage");

    let reward = RustBigUint::from(ONE_EGLD);
    interact
        .interactor
        .inject_validator_reward(&contract, &reward)
        .await
        .expect("injecting a validator reward failed");

    let account_after = interact.interactor.get_account(contract.as_address()).await;
    let storage_after = interact
        .interactor
        .get_account_storage(contract.as_address())
        .await;

    // Nothing but the balance and the reward counter changed on the account.
    assert_eq!(
        parse_balance(&account_after.balance),
        parse_balance(&account_before.balance) + &reward
    );
    assert_eq!(account_after.nonce, account_before.nonce);
    assert_eq!(account_after.code, account_before.code);
    assert_eq!(account_after.code_hash, account_before.code_hash);
    assert_eq!(account_after.code_metadata, account_before.code_metadata);
    assert_eq!(account_after.owner_address, account_before.owner_address);
    assert_eq!(
        account_after.developer_reward,
        account_before.developer_reward
    );
    for (key, value) in &storage_before {
        assert_eq!(
            storage_after.get(key),
            Some(value),
            "storage key {key} changed"
        );
    }
    assert_eq!(
        storage_after.len(),
        storage_before.len() + 1,
        "unexpected storage changes: {storage_after:?}"
    );

    // The contract sees the reward through its blockchain API, and is otherwise unchanged.
    assert_eq!(interact.cumulated_rewards().await, reward);
    assert_eq!(interact.unclaimed_rewards().await, reward);
    assert_eq!(interact.note().await, NOTE);
    assert_eq!(interact.code_metadata().await, metadata_before);

    // The contract can act on the reward: the owner claims it.
    let contract_balance_before_claim = parse_balance(&account_after.balance);
    assert_eq!(interact.claim_rewards().await, reward);
    assert_eq!(interact.claimed_rewards().await, reward);
    assert_eq!(interact.unclaimed_rewards().await, RustBigUint::default());
    assert_eq!(
        parse_balance(
            &interact
                .interactor
                .get_account(contract.as_address())
                .await
                .balance
        ),
        contract_balance_before_claim - &reward
    );
    // Claiming does not reduce the cumulated counter.
    assert_eq!(interact.cumulated_rewards().await, reward);

    // A later reward accumulates on top, and only that part is unclaimed.
    interact
        .interactor
        .inject_validator_reward(&contract, &reward)
        .await
        .expect("injecting a second validator reward failed");
    assert_eq!(interact.cumulated_rewards().await, reward.clone() * 2u32);
    assert_eq!(interact.unclaimed_rewards().await, reward);
    assert_eq!(interact.note().await, NOTE);
}
