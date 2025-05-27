use multiversx_sc_scenario::imports::*;

use builtin_func_features::*;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const USER_ADDRESS: TestAddress = TestAddress::new("user");

const BUILTIN_FEATURES_FUNC_ADDRESS: TestSCAddress = TestSCAddress::new("builtin-func-features");
const CODE_PATH: MxscPath = MxscPath::new("output/builtin-func-features.mxsc.json");

const FUNGIBLE_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("FUNG-123456");
const NON_FUNGIBLE_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("NONFUNG-123456");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace(
        "contracts/feature-tests/composability/builtin-func-features",
    );
    blockchain.register_contract(CODE_PATH, builtin_func_features::ContractBuilder);
    blockchain
}

#[test]
fn transfer_fungible_promise_no_callback_blackbox_test() {
    let mut world = world();

    world
        .account(USER_ADDRESS)
        .nonce(1)
        .balance(1000)
        .esdt_balance(FUNGIBLE_TOKEN_ID, 1000)
        .esdt_balance(NON_FUNGIBLE_TOKEN_ID, 1000);

    world.start_trace();

    world.account(OWNER_ADDRESS).nonce(1);

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(builtin_func_features_proxy::BuiltinFuncFeaturesProxy)
        .init(FUNGIBLE_TOKEN_ID, NON_FUNGIBLE_TOKEN_ID)
        .code(CODE_PATH)
        .new_address(BUILTIN_FEATURES_FUNC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, BUILTIN_FEATURES_FUNC_ADDRESS.to_address());

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(BUILTIN_FEATURES_FUNC_ADDRESS)
        .typed(builtin_func_features_proxy::BuiltinFuncFeaturesProxy)
        .transfer_fungible_promise_no_callback(USER_ADDRESS, 1000u64)
        .run();

    world
        .check_account(USER_ADDRESS)
        .esdt_balance(FUNGIBLE_TOKEN_ID, 1000u64);

    world
        .check_account(BUILTIN_FEATURES_FUNC_ADDRESS)
        .esdt_balance(FUNGIBLE_TOKEN_ID, 0u64);
}

#[test]
fn transfer_fungible_promise_with_callback_blackbox_test() {
    let mut world = world();

    world
        .account(USER_ADDRESS)
        .nonce(1)
        .balance(1000)
        .esdt_balance(FUNGIBLE_TOKEN_ID, 1000)
        .esdt_balance(NON_FUNGIBLE_TOKEN_ID, 1000);

    world.start_trace();

    world.account(OWNER_ADDRESS).nonce(1);

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(builtin_func_features_proxy::BuiltinFuncFeaturesProxy)
        .init(FUNGIBLE_TOKEN_ID, NON_FUNGIBLE_TOKEN_ID)
        .code(CODE_PATH)
        .new_address(BUILTIN_FEATURES_FUNC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, BUILTIN_FEATURES_FUNC_ADDRESS.to_address());

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(BUILTIN_FEATURES_FUNC_ADDRESS)
        .typed(builtin_func_features_proxy::BuiltinFuncFeaturesProxy)
        .transfer_fungible_promise_with_callback(USER_ADDRESS, 1000u64)
        .run();

    world
        .check_account(USER_ADDRESS)
        .esdt_balance(FUNGIBLE_TOKEN_ID, 1000u64);

    world
        .check_account(BUILTIN_FEATURES_FUNC_ADDRESS)
        .esdt_balance(FUNGIBLE_TOKEN_ID, 0u64);
}
