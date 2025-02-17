use multiversx_sc_scenario::{imports::*, scenario_model::TxResponseStatus};
use num_bigint::BigUint;

use lottery_esdt::*;

pub const LOTTERY_PATH_EXPR: &str = "mxsc:output/lottery-esdt.mxsc.json";

pub const MY_ADDRESS: &str = "my_address";
pub const OTHER_SHARD_ADDRESS: &str = "other_shard_address#00";
pub const ACCOUNT1_ADDRESS: &str = "acc1";
pub const ACCOUNT2_ADDRESS: &str = "acc2";
pub const SC_LOTTERY_ADDRESS: &str = "lottery#01";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(LOTTERY_PATH_EXPR, lottery_esdt::ContractBuilder);
    blockchain
}

pub fn address(string: &str) -> String {
    let mut address = "address:".to_string();
    address.push_str(string);

    address
}

pub fn sc_address(string: &str) -> String {
    let mut sc_address = "sc:".to_string();
    sc_address.push_str(string);

    sc_address
}

pub struct LotteryScTestState {
    world: ScenarioWorld,
}

impl LotteryScTestState {
    pub fn new() -> Self {
        let mut world = world();

        world.set_state_step(
            SetStateStep::new()
                .put_account(
                    address(MY_ADDRESS),
                    Account::new().nonce(1).balance(1_000_000u64),
                )
                .new_address(
                    address(MY_ADDRESS).as_str(),
                    1,
                    sc_address(SC_LOTTERY_ADDRESS).as_str(),
                )
                .put_account(
                    address(OTHER_SHARD_ADDRESS),
                    Account::new().nonce(0).balance(1_000_000u64),
                )
                .put_account(
                    address(ACCOUNT1_ADDRESS),
                    Account::new().nonce(0).balance(1_000_000u64),
                )
                .put_account(
                    address(ACCOUNT2_ADDRESS),
                    Account::new().nonce(0).balance(1_000_000u64),
                ),
        );
        Self { world }
    }

    pub fn check_state(&mut self, check_state_step: CheckStateStep) -> &mut Self {
        self.world.check_state_step(check_state_step);
        self
    }

    fn handle_error(tx_error: TxResponseStatus, expected_error: Option<String>) {
        if let Some(error) = expected_error {
            assert!(tx_error == TxResponseStatus::signal_error(&error));
        } else {
            assert!(tx_error.is_success());
        }
    }

    pub fn deploy(&mut self) -> &mut Self {
        let lottery_contract =
            ContractInfo::<lottery_esdt::Proxy<StaticApi>>::new(sc_address(SC_LOTTERY_ADDRESS));

        self.world.chain_deploy(|tx| {
            tx.from(TestAddress::new(MY_ADDRESS))
                .typed(lottery_proxy::LotteryProxy)
                .init()
                .code(MxscPath::new("output/lottery-esdt.mxsc.json"))
                .with_result(WithNewAddress::new(|new_address| {
                    assert_eq!(new_address.to_address(), lottery_contract.to_address());
                }))
        });

        self
    }

    pub fn create_lottery_poll(
        &mut self,
        expected_error: Option<String>,
        lottery_name: String,
        token_identifier: TestTokenIdentifier,
        ticket_price: BigUint,
        opt_total_tickets: Option<usize>,
        opt_deadline: Option<u64>,
        opt_max_entries_per_user: Option<usize>,
        opt_prize_distribution: Option<Vec<u8>>,
        opt_whitelist: Option<Vec<TestAddress>>,
        opt_burn_percentage: OptionalValue<BigUint>,
    ) -> &mut Self {
        let expected_response = WithRawTxResponse(|response| {
            Self::handle_error(response.tx_error.clone(), expected_error);
        });

        self.world.chain_call(|tx| {
            tx.from(TestAddress::new(MY_ADDRESS))
                .to(TestSCAddress::new(SC_LOTTERY_ADDRESS))
                .typed(lottery_proxy::LotteryProxy)
                .create_lottery_pool(
                    lottery_name,
                    token_identifier,
                    ticket_price,
                    opt_total_tickets,
                    opt_deadline,
                    opt_max_entries_per_user,
                    opt_prize_distribution,
                    opt_whitelist,
                    opt_burn_percentage,
                )
                .with_result(expected_response)
        });

        self
    }

    pub fn buy_ticket(
        &mut self,
        expected_error: Option<String>,
        lottery_name: String,
        payment: TestEsdtTransfer,
    ) -> &mut Self {
        let expected_response = WithRawTxResponse(|response| {
            Self::handle_error(response.tx_error.clone(), expected_error);
        });

        self.world.chain_call(|tx| {
            tx.from(TestAddress::new(MY_ADDRESS))
                .to(TestSCAddress::new(SC_LOTTERY_ADDRESS))
                .typed(lottery_proxy::LotteryProxy)
                .buy_ticket(lottery_name)
                .payment(payment)
                .with_result(expected_response)
        });

        self
    }

    // since some of the endpoints have OptionalValue parameters I chose to put the error message the first parameter as convention

    pub fn determine_winner(
        &mut self,
        expected_error: Option<String>,
        lottery_name: String,
    ) -> &mut Self {
        let expected_response = WithRawTxResponse(|response| {
            Self::handle_error(response.tx_error.clone(), expected_error);
        });

        self.world.chain_call(|tx| {
            tx.from(TestAddress::new(MY_ADDRESS))
                .to(TestSCAddress::new(SC_LOTTERY_ADDRESS))
                .typed(lottery_proxy::LotteryProxy)
                .determine_winner(lottery_name)
                .with_result(expected_response)
        });

        self
    }

    pub fn claim_rewards(
        &mut self,
        expected_error: Option<String>,
        tokens: MultiValueVec<TestTokenIdentifier>,
    ) -> &mut Self {
        let expected_response = WithRawTxResponse(|response| {
            Self::handle_error(response.tx_error.clone(), expected_error);
        });

        self.world.chain_call(|tx| {
            tx.from(TestAddress::new(MY_ADDRESS))
                .to(TestSCAddress::new(SC_LOTTERY_ADDRESS))
                .typed(lottery_proxy::LotteryProxy)
                .claim_rewards(tokens)
                .with_result(expected_response)
        });

        self
    }

    pub fn status(&mut self, expected_error: Option<String>, lottery_name: String) -> &mut Self {
        let expected_response = WithRawTxResponse(|response| {
            Self::handle_error(response.tx_error.clone(), expected_error);
        });

        self.world.chain_query(|tx| {
            tx.to(TestSCAddress::new(SC_LOTTERY_ADDRESS))
                .typed(lottery_proxy::LotteryProxy)
                .status(lottery_name)
                .with_result(expected_response)
        });

        self
    }
}
