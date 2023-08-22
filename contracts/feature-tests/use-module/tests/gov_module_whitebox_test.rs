use multiversx_sc::types::Address;
use multiversx_sc_scenario::{scenario_model::AddressValue, ScenarioWorld};

const _GOV_TOKEN_ID_EXPR: &str = "str:GOV-123456";
const _GOV_TOKEN_ID: &[u8] = b"GOV-123456";
const _QUORUM: u64 = 1_500;
const _MIN_BALANCE_PROPOSAL: u64 = 500;
const _VOTING_DELAY_BLOCKS: u64 = 10;
const _VOTING_PERIOD_BLOCKS: u64 = 20;
const _LOCKING_PERIOD_BLOCKS: u64 = 30;

const _INITIAL_GOV_TOKEN_BALANCE: u64 = 1_000;
const _GAS_LIMIT: u64 = 1_000_000;

const _USE_MODULE_ADDRESS_EXPR: &str = "sc:use-module";
const USE_MODULE_PATH_EXPR: &str = "file:output/use-module.wasm";

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
    // setup
    world()
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
