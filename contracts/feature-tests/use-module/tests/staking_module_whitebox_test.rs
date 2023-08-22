use multiversx_sc::types::{Address, EgldOrEsdtTokenIdentifier, ManagedVec};
use multiversx_sc_modules::staking::StakingModule;
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint,
    scenario_model::{Account, AddressValue, ScDeployStep, SetStateStep},
    ScenarioWorld, WhiteboxContract,
};

const STAKING_TOKEN_ID_EXPR: &str = "str:STAKE-123456";
const STAKING_TOKEN_ID: &[u8] = b"STAKE-123456";
const INITIAL_BALANCE: u64 = 2_000_000;
const REQUIRED_STAKE_AMOUNT: u64 = 1_000_000;
const SLASH_AMOUNT: u64 = 600_000;
const QUORUM: usize = 2;

const OWNER_ADDRESS_EXPR: &str = "address:owner";
const ALICE_ADDRESS_EXPR: &str = "address:alice";
const BOB_ADDRESS_EXPR: &str = "address:bob";
const CAROL_ADDRESS_EXPR: &str = "address:carol";
const EVE_ADDRESS_EXPR: &str = "address:eve";

const USE_MODULE_ADDRESS_EXPR: &str = "sc:use-module";
const USE_MODULE_PATH_EXPR: &str = "file:output/use-module.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/features-tests/use-module");

    blockchain.register_contract(USE_MODULE_PATH_EXPR, use_module::ContractBuilder);
    blockchain
}

#[test]
fn test_staking_module() {
    let mut world = world();

    world.set_state_step(
        SetStateStep::new()
            .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
            .new_address(OWNER_ADDRESS_EXPR, 1, USE_MODULE_ADDRESS_EXPR)
            .put_account(
                ALICE_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(STAKING_TOKEN_ID_EXPR, rust_biguint!(INITIAL_BALANCE)),
            )
            .put_account(
                BOB_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(STAKING_TOKEN_ID_EXPR, rust_biguint!(INITIAL_BALANCE)),
            )
            .put_account(
                CAROL_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(STAKING_TOKEN_ID_EXPR, rust_biguint!(INITIAL_BALANCE)),
            )
            .put_account(
                EVE_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(STAKING_TOKEN_ID_EXPR, rust_biguint!(INITIAL_BALANCE)),
            ),
    );

    // init
    let use_module_whitebox =
        WhiteboxContract::new(USE_MODULE_ADDRESS_EXPR, use_module::contract_obj);
    let use_module_code = world.code_expression(USE_MODULE_PATH_EXPR);

    world.whitebox_deploy(
        &use_module_whitebox,
        ScDeployStep::new()
            .from(OWNER_ADDRESS_EXPR)
            .code(use_module_code),
        |sc| {
            let mut whitelist = ManagedVec::new();
            whitelist.push(managed_address!(&address_expr_to_address(
                ALICE_ADDRESS_EXPR
            )));
            whitelist.push(managed_address!(&address_expr_to_address(BOB_ADDRESS_EXPR)));
            whitelist.push(managed_address!(&address_expr_to_address(
                CAROL_ADDRESS_EXPR
            )));

            sc.init_staking_module(
                &EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(STAKING_TOKEN_ID)),
                &managed_biguint!(REQUIRED_STAKE_AMOUNT),
                &managed_biguint!(SLASH_AMOUNT),
                QUORUM,
                &whitelist,
            );
        },
    );
}

fn address_expr_to_address(address_expr: &str) -> Address {
    AddressValue::from(address_expr).to_address()
}
