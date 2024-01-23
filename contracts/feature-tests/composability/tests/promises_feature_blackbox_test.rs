use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::{
    api::StaticApi,
    scenario_model::{Account, CheckAccount, CheckStateStep, ScCallStep, SetStateStep},
    ContractInfo, ScenarioWorld,
};

use promises_features::call_sync_bt::ProxyTrait;

const USER_ADDRESS_EXPR: &str = "address:user";
const PROMISES_FEATURE_ADDRESS_EXPR: &str = "sc:promises-feature";
const PROMISES_FEATURES_PATH_EXPR: &str = "file:promises-features/output/promises-feature.wasm";
const VAULT_ADDRESS_EXPR: &str = "sc:vault";
const VAULT_PATH_EXPR: &str = "file:../vault/output/vault.wasm";

const TOKEN_ID_EXPR: &str = "str:TOKEN-123456";
const TOKEN_ID: &[u8] = b"TOKEN-123456";

type PromisesFeaturesContract = ContractInfo<promises_features::Proxy<StaticApi>>;
type VaultContract = ContractInfo<vault::Proxy<StaticApi>>;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/composability");

    blockchain.register_contract(
        PROMISES_FEATURES_PATH_EXPR,
        promises_features::ContractBuilder,
    );
    blockchain.register_contract(VAULT_PATH_EXPR, vault::ContractBuilder);

    blockchain
}

struct PromisesFeaturesTestState {
    world: ScenarioWorld,
    promises_features_contract: PromisesFeaturesContract,
    vault_contract: VaultContract,
}

impl PromisesFeaturesTestState {
    fn new() -> Self {
        let mut world = world();

        let promises_feature_code = world.code_expression(PROMISES_FEATURES_PATH_EXPR);
        let vault_code = world.code_expression(VAULT_PATH_EXPR);

        world.set_state_step(
            SetStateStep::new()
                .put_account(USER_ADDRESS_EXPR, Account::new().nonce(1))
                .put_account(
                    PROMISES_FEATURE_ADDRESS_EXPR,
                    Account::new().nonce(1).code(promises_feature_code),
                )
                .put_account(
                    VAULT_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .code(vault_code)
                        .esdt_balance(TOKEN_ID_EXPR, "1000"),
                ),
        );

        let promises_features_contract =
            PromisesFeaturesContract::new(PROMISES_FEATURE_ADDRESS_EXPR);
        let vault_contract = VaultContract::new(VAULT_ADDRESS_EXPR);

        Self {
            world,
            promises_features_contract,
            vault_contract,
        }
    }
}

#[test]
fn test_back_transfers() {
    let mut state = PromisesFeaturesTestState::new();
    let token_amount = BigUint::from(1000u64);

    state.world.sc_call(
        ScCallStep::new().from(USER_ADDRESS_EXPR).call(
            state
                .promises_features_contract
                .forward_sync_retrieve_funds_bt(
                    state.vault_contract,
                    TOKEN_ID,
                    0u64,
                    &token_amount,
                ),
        ),
    );

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            state.promises_features_contract,
            CheckAccount::new().esdt_balance(TOKEN_ID_EXPR, token_amount),
        ));
}

#[test]
fn test_multi_call_back_transfers() {
    let mut state = PromisesFeaturesTestState::new();
    let token_amount = BigUint::from(1000u64);
    let half_token_amount = token_amount.clone() / 2u64;
    let vault_address = state.vault_contract.to_address();

    state.world.sc_call(
        ScCallStep::new().from(USER_ADDRESS_EXPR).call(
            state
                .promises_features_contract
                .forward_sync_retrieve_funds_bt_twice(
                    vault_address.clone(),
                    TOKEN_ID,
                    0u64,
                    &half_token_amount,
                ),
        ),
    );

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            state.promises_features_contract,
            CheckAccount::new().esdt_balance(TOKEN_ID_EXPR, token_amount),
        ));
}
