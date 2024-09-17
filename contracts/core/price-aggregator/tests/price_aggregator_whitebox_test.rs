use multiversx_price_aggregator_sc::{
    price_aggregator_data::{OracleStatus, TimestampedPrice, TokenPair},
    PriceAggregator, MAX_ROUND_DURATION_SECONDS,
};
use multiversx_sc_modules::{
    pause::EndpointWrappers as PauseEndpointWrappers,
    staking::{EndpointWrappers as StakingEndpointWrappers, StakingModule},
};
use multiversx_sc_scenario::imports::*;

pub const DECIMALS: u8 = 0;
pub const EGLD_TICKER: &[u8] = b"EGLD";
pub const NR_ORACLES: usize = 4;
pub const SLASH_AMOUNT: u64 = 10;
pub const SLASH_QUORUM: usize = 3;
pub const STAKE_AMOUNT: u64 = 20;
pub const SUBMISSION_COUNT: usize = 3;
pub const USD_TICKER: &[u8] = b"USDC";

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const PRICE_AGGREGATOR_ADDRESS: TestSCAddress = TestSCAddress::new("price-aggregator");
const PRICE_AGGREGATOR_PATH_EXPR: MxscPath =
    MxscPath::new("mxsc:output/multiversx-price-aggregator-sc.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        PRICE_AGGREGATOR_PATH_EXPR,
        multiversx_price_aggregator_sc::ContractBuilder,
    );

    blockchain
}

#[test]
fn test_price_aggregator_submit() {
    let (mut world, oracles) = setup();

    // configure the number of decimals
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.set_pair_decimals(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                DECIMALS,
            )
        });

    // try submit while paused
    world
        .tx()
        .from(&oracles[0])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .returns(ExpectError(4u64, "Contract is paused"))
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.submit(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                99,
                managed_biguint!(100),
                DECIMALS,
            )
        });

    // unpause
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.call_unpause_endpoint();
        });

    // submit first timestamp too old
    world
        .tx()
        .from(&oracles[0])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .returns(ExpectError(4u64, "First submission too old"))
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.submit(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                10,
                managed_biguint!(100),
                DECIMALS,
            )
        });

    // submit ok
    world
        .tx()
        .from(&oracles[0])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.submit(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                95,
                managed_biguint!(100),
                DECIMALS,
            )
        });

    let current_timestamp = 100;
    world.query().to(PRICE_AGGREGATOR_ADDRESS).whitebox(
        multiversx_price_aggregator_sc::contract_obj,
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
                submissions.get(&ManagedAddress::from(&oracles[0])).unwrap(),
                managed_biguint!(100)
            );

            assert_eq!(
                sc.oracle_status()
                    .get(&ManagedAddress::from(&oracles[0]))
                    .unwrap(),
                OracleStatus {
                    total_submissions: 1,
                    accepted_submissions: 1
                }
            );
        },
    );

    // first oracle submit again - submission not accepted
    world
        .tx()
        .from(&oracles[0])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.submit(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                95,
                managed_biguint!(100),
                DECIMALS,
            )
        });

    world.query().to(PRICE_AGGREGATOR_ADDRESS).whitebox(
        multiversx_price_aggregator_sc::contract_obj,
        |sc| {
            assert_eq!(
                sc.oracle_status()
                    .get(&ManagedAddress::from(&oracles[0]))
                    .unwrap(),
                OracleStatus {
                    total_submissions: 2,
                    accepted_submissions: 1
                }
            );
        },
    );
}

#[test]
fn test_price_aggregator_submit_round_ok() {
    let (mut world, oracles) = setup();

    // configure the number of decimals
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.set_pair_decimals(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                DECIMALS,
            )
        });

    // unpause
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.call_unpause_endpoint();
        });

    // submit first
    world
        .tx()
        .from(&oracles[0])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.submit(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                95,
                managed_biguint!(10_000),
                DECIMALS,
            )
        });

    let current_timestamp = 110;
    world.current_block().block_timestamp(current_timestamp);

    // submit second
    world
        .tx()
        .from(&oracles[1])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.submit(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                101,
                managed_biguint!(11_000),
                DECIMALS,
            )
        });

    // submit third
    world
        .tx()
        .from(&oracles[2])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.submit(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                105,
                managed_biguint!(12_000),
                DECIMALS,
            )
        });

    world.query().to(PRICE_AGGREGATOR_ADDRESS).whitebox(
        multiversx_price_aggregator_sc::contract_obj,
        |sc| {
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
        },
    );
}

