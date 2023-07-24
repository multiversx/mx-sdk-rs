use multiversx_price_aggregator_sc::PriceAggregator;
use multiversx_sc::types::{Address, EgldOrEsdtTokenIdentifier, MultiValueEncoded};
use multiversx_sc_modules::pause::EndpointWrappers;
use multiversx_sc_scenario::{managed_address, managed_biguint, managed_buffer, WhiteboxContract};

use multiversx_sc_scenario::{scenario_model::*, *};

const PRICE_AGGREGATOR_PATH_EXPR: &str = "file:output/multiversx-price-aggregator-sc.wasm";

pub const NR_ORACLES: usize = 4;
pub const SUBMISSION_COUNT: usize = 3;
pub const DECIMALS: u8 = 0;
pub static EGLD_TICKER: &[u8] = b"EGLD";
pub static USD_TICKER: &[u8] = b"USDC";

pub const STAKE_AMOUNT: u64 = 20;
pub const SLASH_AMOUNT: u64 = 10;
pub const SLASH_QUORUM: usize = 2;

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
fn price_agg_submit_test() {
    let mut world = world();
    let price_aggregator_whitebox = WhiteboxContract::new(
        "sc:price-aggregator",
        multiversx_price_aggregator_sc::contract_obj,
    );
    let price_aggregator_code = world.code_expression(PRICE_AGGREGATOR_PATH_EXPR);

    let mut oracles = Vec::new();
    for i in 0..NR_ORACLES {
        let oracle_address_expr = format!("address::oracle{i}");
        let oracle_address = Address::from_slice(oracle_address_expr.as_bytes());
        oracles.push(oracle_address);
    }


                // let mut oracle_args = MultiValueEncoded::new();
                // for oracle in &oracles {
                //     oracle_args.push(managed_address!(oracle));
                // }
                // sc.init(
                //     EgldOrEsdtTokenIdentifier::egld(),
                //     managed_biguint!(STAKE_AMOUNT),
                //     managed_biguint!(SLASH_AMOUNT),
                //     SLASH_QUORUM,
                //     SUBMISSION_COUNT,
                //     oracle_args,
                // )

    world
        .set_state_step(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:price-aggregator")
                .block_timestamp(100)
                .put_account(
                    "address:oracle0",
                    Account::new().nonce(1).balance(STAKE_AMOUNT),
                ),
        )
        .sc_deploy(
            ScDeployStep::new()
                .from("address:owner")
                .code(price_aggregator_code)
                .argument()
            },
    // .whitebox_deploy(
    //     &price_aggregator_whitebox,
    //     ScDeployStep::new()
    //         .from("address:owner")
    //         .code(price_aggregator_code),
    //     |sc| {
    //         let mut oracle_args = MultiValueEncoded::new();
    //         for oracle in &oracles {
    //             oracle_args.push(managed_address!(oracle));
    //         }
    //         sc.init(
    //             EgldOrEsdtTokenIdentifier::egld(),
    //             managed_biguint!(STAKE_AMOUNT),
    //             managed_biguint!(SLASH_AMOUNT),
    //             SLASH_QUORUM,
    //             SUBMISSION_COUNT,
    //             oracle_args,
    //         )
    //     },
    // )
    // .whitebox_call(
    //     &price_aggregator_whitebox,
    //     ScCallStep::new().from("address:owner"),
    //     |sc| {
    //         sc.set_pair_decimals(
    //             managed_buffer!(EGLD_TICKER),
    //             managed_buffer!(USD_TICKER),
    //             DECIMALS,
    //         )
    //     },
    // )
    // .whitebox_call_check(
    //     &price_aggregator_whitebox,
    //     ScCallStep::new().from("address:oracle0"),
    //     |sc| {
    //         sc.submit(
    //             managed_buffer!(EGLD_TICKER),
    //             managed_buffer!(USD_TICKER),
    //             99,
    //             managed_biguint!(100),
    //             DECIMALS,
    //         )
    //     },
    //     |r| {
    //         r.assert_user_error("Contract is paused");
    //     },
    // )
    // .whitebox_call(
    //     &price_aggregator_whitebox,
    //     ScCallStep::new().from("address:owner"),
    //     |sc| sc.call_unpause_endpoint(),
    // )
    // .whitebox_call_check(
    //     &price_aggregator_whitebox,
    //     ScCallStep::new().from("address:oracle0"),
    //     |sc| {
    //         sc.submit(
    //             managed_buffer!(EGLD_TICKER),
    //             managed_buffer!(USD_TICKER),
    //             10,
    //             managed_biguint!(100),
    //             DECIMALS,
    //         )
    //     },
    //     |r| {
    //         r.assert_user_error("First submission too old");
    //     },
    // )
    // .whitebox_call_check(
    //     &price_aggregator_whitebox,
    //     ScCallStep::new().from("address:oracle0"),
    //     |sc| {
    //         sc.call_unpause_endpoint();
    //         sc.submit(
    //             managed_buffer!(EGLD_TICKER),
    //             managed_buffer!(USD_TICKER),
    //             95,
    //             managed_biguint!(100),
    //             DECIMALS,
    //         )
    //     },
    //     |r| {
    //         r.assert_ok();
    //     },
    // );

    // pa_setup
    //     .b_mock
    //     .execute_query(&pa_setup.price_agg, |sc| {
    //         let token_pair = TokenPair {
    //             from: managed_buffer!(EGLD_TICKER),
    //             to: managed_buffer!(USD_TICKER),
    //         };
    //         assert_eq!(
    //             sc.first_submission_timestamp(&token_pair).get(),
    //             current_timestamp
    //         );
    //         assert_eq!(
    //             sc.last_submission_timestamp(&token_pair).get(),
    //             current_timestamp
    //         );

    //         let submissions = sc.submissions().get(&token_pair).unwrap();
    //         assert_eq!(submissions.len(), 1);
    //         assert_eq!(
    //             submissions.get(&managed_address!(&oracles[0])).unwrap(),
    //             managed_biguint!(100)
    //         );

    //         assert_eq!(
    //             sc.oracle_status()
    //                 .get(&managed_address!(&oracles[0]))
    //                 .unwrap(),
    //             OracleStatus {
    //                 total_submissions: 1,
    //                 accepted_submissions: 1
    //             }
    //         );
    //     })
    //     .assert_ok();

    // // first oracle submit again - submission not accepted
    // pa_setup.submit(&oracles[0], 95, 100).assert_ok();

    // pa_setup
    //     .b_mock
    //     .execute_query(&pa_setup.price_agg, |sc| {
    //         assert_eq!(
    //             sc.oracle_status()
    //                 .get(&managed_address!(&oracles[0]))
    //                 .unwrap(),
    //             OracleStatus {
    //                 total_submissions: 2,
    //                 accepted_submissions: 1
    //             }
    //         );
    //     })
    //     .assert_ok();
}
