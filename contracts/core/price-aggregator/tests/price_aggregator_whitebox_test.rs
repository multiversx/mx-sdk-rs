use multiversx_price_aggregator_sc::{
    price_aggregator_data::{OracleStatus, TokenPair},
    staking::EndpointWrappers as StakingEndpointWrappers,
    PriceAggregator,
};
use multiversx_sc::types::{EgldOrEsdtTokenIdentifier, MultiValueEncoded};
use multiversx_sc_modules::pause::EndpointWrappers as PauseEndpointWrappers;
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_buffer, scenario_model::*, WhiteboxContract, *,
};

pub const DECIMALS: u8 = 0;
pub const EGLD_TICKER: &[u8] = b"EGLD";
pub const NR_ORACLES: usize = 4;
pub const SLASH_AMOUNT: u64 = 10;
pub const SLASH_QUORUM: usize = 2;
pub const STAKE_AMOUNT: u64 = 20;
pub const SUBMISSION_COUNT: usize = 3;
pub const USD_TICKER: &[u8] = b"USDC";

const OWNER_ADDRESS_EXPR: &str = "address:owner";
const PRICE_AGGREGATOR_ADDRESS_EXPR: &str = "sc:price-aggregator";
const PRICE_AGGREGATOR_PATH_EXPR: &str = "file:output/multiversx-price-aggregator-sc.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/core/price-aggregator");
    blockchain.register_contract(
        PRICE_AGGREGATOR_PATH_EXPR,
        multiversx_price_aggregator_sc::ContractBuilder,
    );

    blockchain
}

#[test]
fn test_price_aggregator_submit() {
    let mut world = world();
    let current_timestamp = 100;
    let price_aggregator_whitebox = WhiteboxContract::new(
        PRICE_AGGREGATOR_ADDRESS_EXPR,
        multiversx_price_aggregator_sc::contract_obj,
    );
    let price_aggregator_code = world.code_expression(PRICE_AGGREGATOR_PATH_EXPR);

    let mut set_state_step = SetStateStep::new()
        .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
        .new_address(OWNER_ADDRESS_EXPR, 1, PRICE_AGGREGATOR_ADDRESS_EXPR)
        .block_timestamp(100);

    let mut oracles = Vec::new();
    for i in 1..=NR_ORACLES {
        let oracle_address_expr = format!("address:oracle{i}");
        let oracle_address = AddressValue::from(oracle_address_expr.as_str());

        set_state_step = set_state_step.put_account(
            oracle_address_expr.as_str(),
            Account::new().nonce(1).balance(STAKE_AMOUNT),
        );
        oracles.push(oracle_address);
    }

    world.set_state_step(set_state_step).whitebox_deploy(
        &price_aggregator_whitebox,
        ScDeployStep::new()
            .from(OWNER_ADDRESS_EXPR)
            .code(price_aggregator_code),
        |sc| {
            let mut oracle_args = MultiValueEncoded::new();
            for oracle_address in &oracles {
                oracle_args.push(managed_address!(&oracle_address.to_address()));
            }

            sc.init(
                EgldOrEsdtTokenIdentifier::egld(),
                managed_biguint!(STAKE_AMOUNT),
                managed_biguint!(SLASH_AMOUNT),
                SLASH_QUORUM,
                SUBMISSION_COUNT,
                oracle_args,
            )
        },
    );

    for oracle_address in &oracles {
        world.whitebox_call(
            &price_aggregator_whitebox,
            ScCallStep::new()
                .from(oracle_address)
                .egld_value(STAKE_AMOUNT),
            |sc| sc.call_stake(),
        );
    }

    world
        .whitebox_call(
            &price_aggregator_whitebox,
            ScCallStep::new().from(OWNER_ADDRESS_EXPR),
            |sc| {
                sc.set_pair_decimals(
                    managed_buffer!(EGLD_TICKER),
                    managed_buffer!(USD_TICKER),
                    DECIMALS,
                )
            },
        )
        .whitebox_call_check(
            &price_aggregator_whitebox,
            ScCallStep::new()
                .from(&oracles[0])
                .expect(TxExpect::user_error("str:Contract is paused")),
            |sc| {
                sc.submit(
                    managed_buffer!(EGLD_TICKER),
                    managed_buffer!(USD_TICKER),
                    99,
                    managed_biguint!(100),
                    DECIMALS,
                )
            },
            |r| r.assert_user_error("Contract is paused"),
        )
        .whitebox_call(
            &price_aggregator_whitebox,
            ScCallStep::new().from(OWNER_ADDRESS_EXPR),
            |sc| sc.call_unpause_endpoint(),
        )
        .whitebox_call_check(
            &price_aggregator_whitebox,
            ScCallStep::new().from(&oracles[0]).no_expect(),
            |sc| {
                sc.submit(
                    managed_buffer!(EGLD_TICKER),
                    managed_buffer!(USD_TICKER),
                    10,
                    managed_biguint!(100),
                    DECIMALS,
                )
            },
            |r| {
                r.assert_user_error("First submission too old");
            },
        )
        .whitebox_call_check(
            &price_aggregator_whitebox,
            ScCallStep::new().from(&oracles[0]),
            |sc| {
                sc.submit(
                    managed_buffer!(EGLD_TICKER),
                    managed_buffer!(USD_TICKER),
                    95,
                    managed_biguint!(100),
                    DECIMALS,
                )
            },
            |r| {
                r.assert_ok();
            },
        );

    world.whitebox_query_check(
        &price_aggregator_whitebox,
        |sc| {
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
                    .get(&managed_address!(&oracles[0].to_address()))
                    .unwrap(),
                managed_biguint!(100)
            );

            assert_eq!(
                sc.oracle_status()
                    .get(&managed_address!(&oracles[0].to_address()))
                    .unwrap(),
                OracleStatus {
                    total_submissions: 1,
                    accepted_submissions: 1
                }
            );
        },
        |r| r.assert_ok(),
    );

    world.whitebox_call_check(
        &price_aggregator_whitebox,
        ScCallStep::new().from(&oracles[0]),
        |sc| {
            sc.submit(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                95,
                managed_biguint!(100),
                DECIMALS,
            )
        },
        |r| r.assert_ok(),
    );

    world.whitebox_query_check(
        &price_aggregator_whitebox,
        |sc| {
            assert_eq!(
                sc.oracle_status()
                    .get(&managed_address!(&oracles[0].to_address()))
                    .unwrap(),
                OracleStatus {
                    total_submissions: 2,
                    accepted_submissions: 1
                }
            );
        },
        |r| r.assert_ok(),
    );
}
