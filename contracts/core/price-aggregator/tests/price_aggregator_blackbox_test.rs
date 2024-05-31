use multiversx_price_aggregator_sc::{
    price_aggregator_data::{OracleStatus, TimestampedPrice, TokenPair},
    ContractObj, PriceAggregator, MAX_ROUND_DURATION_SECONDS,
};

use multiversx_sc_scenario::imports::*;

mod price_aggregator_proxy;

const DECIMALS: u8 = 0;
const EGLD_TICKER: &[u8] = b"EGLD";
const NR_ORACLES: usize = 4;
const SLASH_AMOUNT: u64 = 10;
const SLASH_QUORUM: usize = 3;
const STAKE_AMOUNT: u64 = 20;
const SUBMISSION_COUNT: usize = 3;
const USD_TICKER: &[u8] = b"USDC";

const PRICE_AGGREGATOR_ADDRESS: TestSCAddress = TestSCAddress::new("price-aggregator");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const PRICE_AGGREGATOR_PATH: MxscPath =
    MxscPath::new("output/multiversx-price-aggregator-sc.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        PRICE_AGGREGATOR_PATH,
        multiversx_price_aggregator_sc::ContractBuilder,
    );

    blockchain
}

struct PriceAggregatorTestState {
    world: ScenarioWorld,
    oracles: Vec<AddressValue>,
    price_aggregator_whitebox: WhiteboxContract<ContractObj<DebugApi>>,
}

impl PriceAggregatorTestState {
    fn new() -> Self {
        let mut world = world();

        world.account(OWNER_ADDRESS).nonce(1);
        world.current_block().block_timestamp(100);

        world.new_address(OWNER_ADDRESS, 1, PRICE_AGGREGATOR_ADDRESS);

        let mut oracles = Vec::new();
        for i in 1..=NR_ORACLES {
            let address_name = format!("oracle{i}");
            let address = TestAddress::new(&address_name);
            let address_value = AddressValue::from(address);

            world.account(address).nonce(1).balance(STAKE_AMOUNT);
            oracles.push(address_value);
        }

        let price_aggregator_whitebox = WhiteboxContract::new(
            PRICE_AGGREGATOR_ADDRESS,
            multiversx_price_aggregator_sc::contract_obj,
        );

        Self {
            world,
            oracles,
            price_aggregator_whitebox,
        }
    }

    fn deploy(&mut self) -> &mut Self {
        let oracles = MultiValueVec::from(
            self.oracles
                .iter()
                .map(|oracle| oracle.to_address())
                .collect::<Vec<_>>(),
        );

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(price_aggregator_proxy::PriceAggregatorProxy)
            .init(
                EgldOrEsdtTokenIdentifier::egld(),
                STAKE_AMOUNT,
                SLASH_AMOUNT,
                SLASH_QUORUM,
                SUBMISSION_COUNT,
                oracles,
            )
            .code(PRICE_AGGREGATOR_PATH)
            .run();

        for address in self.oracles.iter() {
            self.world
                .tx()
                .from(address)
                .to(PRICE_AGGREGATOR_ADDRESS)
                .typed(price_aggregator_proxy::PriceAggregatorProxy)
                .stake()
                .egld(STAKE_AMOUNT)
                .run();
        }

        self
    }

    fn set_pair_decimals(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(PRICE_AGGREGATOR_ADDRESS)
            .typed(price_aggregator_proxy::PriceAggregatorProxy)
            .set_pair_decimals(EGLD_TICKER, USD_TICKER, DECIMALS)
            .run();
    }

    fn unpause_endpoint(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(PRICE_AGGREGATOR_ADDRESS)
            .typed(price_aggregator_proxy::PriceAggregatorProxy)
            .unpause_endpoint()
            .run();
    }

    fn submit(&mut self, from: &AddressValue, submission_timestamp: u64, price: u64) {
        self.world
            .tx()
            .from(from)
            .to(PRICE_AGGREGATOR_ADDRESS)
            .typed(price_aggregator_proxy::PriceAggregatorProxy)
            .submit(
                EGLD_TICKER,
                USD_TICKER,
                submission_timestamp,
                price,
                DECIMALS,
            )
            .run();
    }

    fn submit_and_expect_err(
        &mut self,
        from: &AddressValue,
        submission_timestamp: u64,
        price: u64,
        err_message: &str,
    ) {
        self.world
            .tx()
            .from(from)
            .to(PRICE_AGGREGATOR_ADDRESS)
            .typed(price_aggregator_proxy::PriceAggregatorProxy)
            .submit(
                EGLD_TICKER,
                USD_TICKER,
                submission_timestamp,
                price,
                DECIMALS,
            )
            .with_result(ExpectStatus(4))
            .with_result(ExpectMessage(err_message))
            .run();
    }

    fn vote_slash_member(&mut self, from: &AddressValue, member_to_slash: Address) {
        self.world
            .tx()
            .from(from)
            .to(PRICE_AGGREGATOR_ADDRESS)
            .typed(price_aggregator_proxy::PriceAggregatorProxy)
            .vote_slash_member(member_to_slash)
            .run();
    }
}

