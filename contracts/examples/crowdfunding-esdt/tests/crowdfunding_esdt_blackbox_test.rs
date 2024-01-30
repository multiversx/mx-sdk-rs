use crowdfunding_esdt::{ProxyTrait as _, Status};
use multiversx_sc::{
    storage::mappers::SingleValue,
    types::{Address, EgldOrEsdtTokenIdentifier},
};
use multiversx_sc_scenario::{
    api::StaticApi,
    scenario_model::{
        Account, AddressValue, CheckAccount, CheckStateStep, ScCallStep, ScDeployStep, ScQueryStep,
        SetStateStep, TxExpect,
    },
    ContractInfo, ScenarioWorld,
};
use num_bigint::BigUint;

const CF_DEADLINE: u64 = 7 * 24 * 60 * 60; // 1 week in seconds
const CF_TOKEN_ID: &[u8] = b"CROWD-123456";
const CF_TOKEN_ID_EXPR: &str = "str:CROWD-123456";
const CROWDFUNDING_ESDT_ADDRESS_EXPR: &str = "sc:crowdfunding-esdt";
const CROWDFUNDING_ESDT_PATH_EXPR: &str = "mxsc:output/crowdfunding-esdt.mxsc.json";
const FIRST_USER_ADDRESS_EXPR: &str = "address:first-user";
const OWNER_ADDRESS_EXPR: &str = "address:owner";
const SECOND_USER_ADDRESS_EXPR: &str = "address:second-user";

type CrowdfundingESDTContract = ContractInfo<crowdfunding_esdt::Proxy<StaticApi>>;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-esdt");

    blockchain.register_contract(
        CROWDFUNDING_ESDT_PATH_EXPR,
        crowdfunding_esdt::ContractBuilder,
    );
    blockchain
}

struct CrowdfundingESDTTestState {
    world: ScenarioWorld,
    crowdfunding_esdt_contract: CrowdfundingESDTContract,
    first_user_address: Address,
    second_user_address: Address,
}

impl CrowdfundingESDTTestState {
    fn new() -> Self {
        let mut world = world();

        world.set_state_step(
            SetStateStep::new()
                .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
                .new_address(OWNER_ADDRESS_EXPR, 1, CROWDFUNDING_ESDT_ADDRESS_EXPR)
                .put_account(
                    FIRST_USER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(CF_TOKEN_ID_EXPR, "1_000"),
                )
                .put_account(
                    SECOND_USER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .esdt_balance(CF_TOKEN_ID_EXPR, "1_000"),
                ),
        );

        let crowdfunding_esdt_contract =
            CrowdfundingESDTContract::new(CROWDFUNDING_ESDT_ADDRESS_EXPR);

        let first_user_address = AddressValue::from(FIRST_USER_ADDRESS_EXPR).to_address();
        let second_user_address = AddressValue::from(SECOND_USER_ADDRESS_EXPR).to_address();

        Self {
            world,
            crowdfunding_esdt_contract,
            first_user_address,
            second_user_address,
        }
    }

    fn deploy(&mut self) -> &mut Self {
        let crowdfunding_esdt_code = self.world.code_expression(CROWDFUNDING_ESDT_PATH_EXPR);

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(crowdfunding_esdt_code)
                .call(self.crowdfunding_esdt_contract.init(
                    2_000u32,
                    CF_DEADLINE,
                    EgldOrEsdtTokenIdentifier::esdt(CF_TOKEN_ID),
                )),
        );

