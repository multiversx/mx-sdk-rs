use multiversx_price_aggregator_sc::{
    price_aggregator_data::{OracleStatus, TimestampedPrice, TokenPair},
    staking::StakingModule,
    PriceAggregator, MAX_ROUND_DURATION_SECONDS,
};
use multiversx_sc_scenario::{managed_address, managed_biguint, managed_buffer, rust_biguint};

mod price_agg_setup;
use price_agg_setup::*;

#[test]
fn price_agg_submit_test() {
    let mut pa_setup = PriceAggSetup::new(multiversx_price_aggregator_sc::contract_obj);
    let current_timestamp = 100;
    let oracles = pa_setup.oracles.clone();

    // configure the number of decimals
    pa_setup.set_pair_decimals(EGLD_TICKER, USD_TICKER, DECIMALS);

    // try submit while paused
    pa_setup
        .submit(&oracles[0], 99, 100)
        .assert_user_error("Contract is paused");

    // unpause
    pa_setup.unpause();

    // submit first timestamp too old
    pa_setup
        .submit(&oracles[0], 10, 100)
        .assert_user_error("First submission too old");

    // submit ok
    pa_setup.submit(&oracles[0], 95, 100).assert_ok();

    pa_setup
        .b_mock
        .execute_query(&pa_setup.price_agg, |sc| {
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
                submissions.get(&managed_address!(&oracles[0])).unwrap(),
                managed_biguint!(100)
            );

            assert_eq!(
                sc.oracle_status()
                    .get(&managed_address!(&oracles[0]))
                    .unwrap(),
                OracleStatus {
                    total_submissions: 1,
                    accepted_submissions: 1
                }
            );
        })
        .assert_ok();

    // first oracle submit again - submission not accepted
    pa_setup.submit(&oracles[0], 95, 100).assert_ok();

    pa_setup
        .b_mock
        .execute_query(&pa_setup.price_agg, |sc| {
            assert_eq!(
                sc.oracle_status()
                    .get(&managed_address!(&oracles[0]))
                    .unwrap(),
                OracleStatus {
                    total_submissions: 2,
                    accepted_submissions: 1
                }
            );
        })
        .assert_ok();
}

#[test]
fn price_agg_submit_round_ok_test() {
    let mut pa_setup = PriceAggSetup::new(multiversx_price_aggregator_sc::contract_obj);
    let oracles = pa_setup.oracles.clone();

    // configure the number of decimals
    pa_setup.set_pair_decimals(EGLD_TICKER, USD_TICKER, DECIMALS);

    // unpause
    pa_setup.unpause();

    // submit first
    pa_setup.submit(&oracles[0], 95, 10_000).assert_ok();

    let current_timestamp = 110;
    pa_setup.b_mock.set_block_timestamp(current_timestamp);

    // submit second
    pa_setup.submit(&oracles[1], 101, 11_000).assert_ok();

    // submit third
    pa_setup.submit(&oracles[2], 105, 12_000).assert_ok();

    pa_setup
        .b_mock
        .execute_query(&pa_setup.price_agg, |sc| {
            let result = sc
                .latest_price_feed(managed_buffer!(EGLD_TICKER), managed_buffer!(USD_TICKER))
                .unwrap();

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
        })
        .assert_ok();
}

#[test]
fn price_agg_discarded_round_test() {
    let mut pa_setup = PriceAggSetup::new(multiversx_price_aggregator_sc::contract_obj);
    let oracles = pa_setup.oracles.clone();

    // configure the number of decimals
    pa_setup.set_pair_decimals(EGLD_TICKER, USD_TICKER, DECIMALS);

    // unpause
    pa_setup.unpause();

    // submit first
    pa_setup.submit(&oracles[0], 95, 10_000).assert_ok();

    let current_timestamp = 100 + MAX_ROUND_DURATION_SECONDS + 1;
    pa_setup.b_mock.set_block_timestamp(current_timestamp);

    // submit second - this will discard the previous submission
    pa_setup
        .submit(&oracles[1], current_timestamp - 1, 11_000)
        .assert_ok();

    pa_setup
        .b_mock
        .execute_query(&pa_setup.price_agg, |sc| {
            let token_pair = TokenPair {
                from: managed_buffer!(EGLD_TICKER),
                to: managed_buffer!(USD_TICKER),
            };
            let submissions = sc.submissions().get(&token_pair).unwrap();
            assert_eq!(submissions.len(), 1);
            assert_eq!(
                submissions.get(&managed_address!(&oracles[1])).unwrap(),
                managed_biguint!(11_000)
            );
        })
        .assert_ok();
}

#[test]
fn price_agg_slashing_test() {
    let rust_zero = rust_biguint!(0);
    let mut pa_setup = PriceAggSetup::new(multiversx_price_aggregator_sc::contract_obj);
    let oracles = pa_setup.oracles.clone();

    // unpause
    pa_setup.unpause();

    pa_setup
        .b_mock
        .execute_tx(&oracles[0], &pa_setup.price_agg, &rust_zero, |sc| {
            sc.vote_slash_member(managed_address!(&oracles[1]));
        })
        .assert_ok();

    pa_setup
        .b_mock
        .execute_tx(&oracles[2], &pa_setup.price_agg, &rust_zero, |sc| {
            sc.vote_slash_member(managed_address!(&oracles[1]));
        })
        .assert_ok();

    pa_setup
        .b_mock
        .execute_tx(&oracles[0], &pa_setup.price_agg, &rust_zero, |sc| {
            sc.slash_member(managed_address!(&oracles[1]));
        })
        .assert_ok();

    // oracle 1 try submit after slashing
    pa_setup
        .submit(&oracles[1], 95, 10_000)
        .assert_user_error("only oracles allowed");
}
