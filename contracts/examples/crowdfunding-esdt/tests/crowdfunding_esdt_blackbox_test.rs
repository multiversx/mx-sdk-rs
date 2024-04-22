use crowdfunding_esdt::crowdfunding_esdt_proxy;

use multiversx_sc_scenario::imports::*;
use num_bigint::BigUint;

const CF_DEADLINE: u64 = 7 * 24 * 60 * 60; // 1 week in seconds
const CF_TOKEN_ID: &[u8] = b"CROWD-123456";
const CF_TOKEN_ID_EXPR: &str = "str:CROWD-123456";
const FIRST_USER_ADDRESS: AddressExpr = AddressExpr("first-user");
const OWNER_ADDRESS: AddressExpr = AddressExpr("owner");
const SECOND_USER_ADDRESS: AddressExpr = AddressExpr("second-user");
const CODE_EXPR: MxscExpr = MxscExpr("output/crowdfunding-esdt.mxsc.json");
const SC_CROWDFUNDING_ESDT_EXPR: ScExpr = ScExpr("crowdfunding-esdt");

fn world() -> ScenarioWorld {
    let contract_path: &str = "mxsc:output/crowdfunding-esdt.mxsc.json";

    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-esdt");

    blockchain.register_contract(contract_path, crowdfunding_esdt::ContractBuilder);
    blockchain
}

struct CrowdfundingESDTTestState {
    world: ScenarioWorld,
}

impl CrowdfundingESDTTestState {
    fn new() -> Self {
        let mut world = world();
        let owner_address: &str = "address:owner";

        world
            .account(owner_address)
            .nonce(1)
            .account(FIRST_USER_ADDRESS)
            .nonce(1)
            .balance("1000")
            .esdt_balance(CF_TOKEN_ID_EXPR, "1000")
            .account(SECOND_USER_ADDRESS)
            .nonce(1)
            .esdt_balance(CF_TOKEN_ID_EXPR, "1000");

        world.new_address(
            owner_address,
            1,
            SC_CROWDFUNDING_ESDT_EXPR.eval_to_expr().as_str(),
        );

        Self { world }
    }

    fn deploy(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(crowdfunding_esdt_proxy::CrowdfundingProxy)
            .init(
                2_000u32,
                CF_DEADLINE,
                EgldOrEsdtTokenIdentifier::esdt(CF_TOKEN_ID),
            )
            .code(CODE_EXPR)
            .run();
    }

    fn fund(&mut self, address: AddressExpr, amount: u64) {
        self.world
            .tx()
            .from(address)
            .to(SC_CROWDFUNDING_ESDT_EXPR)
            .typed(crowdfunding_esdt_proxy::CrowdfundingProxy)
            .fund()
            .egld_or_single_esdt(
                &EgldOrEsdtTokenIdentifier::esdt(CF_TOKEN_ID),
                0u64,
                &multiversx_sc::proxy_imports::BigUint::from(amount),
            )
            .run();
    }

    fn check_deposit(&mut self, donor: AddressExpr, amount: u64) {
        let value = self
            .world
            .query()
            .to(SC_CROWDFUNDING_ESDT_EXPR)
            .typed(crowdfunding_esdt_proxy::CrowdfundingProxy)
            .deposit(donor.eval_to_array())
            .returns(ReturnsResultConv::<BigUint>::new())
            .run();

        assert_eq!(value, amount.into());
    }

    fn check_status(&mut self, expected_value: crowdfunding_esdt_proxy::Status) {
        let status = self
            .world
            .query()
            .to(SC_CROWDFUNDING_ESDT_EXPR)
            .typed(crowdfunding_esdt_proxy::CrowdfundingProxy)
            .status()
            .returns(ReturnsResult)
            .run();

        assert_eq!(status, expected_value);
    }

    fn claim(&mut self, address: AddressExpr) {
        self.world
            .tx()
            .from(address)
            .to(SC_CROWDFUNDING_ESDT_EXPR)
            .typed(crowdfunding_esdt_proxy::CrowdfundingProxy)
            .claim()
            .run();
    }

    fn check_esdt_balance(&mut self, address: AddressExpr, balance_expr: &str) {
        self.world
            .check_account(address)
            .esdt_balance(CF_TOKEN_ID_EXPR, balance_expr);
    }

    fn set_block_timestamp(&mut self, block_timestamp_expr: u64) {
        self.world
            .set_state_step(SetStateStep::new().block_timestamp(block_timestamp_expr));
    }
}

#[test]
fn test_fund() {
    let mut state = CrowdfundingESDTTestState::new();
    state.deploy();

    state.fund(FIRST_USER_ADDRESS, 1_000u64);
    state.check_deposit(FIRST_USER_ADDRESS, 1_000u64);
}

#[test]
fn test_status() {
    let mut state = CrowdfundingESDTTestState::new();
    state.deploy();

    state.check_status(crowdfunding_esdt_proxy::Status::FundingPeriod);
}

#[test]
fn test_sc_error() {
    let mut state = CrowdfundingESDTTestState::new();
    state.deploy();

    state
        .world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(SC_CROWDFUNDING_ESDT_EXPR)
        .typed(crowdfunding_esdt_proxy::CrowdfundingProxy)
        .fund()
        .egld(1000)
        .with_result(ExpectError(4, "wrong token"))
        .run();

    state.check_deposit(FIRST_USER_ADDRESS, 0);
}

#[test]
fn test_successful_cf() {
    let mut state = CrowdfundingESDTTestState::new();
    state.deploy();

    // first user fund
    state.fund(FIRST_USER_ADDRESS, 1_000u64);
    state.check_deposit(FIRST_USER_ADDRESS, 1_000);

    // second user fund
    state.fund(SECOND_USER_ADDRESS, 1000);
    state.check_deposit(SECOND_USER_ADDRESS, 1_000);

    // set block timestamp after deadline
    state.set_block_timestamp(CF_DEADLINE + 1);

    // check status successful
    state.check_status(crowdfunding_esdt_proxy::Status::Successful);

    state
        .world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(SC_CROWDFUNDING_ESDT_EXPR)
        .typed(crowdfunding_esdt_proxy::CrowdfundingProxy)
        .claim()
        .with_result(ExpectError(4, "only owner can claim successful funding"))
        .run();

    // owner claim
    state.claim(OWNER_ADDRESS);

    state.check_esdt_balance(OWNER_ADDRESS, "2_000");
    state.check_esdt_balance(FIRST_USER_ADDRESS, "0");
    state.check_esdt_balance(SECOND_USER_ADDRESS, "0");
}

#[test]
fn test_failed_cf() {
    let mut state = CrowdfundingESDTTestState::new();
    state.deploy();

    // first user fund
    state.fund(
        FIRST_USER_ADDRESS, //.eval_to_expr().as_str()
        300,
    );
    state.check_deposit(FIRST_USER_ADDRESS, 300u64);

    // second user fund
    state.fund(
        SECOND_USER_ADDRESS, //.eval_to_expr().as_str()
        600,
    );
    state.check_deposit(SECOND_USER_ADDRESS, 600u64);

    // set block timestamp after deadline
    state.set_block_timestamp(CF_DEADLINE + 1);

    // check status failed
    state.check_status(crowdfunding_esdt_proxy::Status::Failed);

    // first user claim
    state.claim(FIRST_USER_ADDRESS);

    // second user claim
    state.claim(SECOND_USER_ADDRESS);

    state.check_esdt_balance(OWNER_ADDRESS, "0");
    state.check_esdt_balance(FIRST_USER_ADDRESS, "1_000");
    state.check_esdt_balance(SECOND_USER_ADDRESS, "1_000");
}