        self
    }

    fn fund(&mut self, address: &str, amount: &str) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address)
                .esdt_transfer(CF_TOKEN_ID_EXPR, 0, amount)
                .call(self.crowdfunding_esdt_contract.fund()),
        );

        self
    }

    fn check_deposit(&mut self, donor: Address, amount: u64) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.crowdfunding_esdt_contract.deposit(&donor))
                .expect_value(SingleValue::from(BigUint::from(amount))),
        );

        self
    }

    fn check_status(&mut self, expected_value: Status) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.crowdfunding_esdt_contract.status())
                .expect_value(expected_value),
        );

        self
    }

    fn claim(&mut self, address: &str) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address)
                .call(self.crowdfunding_esdt_contract.claim()),
        );

        self
    }

    fn check_esdt_balance(&mut self, address_expr: &str, balance_expr: &str) -> &mut Self {
        self.world
            .check_state_step(CheckStateStep::new().put_account(
                address_expr,
                CheckAccount::new().esdt_balance(CF_TOKEN_ID_EXPR, balance_expr),
            ));

        self
    }

    fn set_block_timestamp(&mut self, block_timestamp_expr: u64) -> &mut Self {
        self.world
            .set_state_step(SetStateStep::new().block_timestamp(block_timestamp_expr));

        self
    }
}

#[test]
fn test_fund() {
    let mut state = CrowdfundingESDTTestState::new();
    state.deploy();

    state.fund(FIRST_USER_ADDRESS_EXPR, "1000");
    state.check_deposit(state.first_user_address.clone(), 1_000);
}

#[test]
fn test_status() {
    let mut state = CrowdfundingESDTTestState::new();
    state.deploy();

    state.check_status(Status::FundingPeriod);
}

#[test]
fn test_sc_error() {
    let mut state = CrowdfundingESDTTestState::new();
    state.deploy();

    state.world.sc_call(
        ScCallStep::new()
            .from(FIRST_USER_ADDRESS_EXPR)
            .egld_value("1_000")
            .call(state.crowdfunding_esdt_contract.fund())
            .expect(TxExpect::user_error("str:wrong token")),
    );
    state.world.sc_query(
        ScQueryStep::new()
            .call(
                state
                    .crowdfunding_esdt_contract
                    .deposit(&state.first_user_address),
            )
            .expect(TxExpect::ok().result("0x")),
    );
}

#[test]
fn test_successful_cf() {
    let mut state = CrowdfundingESDTTestState::new();
    state.deploy();

    // first user fund
    state.fund(FIRST_USER_ADDRESS_EXPR, "1_000");
    state.check_deposit(state.first_user_address.clone(), 1_000);

    // second user fund
    state.fund(SECOND_USER_ADDRESS_EXPR, "1_000");
    state.check_deposit(state.second_user_address.clone(), 1_000);

    // set block timestamp after deadline
    state.set_block_timestamp(CF_DEADLINE + 1);

    // check status successful
    state.check_status(Status::Successful);

    // user try claim
    state.world.sc_call(
        ScCallStep::new()
            .from(FIRST_USER_ADDRESS_EXPR)
            .call(state.crowdfunding_esdt_contract.claim())
            .expect(TxExpect::user_error(
                "str:only owner can claim successful funding",
            )),
    );

    // owner claim
    state.claim(OWNER_ADDRESS_EXPR);

    state.check_esdt_balance(OWNER_ADDRESS_EXPR, "2_000");
    state.check_esdt_balance(FIRST_USER_ADDRESS_EXPR, "0");
    state.check_esdt_balance(SECOND_USER_ADDRESS_EXPR, "0");
}

#[test]
fn test_failed_cf() {
    let mut state = CrowdfundingESDTTestState::new();
    state.deploy();

    // first user fund
    state.fund(FIRST_USER_ADDRESS_EXPR, "300");
    state.check_deposit(state.first_user_address.clone(), 300u64);

    // second user fund
    state.fund(SECOND_USER_ADDRESS_EXPR, "600");
    state.check_deposit(state.second_user_address.clone(), 600u64);

    // set block timestamp after deadline
    state.set_block_timestamp(CF_DEADLINE + 1);

    // check status failed
    state.check_status(Status::Failed);

    // first user claim
    state.claim(FIRST_USER_ADDRESS_EXPR);

    // second user claim
    state.claim(SECOND_USER_ADDRESS_EXPR);

    state.check_esdt_balance(OWNER_ADDRESS_EXPR, "0");
    state.check_esdt_balance(FIRST_USER_ADDRESS_EXPR, "1_000");
    state.check_esdt_balance(SECOND_USER_ADDRESS_EXPR, "1_000");
}
