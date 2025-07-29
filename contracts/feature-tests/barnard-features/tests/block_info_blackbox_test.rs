use multiversx_sc_scenario::imports::*;

use barnard_features::barnard_features_proxy;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("barnard-features");
const CODE_PATH: MxscPath = MxscPath::new("output/barnard-features.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/barnard-features");
    blockchain.register_contract(CODE_PATH, barnard_features::ContractBuilder);
    blockchain
}

#[test]
fn block_info_blackbox() {
    let mut world = world();

    world.account(OWNER_ADDRESS).nonce(1);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(barnard_features_proxy::BarnardFeaturesProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();

    world
        .epoch_start_block()
        .block_timestamp_ms(123_000_000)
        .block_nonce(15_000)
        .block_round(17_000);

    world.block_round_time_ms(600);

    let result = world
        .query()
        .to(SC_ADDRESS)
        .typed(barnard_features_proxy::BarnardFeaturesProxy)
        .epoch_info()
        .returns(ReturnsResult)
        .run();

    let (
        block_round_time_ms,
        epoch_start_block_timestamp_ms,
        epoch_start_block_nonce,
        epoch_start_block_round,
    ) = result.into_tuple();

    assert_eq!(block_round_time_ms, 600);
    assert_eq!(epoch_start_block_timestamp_ms, 123_000_000);
    assert_eq!(epoch_start_block_nonce, 15_000);
    assert_eq!(epoch_start_block_round, 17_000);
}
