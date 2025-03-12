use forwarder::forwarder_proxy;
use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::imports::*;

use promises_features::promises_feature_proxy;

const USER_ADDRESS: TestAddress = TestAddress::new("user");
const PROMISES_FEATURE_ADDRESS: TestSCAddress = TestSCAddress::new("promises-feature");
const PROMISES_FEATURES_PATH: MxscPath =
    MxscPath::new("promises-features/output/promises-features.mxsc.json");
const VAULT_ADDRESS: TestSCAddress = TestSCAddress::new("vault");
const VAULT_PATH: MxscPath = MxscPath::new("../vault/output/vault.mxsc.json");

const TOKEN_ID_EXPR: TestTokenIdentifier = TestTokenIdentifier::new("TOKEN-123456");
const TOKEN_ID: &[u8] = b"TOKEN-123456";

const OTHER_TOKEN_ID_EXPR: TestTokenIdentifier = TestTokenIdentifier::new("TOKEN-789123");

const FORWARDER_ADDRESS: TestSCAddress = TestSCAddress::new("forwarder");
const FORWARDER_PATH: MxscPath = MxscPath::new("../forwarder/output/forwarder.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/composability");
    blockchain.register_contract(PROMISES_FEATURES_PATH, promises_features::ContractBuilder);
    blockchain.register_contract(VAULT_PATH, vault::ContractBuilder);
    blockchain.register_contract(FORWARDER_PATH, forwarder::ContractBuilder);

    blockchain
}

struct PromisesFeaturesTestState {
    world: ScenarioWorld,
}

impl PromisesFeaturesTestState {
    fn new() -> Self {
        let mut world = world();

        world
            .account(USER_ADDRESS)
            .nonce(1)
            .balance(100)
            .esdt_balance(TOKEN_ID_EXPR, 1000)
            .esdt_balance(OTHER_TOKEN_ID_EXPR, 1000);
        world
            .account(PROMISES_FEATURE_ADDRESS)
            .nonce(1)
            .code(PROMISES_FEATURES_PATH);
        world
            .account(VAULT_ADDRESS)
            .nonce(1)
            .code(VAULT_PATH)
            .esdt_balance(TOKEN_ID_EXPR, 1000);
        world
            .account(FORWARDER_ADDRESS)
            .nonce(1)
            .code(FORWARDER_PATH);

        Self { world }
    }
}

#[test]
fn test_back_transfers() {
    let mut state = PromisesFeaturesTestState::new();
    let token_amount = BigUint::from(1000u64);

    state
        .world
        .tx()
        .from(USER_ADDRESS)
        .to(PROMISES_FEATURE_ADDRESS)
        .typed(promises_feature_proxy::PromisesFeaturesProxy)
        .forward_sync_retrieve_funds_bt(VAULT_ADDRESS, TOKEN_ID, 0u64, &token_amount)
        .run();

    state
        .world
        .check_account(PROMISES_FEATURE_ADDRESS)
        .esdt_balance(TOKEN_ID_EXPR, token_amount);
}

#[test]
fn test_back_transfers_reset() {
    let mut state = PromisesFeaturesTestState::new();
    let token_amount = BigUint::from(1000u64);
    let half_token_amount = token_amount.clone() / 2u64;

    state
        .world
        .tx()
        .from(USER_ADDRESS)
        .to(PROMISES_FEATURE_ADDRESS)
        .typed(promises_feature_proxy::PromisesFeaturesProxy)
        .forward_sync_retrieve_funds_bt_reset_twice(
            VAULT_ADDRESS,
            TOKEN_ID,
            0u64,
            &half_token_amount,
        )
        .run();

    state
        .world
        .check_account(PROMISES_FEATURE_ADDRESS)
        .esdt_balance(TOKEN_ID_EXPR, token_amount);
}

#[test]
fn test_multi_call_back_transfers() {
    let mut state = PromisesFeaturesTestState::new();
    let token_amount = BigUint::from(1000u64);
    let half_token_amount = token_amount.clone() / 2u64;

    state
        .world
        .tx()
        .from(USER_ADDRESS)
        .to(PROMISES_FEATURE_ADDRESS)
        .typed(promises_feature_proxy::PromisesFeaturesProxy)
        .forward_sync_retrieve_funds_bt_twice(VAULT_ADDRESS, TOKEN_ID, 0u64, &half_token_amount)
        .run();

    state
        .world
        .check_account(PROMISES_FEATURE_ADDRESS)
        .esdt_balance(TOKEN_ID_EXPR, token_amount);
}

