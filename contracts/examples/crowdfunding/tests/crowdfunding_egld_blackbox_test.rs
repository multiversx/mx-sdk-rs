use crowdfunding::crowdfunding_proxy;

use multiversx_sc_scenario::imports::*;

const CF_DEADLINE: TimestampMillis = TimestampMillis::new(7 * 24 * 60 * 60 * 1000); // 1 week in milliseconds

const FIRST_USER_ADDRESS: TestAddress = TestAddress::new("first-user");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const SECOND_USER_ADDRESS: TestAddress = TestAddress::new("second-user");

const CODE_PATH: MxscPath = MxscPath::new("output/crowdfunding.mxsc.json");
const CROWDFUNDING_ADDRESS: TestSCAddress = TestSCAddress::new("crowdfunding-sc");

const OTHER_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("OTHER-123456");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding");
    blockchain.register_contract(CODE_PATH, crowdfunding::ContractBuilder);
    blockchain
}

struct CrowdfundingTestState {
    world: ScenarioWorld,
}

impl CrowdfundingTestState {
    fn new() -> Self {
        let mut world = world();

        world.account(OWNER_ADDRESS).nonce(1);

        world
            .account(FIRST_USER_ADDRESS)
            .nonce(1)
            .balance(1000)
            .esdt_balance(OTHER_TOKEN_ID, 1000);

        world.account(SECOND_USER_ADDRESS).nonce(1).balance(1000);

        Self { world }
    }

    fn deploy(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .init(TokenId::native(), 2_000u32, CF_DEADLINE)
            .code(CODE_PATH)
            .new_address(CROWDFUNDING_ADDRESS)
            .run();
    }

    fn fund(&mut self, address: TestAddress, amount: u64) {
        self.world
            .tx()
            .from(address)
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .fund()
            .egld(amount)
            .run();
    }

    fn check_deposit(&mut self, donor: TestAddress, amount: u64) {
        self.world
            .query()
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .deposit(donor)
            .returns(ExpectValue(amount))
            .run();
    }

    fn check_status(&mut self, expected_value: crowdfunding_proxy::Status) {
        self.world
            .query()
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .status()
            .returns(ExpectValue(expected_value))
            .run();
    }

    fn claim(&mut self, address: TestAddress) {
        self.world
            .tx()
            .from(address)
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .claim()
            .run();
    }

    fn check_balance(&mut self, address: TestAddress, balance: u64) {
        self.world.check_account(address).balance(balance);
    }

    fn set_block_timestamp(&mut self, block_timestamp: TimestampMillis) {
        self.world
            .current_block()
            .block_timestamp_millis(block_timestamp);
    }
}

#[test]
fn test_fund_egld() {
    let mut state = CrowdfundingTestState::new();
    state.deploy();

    state.fund(FIRST_USER_ADDRESS, 1_000u64);
    state.check_deposit(FIRST_USER_ADDRESS, 1_000u64);
}

#[test]
fn test_status_egld() {
    let mut state = CrowdfundingTestState::new();
    state.deploy();

    state.check_status(crowdfunding_proxy::Status::FundingPeriod);
}

#[test]
fn test_sc_error_egld() {
    let mut state = CrowdfundingTestState::new();

    state.deploy();

    state
        .world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .fund()
        .payment(Payment::new(
            OTHER_TOKEN_ID,
            0,
            NonZeroBigUint::try_from(1000u128).unwrap(),
        ))
        .with_result(ExpectError(4, "wrong token"))
        .run();

    state.check_deposit(FIRST_USER_ADDRESS, 0);
}

#[test]
fn test_successful_cf_egld() {
    let mut state = CrowdfundingTestState::new();
    state.deploy();

    // first user fund
    state.fund(FIRST_USER_ADDRESS, 1_000u64);
    state.check_deposit(FIRST_USER_ADDRESS, 1_000);

    // second user fund
    state.fund(SECOND_USER_ADDRESS, 1000);
    state.check_deposit(SECOND_USER_ADDRESS, 1_000);

    // set block timestamp after deadline
    state.set_block_timestamp(CF_DEADLINE + DurationMillis::new(1));

    // check status successful
    state.check_status(crowdfunding_proxy::Status::Successful);

    state
        .world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .claim()
        .with_result(ExpectError(4, "only owner can claim successful funding"))
        .run();

    // owner claim
    state.claim(OWNER_ADDRESS);

    state.check_balance(OWNER_ADDRESS, 2000);
    state.check_balance(FIRST_USER_ADDRESS, 0);
    state.check_balance(SECOND_USER_ADDRESS, 0);
}

#[test]
fn test_failed_cf_egld() {
    let mut state = CrowdfundingTestState::new();
    state.deploy();

    // first user fund
    state.fund(FIRST_USER_ADDRESS, 300);
    state.check_deposit(FIRST_USER_ADDRESS, 300u64);

    // second user fund
    state.fund(SECOND_USER_ADDRESS, 600);
    state.check_deposit(SECOND_USER_ADDRESS, 600u64);

    // set block timestamp after deadline
    state.set_block_timestamp(CF_DEADLINE + DurationMillis::new(1));

    // check status failed
    state.check_status(crowdfunding_proxy::Status::Failed);

    // first user claim
    state.claim(FIRST_USER_ADDRESS);

    // second user claim
    state.claim(SECOND_USER_ADDRESS);

    state.check_balance(OWNER_ADDRESS, 0);
    state.check_balance(FIRST_USER_ADDRESS, 1000);
    state.check_balance(SECOND_USER_ADDRESS, 1000);
}