#[test]
fn test_price_aggregator_discarded_round() {
    let (mut world, oracles) = setup();

    // configure the number of decimals
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.set_pair_decimals(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                DECIMALS,
            )
        });

    // unpause
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.call_unpause_endpoint();
        });

    // submit first
    world
        .tx()
        .from(&oracles[0])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.submit(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                95,
                managed_biguint!(10_000),
                DECIMALS,
            )
        });

    let current_timestamp = 100 + MAX_ROUND_DURATION_SECONDS + 1;
    world.current_block().block_timestamp(current_timestamp);

    // submit second - this will discard the previous submission
    world
        .tx()
        .from(&oracles[1])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.submit(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                current_timestamp - 1,
                managed_biguint!(11_000),
                DECIMALS,
            )
        });

    world.query().to(PRICE_AGGREGATOR_ADDRESS).whitebox(
        multiversx_price_aggregator_sc::contract_obj,
        |sc| {
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
        },
    );
}

#[test]
fn test_price_aggregator_slashing() {
    let (mut world, oracles) = setup();

    // configure the number of decimals
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.set_pair_decimals(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                DECIMALS,
            )
        });

    // unpause
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.call_unpause_endpoint();
        });

    world
        .tx()
        .from(&oracles[0])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.vote_slash_member(ManagedAddress::from(&oracles[1]));
        });

    world
        .tx()
        .from(&oracles[2])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.vote_slash_member(ManagedAddress::from(&oracles[1]))
        });

    world
        .tx()
        .from(&oracles[3])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.vote_slash_member(ManagedAddress::from(&oracles[1]));
        });

    world.query().to(PRICE_AGGREGATOR_ADDRESS).whitebox(
        multiversx_price_aggregator_sc::contract_obj,
        |sc| {
            let list = sc.slashing_proposal_voters(&ManagedAddress::from(&oracles[1]));
            assert!(list.contains(&ManagedAddress::from(&oracles[0])));
            assert!(list.contains(&ManagedAddress::from(&oracles[2])));
            assert!(list.contains(&ManagedAddress::from(&oracles[3])));
        },
    );

    world
        .tx()
        .from(&oracles[0])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.slash_member(ManagedAddress::from(&oracles[1]));
        });

    // oracle 1 try submit after slashing
    world
        .tx()
        .from(&oracles[1])
        .to(PRICE_AGGREGATOR_ADDRESS)
        .returns(ExpectError(4u64, "only oracles allowed"))
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            sc.submit(
                managed_buffer!(EGLD_TICKER),
                managed_buffer!(USD_TICKER),
                95,
                managed_biguint!(10_000),
                DECIMALS,
            )
        });
}

fn setup() -> (ScenarioWorld, Vec<Address>) {
    // setup
    let mut world = world();

    world.account(OWNER_ADDRESS).nonce(1);
    world.current_block().block_timestamp(100);

    let mut oracles = Vec::new();
    for i in 1..=NR_ORACLES {
        let oracle_address_expr = format!("oracle{i}");
        let oracle_address = TestAddress::new(&oracle_address_expr);

        world.account(oracle_address).nonce(1).balance(STAKE_AMOUNT);

        oracles.push(oracle_address.to_address());
    }

    // init price aggregator
    world
        .tx()
        .from(OWNER_ADDRESS)
        .raw_deploy()
        .code(PRICE_AGGREGATOR_PATH_EXPR)
        .new_address(PRICE_AGGREGATOR_ADDRESS)
        .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
            let mut oracle_args = MultiValueEncoded::new();
            for oracle_address in &oracles {
                oracle_args.push(ManagedAddress::from(oracle_address));
            }

            sc.init(
                EgldOrEsdtTokenIdentifier::egld(),
                managed_biguint!(STAKE_AMOUNT),
                managed_biguint!(SLASH_AMOUNT),
                SLASH_QUORUM,
                SUBMISSION_COUNT,
                oracle_args,
            )
        });

    for oracle_address in &oracles {
        world
            .tx()
            .from(oracle_address)
            .to(PRICE_AGGREGATOR_ADDRESS)
            .egld(STAKE_AMOUNT)
            .whitebox(multiversx_price_aggregator_sc::contract_obj, |sc| {
                sc.call_stake();
            });
    }

    (world, oracles)
}