#[test]
fn test_back_transfers_logs() {
    let mut state = PromisesFeaturesTestState::new();
    let token_amount = BigUint::from(1000u64);

    let logs = state
        .world
        .tx()
        .from(USER_ADDRESS)
        .to(PROMISES_FEATURE_ADDRESS)
        .typed(promises_feature_proxy::PromisesFeaturesProxy)
        .forward_sync_retrieve_funds_bt(VAULT_ADDRESS, TOKEN_ID, 0u64, &token_amount)
        .returns(ReturnsLogs)
        .run();

    assert!(!logs.is_empty() && !logs[0].topics.is_empty());
    assert_eq!(logs[0].address, PROMISES_FEATURE_ADDRESS);
    assert_eq!(logs[0].endpoint, "transferValueOnly");
}

#[test]
fn test_multi_call_back_transfers_logs() {
    let mut state = PromisesFeaturesTestState::new();
    let token_amount = BigUint::from(1000u64);
    let half_token_amount = token_amount.clone() / 2u64;

    let logs = state
        .world
        .tx()
        .from(USER_ADDRESS)
        .to(PROMISES_FEATURE_ADDRESS)
        .typed(promises_feature_proxy::PromisesFeaturesProxy)
        .forward_sync_retrieve_funds_bt_twice(VAULT_ADDRESS, TOKEN_ID, 0u64, &half_token_amount)
        .returns(ReturnsLogs)
        .run();

    assert!(!logs.is_empty() && !logs[0].topics.is_empty());
    assert_eq!(logs[0].address, PROMISES_FEATURE_ADDRESS);
    assert_eq!(logs[0].endpoint, "transferValueOnly");
}

#[test]
fn test_back_transfers_handlers() {
    let mut state = PromisesFeaturesTestState::new();
    let egld_amount = BigUint::from(2u64);
    let token_amount = BigUint::from(500u64);

    let result = state
        .world
        .tx()
        .from(USER_ADDRESS)
        .to(FORWARDER_ADDRESS)
        .typed(forwarder_proxy::ForwarderProxy)
        .forward_sync_accept_funds_rh_egld(VAULT_ADDRESS)
        .egld(&egld_amount)
        .returns(ReturnsResult)
        .run();

    assert_eq!(result, egld_amount);

    let result = state
        .world
        .tx()
        .from(USER_ADDRESS)
        .to(FORWARDER_ADDRESS)
        .typed(forwarder_proxy::ForwarderProxy)
        .forward_sync_accept_funds_rh_single_esdt(VAULT_ADDRESS)
        .single_esdt(&TOKEN_ID_EXPR.to_token_identifier(), 0u64, &token_amount)
        .returns(ReturnsResult)
        .run();

    assert!(
        result.token_identifier == TOKEN_ID_EXPR.to_token_identifier()
            && result.token_nonce == 0u64
            && result.amount == token_amount
    );

    let mut multi_transfer = ManagedVec::<StaticApi, EsdtTokenPayment<StaticApi>>::new();
    multi_transfer.push(EsdtTokenPayment::new(
        TOKEN_ID_EXPR.to_token_identifier(),
        0u64,
        token_amount.clone(),
    ));
    multi_transfer.push(EsdtTokenPayment::new(
        OTHER_TOKEN_ID_EXPR.to_token_identifier(),
        0u64,
        token_amount.clone(),
    ));

    let result = state
        .world
        .tx()
        .from(USER_ADDRESS)
        .to(FORWARDER_ADDRESS)
        .typed(forwarder_proxy::ForwarderProxy)
        .forward_sync_accept_funds_rh_multi_esdt(VAULT_ADDRESS)
        .payment(&multi_transfer)
        .returns(ReturnsResult)
        .run();

    assert_eq!(result.len(), 2usize);
    assert!(
        result.get(0).token_identifier == TOKEN_ID_EXPR.to_token_identifier()
            && result.get(0).amount == token_amount
    );
    assert!(
        result.get(1).token_identifier == OTHER_TOKEN_ID_EXPR.to_token_identifier()
            && result.get(1).amount == token_amount
    );
}