#[test]
fn test_price_aggregator_submit() {
    let mut state = PriceAggregatorTestState::new();
    state.deploy();

    // configure the number of decimals
    state.set_pair_decimals();

    // try submit while paused
    state.submit_and_expect_err(&state.oracles[0].clone(), 99, 100, "Contract is paused");

    // unpause
    state.unpause_endpoint();

    // submit first timestamp too old
    state.submit_and_expect_err(
        &state.oracles[0].clone(),
        10,
        100,
        "First submission too old",
    );

    // submit ok
    state.submit(&state.oracles[0].clone(), 95, 100);

    let current_timestamp = 100;
    state
        .world
        .whitebox_query(&state.price_aggregator_whitebox, |sc| {
            let token_pair = TokenPair {
                from: managed_buffer!(EGLD_TICKER),
                to: managed_buffer!(USD_TICKER),
            };
            assert_eq!(
                sc.first_submission_timestamp(&token_pair).get(),
                current_timestamp
            );
            assert_eq!(
                sc.last_submission_timestamp(&token_pair).get(),
                current_timestamp
            );

            let submissions = sc.submissions().get(&token_pair).unwrap();
            assert_eq!(submissions.len(), 1);
            assert_eq!(
                submissions
                    .get(&managed_address!(&state.oracles[0].to_address()))
                    .unwrap(),
                managed_biguint!(100)
            );

            assert_eq!(
                sc.oracle_status()
                    .get(&managed_address!(&state.oracles[0].to_address()))
                    .unwrap(),
                OracleStatus {
                    total_submissions: 1,
                    accepted_submissions: 1
                }
            );
        });

    // first oracle submit again - submission not accepted
    state.submit(&state.oracles[0].clone(), 95, 100);

    state
        .world
        .whitebox_query(&state.price_aggregator_whitebox, |sc| {
            assert_eq!(
                sc.oracle_status()
                    .get(&managed_address!(&state.oracles[0].to_address()))
                    .unwrap(),
                OracleStatus {
                    total_submissions: 2,
                    accepted_submissions: 1
                }
            );
        });
}

#[test]
fn test_price_aggregator_submit_round_ok() {
    let mut state = PriceAggregatorTestState::new();
    state.deploy();

    // configure the number of decimals
    state.set_pair_decimals();

    // unpause
    state.unpause_endpoint();

    // submit first
    state.submit(&state.oracles[0].clone(), 95, 10_000);

    let current_timestamp = 110;
    state
        .world
        .current_block()
        .block_timestamp(current_timestamp);

    // submit second
    state.submit(&state.oracles[1].clone(), 101, 11_000);

    // submit third
    state.submit(&state.oracles[2].clone(), 105, 12_000);

    state
        .world
        .whitebox_query(&state.price_aggregator_whitebox, |sc| {
            let result =
                sc.latest_price_feed(managed_buffer!(EGLD_TICKER), managed_buffer!(USD_TICKER));

            let (round_id, from, to, timestamp, price, decimals) = result.into_tuple();
            assert_eq!(round_id, 1);
            assert_eq!(from, managed_buffer!(EGLD_TICKER));
            assert_eq!(to, managed_buffer!(USD_TICKER));
            assert_eq!(timestamp, current_timestamp);
            assert_eq!(price, managed_biguint!(11_000));
            assert_eq!(decimals, DECIMALS);

            // submissions are deleted after round is created
            let token_pair = TokenPair { from, to };
            let submissions = sc.submissions().get(&token_pair).unwrap();
            assert_eq!(submissions.len(), 0);

            let rounds = sc.rounds().get(&token_pair).unwrap();
            assert_eq!(rounds.len(), 1);
            assert_eq!(
                rounds.get(1),
                TimestampedPrice {
                    timestamp,
                    price,
                    decimals
                }
            );
        });
}

#[test]
fn test_price_aggregator_discarded_round() {
    let mut state = PriceAggregatorTestState::new();
    state.deploy();

    // configure the number of decimals
    state.set_pair_decimals();

    // unpause
    state.unpause_endpoint();

    // submit first
    state.submit(&state.oracles[0].clone(), 95, 10_000);

    let current_timestamp = 100 + MAX_ROUND_DURATION_SECONDS + 1;
    state
        .world
        .current_block()
        .block_timestamp(current_timestamp);

    // submit second - this will discard the previous submission
    state.submit(&state.oracles[1].clone(), current_timestamp - 1, 11_000);

    state
        .world
        .whitebox_query(&state.price_aggregator_whitebox, |sc| {
            let token_pair = TokenPair {
                from: managed_buffer!(EGLD_TICKER),
                to: managed_buffer!(USD_TICKER),
            };
            let submissions = sc.submissions().get(&token_pair).unwrap();
            assert_eq!(submissions.len(), 1);
            assert_eq!(
                submissions
                    .get(&managed_address!(&state.oracles[1].to_address()))
                    .unwrap(),
                managed_biguint!(11_000)
            );
        });
}

#[test]
fn test_price_aggregator_slashing() {
    let mut state = PriceAggregatorTestState::new();
    state.deploy();

    // unpause
    state.unpause_endpoint();

    state.vote_slash_member(&state.oracles[0].clone(), state.oracles[1].to_address());
    state.vote_slash_member(&state.oracles[2].clone(), state.oracles[1].to_address());
    state.vote_slash_member(&state.oracles[3].clone(), state.oracles[1].to_address());

    state
        .world
        .tx()
        .from(&state.oracles[0])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .typed(price_aggregator_proxy::PriceAggregatorProxy)
        .slash_member(state.oracles[1].to_address())
        .run();

    // oracle 1 try submit after slashing
    state.submit_and_expect_err(
        &state.oracles[1].clone(),
        95,
        10_000,
        "only oracles allowed",
    );
}
