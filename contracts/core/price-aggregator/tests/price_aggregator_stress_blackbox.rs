use multiversx_price_aggregator_sc::{
    price_aggregator_data::{OracleStatus, TokenPair},
    ContractObj, PriceAggregator, ProxyTrait as _,
};
use multiversx_sc::{
    codec::multi_types::MultiValueVec, contract_base::ContractBase,
    types::EgldOrEsdtTokenIdentifier,
};
use multiversx_sc_modules::{pause::ProxyTrait, staking::ProxyTrait as _};
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_address, managed_buffer,
    scenario_model::{Account, AddressValue, ScCallStep, ScDeployStep, SetStateStep},
    ContractInfo, DebugApi, ScenarioWorld, WhiteboxContract,
};

const DECIMALS: u8 = 0;
const EGLD_TICKER: &[u8] = b"EGLD";
const NR_ORACLES: usize = 50;
const OWNER_ADDRESS_EXPR: &str = "address:owner";
const PRICE_AGGREGATOR_ADDRESS_EXPR: &str = "sc:price-aggregator";
const PRICE_AGGREGATOR_PATH_EXPR: &str = "file:../output/multiversx-price-aggregator-sc.wasm";
const SLASH_AMOUNT: u64 = 10;
const SLASH_QUORUM: usize = 3;
const STAKE_AMOUNT: u64 = 20;
const SUBMISSION_COUNT: usize = 50;
const USD_TICKER: &[u8] = b"USDC";

type PriceAggregatorContract = ContractInfo<multiversx_price_aggregator_sc::Proxy<StaticApi>>;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/core/price-aggregator");
    blockchain.register_contract(
        PRICE_AGGREGATOR_PATH_EXPR,
        multiversx_price_aggregator_sc::ContractBuilder,
    );

    blockchain
}

struct PriceAggregatorTestState {
    world: ScenarioWorld,
    oracles: Vec<AddressValue>,
    price_aggregator_contract: PriceAggregatorContract,
    price_aggregator_whitebox: WhiteboxContract<ContractObj<DebugApi>>,
}

impl PriceAggregatorTestState {
    fn new() -> Self {
        let mut world = world();

        let mut set_state_step = SetStateStep::new()
            .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
            .new_address(OWNER_ADDRESS_EXPR, 1, PRICE_AGGREGATOR_ADDRESS_EXPR)
            .block_timestamp(100);

        let mut oracles = Vec::new();
        for i in 1..=NR_ORACLES {
            let address_expr = format!("address:oracle{}", i);
            let address_value = AddressValue::from(address_expr.as_str());

            set_state_step = set_state_step.put_account(
                address_expr.as_str(),
                Account::new().nonce(1).balance(STAKE_AMOUNT),
            );

            oracles.push(address_value);
        }
        world.start_trace().set_state_step(set_state_step);

        let price_aggregator_contract = PriceAggregatorContract::new(PRICE_AGGREGATOR_ADDRESS_EXPR);
        let price_aggregator_whitebox = WhiteboxContract::new(
            PRICE_AGGREGATOR_ADDRESS_EXPR,
            multiversx_price_aggregator_sc::contract_obj,
        );

        Self {
            world,
            oracles,
            price_aggregator_contract,
            price_aggregator_whitebox,
        }
    }

    fn deploy(&mut self) -> &mut Self {
        let price_aggregator_code = self.world.code_expression(PRICE_AGGREGATOR_PATH_EXPR);

        let oracles = MultiValueVec::from(
            self.oracles
                .iter()
                .map(|oracle| oracle.to_address())
                .collect::<Vec<_>>(),
        );

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(price_aggregator_code)
                .call(self.price_aggregator_contract.init(
                    EgldOrEsdtTokenIdentifier::egld(),
                    STAKE_AMOUNT,
                    SLASH_AMOUNT,
                    SLASH_QUORUM,
                    SUBMISSION_COUNT,
                    oracles,
                ))
                .gas_limit("120,000,000"),
        );

        for address in self.oracles.iter() {
            self.world.sc_call(
                ScCallStep::new()
                    .from(address)
                    .egld_value(STAKE_AMOUNT)
                    .call(self.price_aggregator_contract.stake())
                    .gas_limit("5,000,000"),
            );
        }

        self
    }

    fn set_pair_decimals(&mut self) {
        self.world.sc_call(
            ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.price_aggregator_contract
                    .set_pair_decimals(EGLD_TICKER, USD_TICKER, DECIMALS),
            ),
        );
    }

    fn unpause_endpoint(&mut self) {
        self.world.sc_call(
            ScCallStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .call(self.price_aggregator_contract.unpause_endpoint())
                .gas_limit("5,000,000"),
        );
    }

    fn submit(&mut self, from: &AddressValue, submission_timestamp: u64, price: u64) {
        self.world.sc_call(
            ScCallStep::new()
                .from(from)
                .call(self.price_aggregator_contract.submit(
                    EGLD_TICKER,
                    USD_TICKER,
                    submission_timestamp,
                    price,
                    DECIMALS,
                ))
                .gas_limit("7,000,000"),
        );
    }
}

#[test]
fn test_price_aggregator_submit() {
    let mut state = PriceAggregatorTestState::new();
    state.deploy();

    // configure the number of decimals
    state.set_pair_decimals();

    // unpause
    state.unpause_endpoint();

    // submit first
    state.submit(&state.oracles[0].clone(), 95, rand::random::<u64>());

    // submit ok
    for index in 1..SUBMISSION_COUNT - 1 {
        state.submit(&state.oracles[index].clone(), 100, rand::random::<u64>());
    }

    let current_timestamp = 100;
    state
        .world
        .whitebox_query(&state.price_aggregator_whitebox, |sc| {
            let blockchain_timestamp = sc.blockchain().get_block_timestamp();

            let token_pair = TokenPair {
                from: managed_buffer!(EGLD_TICKER),
                to: managed_buffer!(USD_TICKER),
            };
            assert_eq!(blockchain_timestamp, current_timestamp);

            let submission_count = sc.submission_count().get();

            assert_eq!(submission_count, SUBMISSION_COUNT);
            assert_eq!(
                sc.first_submission_timestamp(&token_pair).get(),
                current_timestamp
            );
            assert_eq!(
                sc.last_submission_timestamp(&token_pair).get(),
                current_timestamp
            );

            let submissions = sc.submissions().get(&token_pair).unwrap();
            assert_eq!(submissions.len(), SUBMISSION_COUNT - 1);

            for index in 0..SUBMISSION_COUNT - 1 {
                assert_eq!(
                    sc.oracle_status()
                        .get(&managed_address!(&state.oracles[index].to_address()))
                        .unwrap(),
                    OracleStatus {
                        total_submissions: 1,
                        accepted_submissions: 1
                    }
                );
            }
        });

    // submit last that resets the round
    state.submit(
        &state.oracles[SUBMISSION_COUNT - 1].clone(),
        100,
        rand::random::<u64>(),
    );
    state
        .world
        .write_scenario_trace("scenarios/stress_submit_test.scen.json");
}
