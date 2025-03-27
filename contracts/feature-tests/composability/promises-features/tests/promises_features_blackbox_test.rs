use multiversx_sc::types::{TestAddress, TestSCAddress, TestTokenIdentifier};

use multiversx_sc_scenario::{imports::*, ScenarioTxRun, ScenarioWorld};

use promises_features::*;

const PROMISES_FEATURES_CODE_PATH: MxscPath = MxscPath::new("output/promises-features.mxsc.json");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const USER_ADDRESS: TestAddress = TestAddress::new("user");
const PROMISES_FEATURES_ADDRESS: TestSCAddress = TestSCAddress::new("multi-transfer");
const TEST_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("TEST-123456");
const GAS_LIMIT: u64 = 10_000_000;
const EXTRA_GAS_FOR_CALLBACK: u64 = 5_000_000;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        PROMISES_FEATURES_CODE_PATH,
        promises_features::ContractBuilder,
    );

    blockchain
}

struct PromisesFeaturesTestState {
    world: ScenarioWorld,
}

impl PromisesFeaturesTestState {
    fn new() -> Self {
        let mut world = world();

        world
            .account(OWNER_ADDRESS)
            .nonce(1)
            .esdt_balance(TEST_TOKEN_ID, 1000u64)
            .account(USER_ADDRESS)
            .nonce(1);

        Self { world }
    }

    fn promises_features_deploy(&mut self) -> &mut Self {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .init()
            .code(PROMISES_FEATURES_CODE_PATH)
            .new_address(PROMISES_FEATURES_ADDRESS)
            .run();
        self
    }
}

#[test]
fn basic_setup_test() {
    let mut state = PromisesFeaturesTestState::new();

    state.promises_features_deploy();
}

#[test]
fn promises_transfer_test() {
    let mut state = PromisesFeaturesTestState::new();
    let initial_amount = BigUint::from(1000u64);
    let transfer_amount = BigUint::from(100u64);

    state.promises_features_deploy();

    state
        .world
        .check_account(OWNER_ADDRESS)
        .esdt_balance(TEST_TOKEN_ID, initial_amount.clone());

    state
        .world
        .check_account(USER_ADDRESS)
        .esdt_balance(TEST_TOKEN_ID, 0u64);

    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(PROMISES_FEATURES_ADDRESS)
        .typed(promises_feature_proxy::PromisesFeaturesProxy)
        .promise_raw_single_token_to_user(USER_ADDRESS, GAS_LIMIT, EXTRA_GAS_FOR_CALLBACK)
        .single_esdt(&(TEST_TOKEN_ID.into()), 0, &transfer_amount)
        .returns(ReturnsResult)
        .run();

    state
        .world
        .check_account(OWNER_ADDRESS)
        .esdt_balance(TEST_TOKEN_ID, initial_amount - transfer_amount.clone());

    state
        .world
        .check_account(USER_ADDRESS)
        .esdt_balance(TEST_TOKEN_ID, transfer_amount.clone());
}
