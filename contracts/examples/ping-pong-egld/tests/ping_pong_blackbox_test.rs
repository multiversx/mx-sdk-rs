use multiversx_sc_scenario::imports::*;

use ping_pong_egld::*;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("ping-pong");
const CODE_PATH: MxscPath = MxscPath::new("output/ping-pong-egld.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/examples/ping-pong-egld");
    blockchain.register_contract(CODE_PATH, ping_pong_egld::ContractBuilder);
    blockchain
}

#[test]
fn ping_pong_storage_check_blackbox_test() {
    let mut world = world();

    world.start_trace();

    world.account(OWNER_ADDRESS).nonce(1);

    // set value for 4 keys in storage
    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy_ping_pong_egld::PingPongProxy)
        .init(
            BigUint::from(1u64),
            100u64,
            Some(178737u64),
            OptionalValue::Some(BigUint::from(100u64)),
        )
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS.to_address());

    // check value for only 2 keys
    world
        .check_account(SC_ADDRESS)
        .check_storage("str:deadline", "178837")
        .check_storage("str:activationTimestamp", "178737");

    world.write_scenario_trace("scenarios/ping-pong-partial-key-check.scen.json");
}
