use multiversx_sc::types::{Address, EgldOrEsdtTokenIdentifier, ManagedVec};
use multiversx_sc_modules::staking::StakingModule;
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint,
    scenario_model::{
        Account, AddressValue, CheckAccount, CheckStateStep, ScCallStep, ScDeployStep, SetStateStep,
    },
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

    // try stake - not a board member
    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new()
            .from(EVE_ADDRESS_EXPR)
            .esdt_transfer(STAKING_TOKEN_ID, 0, rust_biguint!(REQUIRED_STAKE_AMOUNT))
            .no_expect(),
        |sc| sc.stake(),
        |r| {
            r.assert_user_error("Only whitelisted members can stake");
        },
    );

    // stake half and try unstake
    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(ALICE_ADDRESS_EXPR).esdt_transfer(
            STAKING_TOKEN_ID,
            0,
            rust_biguint!(REQUIRED_STAKE_AMOUNT / 2),
        ),
        |sc| sc.stake(),
    );

    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new().from(ALICE_ADDRESS_EXPR).no_expect(),
        |sc| sc.unstake(managed_biguint!(REQUIRED_STAKE_AMOUNT / 4)),
        |r| {
            r.assert_user_error("Not enough stake");
        },
    );

    // bob and carol stake
    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(BOB_ADDRESS_EXPR).esdt_transfer(
            STAKING_TOKEN_ID,
            0,
            rust_biguint!(REQUIRED_STAKE_AMOUNT),
        ),
        |sc| sc.stake(),
    );

    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(CAROL_ADDRESS_EXPR).esdt_transfer(
            STAKING_TOKEN_ID,
            0,
            rust_biguint!(REQUIRED_STAKE_AMOUNT),
        ),
        |sc| sc.stake(),
    );

    // try vote slash, not enough stake
    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new().from(ALICE_ADDRESS_EXPR).no_expect(),
        |sc| sc.vote_slash_member(managed_address!(&address_expr_to_address(BOB_ADDRESS_EXPR))),
        |r| {
            r.assert_user_error("Not enough stake");
        },
    );

    // try vote slash, slashed address not a board member
    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new().from(ALICE_ADDRESS_EXPR).no_expect(),
        |sc| sc.vote_slash_member(managed_address!(&address_expr_to_address(EVE_ADDRESS_EXPR))),
        |r| {
            r.assert_user_error("Voted user is not a staked board member");
        },
    );

    // alice stake over max amount and withdraw surplus
    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(ALICE_ADDRESS_EXPR).esdt_transfer(
            STAKING_TOKEN_ID,
            0,
            rust_biguint!(REQUIRED_STAKE_AMOUNT),
        ),
        |sc| {
            sc.stake();
            let alice_staked_amount = sc
                .staked_amount(&managed_address!(&address_expr_to_address(
                    ALICE_ADDRESS_EXPR
                )))
                .get();
            assert_eq!(alice_staked_amount, managed_biguint!(1_500_000));
        },
    );

    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(ALICE_ADDRESS_EXPR),
        |sc| {
            sc.unstake(managed_biguint!(500_000));

            let alice_staked_amount = sc
                .staked_amount(&managed_address!(&address_expr_to_address(
                    ALICE_ADDRESS_EXPR
                )))
                .get();
            assert_eq!(alice_staked_amount, managed_biguint!(1_000_000));
        },
    );

    world.check_state_step(CheckStateStep::new().put_account(
        ALICE_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(STAKING_TOKEN_ID_EXPR, rust_biguint!(1_000_000)),
    ));
}

fn address_expr_to_address(address_expr: &str) -> Address {
    AddressValue::from(address_expr).to_address()
}
