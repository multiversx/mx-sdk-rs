use multiversx_sc::types::{Address, ManagedVec, MultiValueEncoded};
use multiversx_sc_modules::governance::{
    governance_configurable::GovernanceConfigurablePropertiesModule, GovernanceModule,
};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_buffer, managed_token_id, rust_biguint,
    scenario_model::{Account, AddressValue, ScCallStep, ScDeployStep, SetStateStep},
    ScenarioWorld, WhiteboxContract,
};

const GOV_TOKEN_ID_EXPR: &str = "str:GOV-123456";
const GOV_TOKEN_ID: &[u8] = b"GOV-123456";
const QUORUM: u64 = 1_500;
const MIN_BALANCE_PROPOSAL: u64 = 500;
const VOTING_DELAY_BLOCKS: u64 = 10;
const VOTING_PERIOD_BLOCKS: u64 = 20;
const LOCKING_PERIOD_BLOCKS: u64 = 30;

const INITIAL_GOV_TOKEN_BALANCE: u64 = 1_000;
const GAS_LIMIT: u64 = 1_000_000;

const USE_MODULE_ADDRESS_EXPR: &str = "sc:use-module";
const USE_MODULE_PATH_EXPR: &str = "file:output/use-module.wasm";

const OWNER_ADDRESS_EXPR: &str = "address:owner";
const FIRST_USER_ADDRESS_EXPR: &str = "address:first-user";
const SECOND_USER_ADDRESS_EXPR: &str = "address:second-user";
const THIRD_USER_ADDRESS_EXPR: &str = "address:third-user";

pub struct Payment {
    pub token: Vec<u8>,
    pub nonce: u64,
    pub amount: u64,
}

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/features-tests/use-module");

    blockchain.register_contract(USE_MODULE_PATH_EXPR, use_module::ContractBuilder);
    blockchain
}

fn setup() -> ScenarioWorld {
    let mut world = world();

    world.set_state_step(
        SetStateStep::new()
            .put_account(
                OWNER_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(GOV_TOKEN_ID_EXPR, rust_biguint!(INITIAL_GOV_TOKEN_BALANCE)),
            )
            .new_address(OWNER_ADDRESS_EXPR, 1, USE_MODULE_ADDRESS_EXPR)
            .put_account(
                FIRST_USER_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(GOV_TOKEN_ID_EXPR, rust_biguint!(INITIAL_GOV_TOKEN_BALANCE)),
            )
            .put_account(
                SECOND_USER_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(GOV_TOKEN_ID_EXPR, rust_biguint!(INITIAL_GOV_TOKEN_BALANCE)),
            )
            .put_account(
                THIRD_USER_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(GOV_TOKEN_ID_EXPR, rust_biguint!(INITIAL_GOV_TOKEN_BALANCE)),
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
            sc.init_governance_module(
                managed_token_id!(GOV_TOKEN_ID),
                managed_biguint!(QUORUM),
                managed_biguint!(MIN_BALANCE_PROPOSAL),
                VOTING_DELAY_BLOCKS,
                VOTING_PERIOD_BLOCKS,
                LOCKING_PERIOD_BLOCKS,
            );
        },
    );

    world.set_state_step(SetStateStep::new().block_nonce(10));

    world
}

pub fn propose(
    world: &mut ScenarioWorld,
    proposer: &Address,
    gov_token_amount: u64,
    dest_address: &Address,
    endpoint_name: &[u8],
    args: Vec<Vec<u8>>,
) -> usize {
    let use_module_whitebox =
        WhiteboxContract::new(USE_MODULE_ADDRESS_EXPR, use_module::contract_obj);

    let mut proposal_id = 0;

    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(proposer).esdt_transfer(
            GOV_TOKEN_ID,
            0,
            &rust_biguint!(gov_token_amount),
        ),
        |sc| {
            let mut args_managed = ManagedVec::new();
            for arg in args {
                args_managed.push(managed_buffer!(&arg));
            }

            let mut actions = MultiValueEncoded::new();
            actions.push(
                (
                    GAS_LIMIT,
                    managed_address!(dest_address),
                    managed_buffer!(endpoint_name),
                    args_managed,
                )
                    .into(),
            );

            proposal_id = sc.propose(managed_buffer!(b"change quorum"), actions);
        },
    );

    proposal_id
}

#[test]
fn test_init() {
    setup();
}

#[test]
fn test_change_gov_config() {}

#[test]
fn test_down_veto_gov_config() {}

#[test]
fn test_abstain_vote_gov_config() {}

#[test]
fn test_gov_cancel_defeated_proposal() {}

fn _address_expr_to_address(address_expr: &str) -> Address {
    AddressValue::from(address_expr).to_address()
}
