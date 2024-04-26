use multiversx_sc_scenario::imports::*;
use num_bigint::BigUint;

use scenario_tester::*;

const SC_SCENARIO_TESTER_PATH_EXPR: &str = "mxsc:output/scenario-tester.mxsc.json";

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const OTHER_ADDRESS: TestAddress = TestAddress::new("other");
const ST_ADDRESS: TestSCAddress = TestSCAddress::new("scenario-tester");
const CODE_PATH: MxscPath = MxscPath::new("output/scenario-tester.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/scenario-tester");

    blockchain.register_contract(
        SC_SCENARIO_TESTER_PATH_EXPR,
        scenario_tester::ContractBuilder,
    );
    blockchain
}

#[test]
fn st_blackbox() {
    let mut world = world();

    world.start_trace();

    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .balance(100)
        .account(OTHER_ADDRESS)
        .nonce(2)
        .balance(300)
        .esdt_balance("str:TOKEN-123456", "500")
        .commit();

    world
        .check_account(OWNER_ADDRESS)
        .nonce(1)
        .balance(100)
        .check_account(OTHER_ADDRESS)
        .nonce(2)
        .balance(300)
        .esdt_balance("str:TOKEN-123456", "500")
        .commit();

    world.new_address(OWNER_ADDRESS, 1, ST_ADDRESS);

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .init(5u32)
        .code(CODE_PATH)
        .returns(ReturnsNewAddress)
        .run();
    assert_eq!(new_address, ST_ADDRESS.to_address());

    let value = world
        .query()
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .sum()
        .returns(ReturnsResultConv::<BigUint>::new())
        .run();
    assert_eq!(value, BigUint::from(5u32));

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .add(1u32)
        .run();

    world
        .check_account(OWNER_ADDRESS)
        .nonce(3)
        .balance(100)
        .check_account(ST_ADDRESS)
        .check_storage("str:sum", "6")
        .commit();

    world
        .tx()
        .from(OTHER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .add(1u32)
        .run();

    world.write_scenario_trace("trace1.scen.json");
}

#[test]
fn set_state_test() {
    let mut world = world();
    let first = TestAddress::new("first");
    let second = TestAddress::new("second");
    let third = TestAddress::new("third");
    let fourth = TestAddress::new("fourth");
    let fifth = TestAddress::new("fifth");
    let sixth = TestAddress::new("sixth");

    world.start_trace();

    world
        .account(first)
        .nonce(1)
        .balance(100)
        .account(second)
        .nonce(2)
        .balance(300)
        .esdt_balance("str:TOKEN-123456", "500")
        .commit();

    world
        .check_account(first)
        .nonce(1)
        .balance(100)
        .check_account(second)
        .nonce(2)
        .balance(300)
        .esdt_balance("str:TOKEN-123456", "500")
        .commit();

    world
        .account(third)
        .nonce(3)
        .balance(50)
        .esdt_nft_balance("str:NFT-123456", "2", "1", Some(Vec::<u8>::new()))
        .commit();

    world
        .check_account(third)
        .nonce(3)
        .balance(50)
        .esdt_nft_balance_and_attributes("str:NFT-123456", "2", "1", Some(Vec::<u8>::new()))
        .commit();

    // using no commit should drop the value naturally
    world
        .account(fourth)
        .nonce(4)
        .balance(400)
        .account(fifth)
        .nonce(5)
        .balance(250)
        .esdt_balance("str:TOKEN-123456", "2");

    world
        .check_account(fourth)
        .nonce(4)
        .balance(400)
        .check_account(fifth)
        .nonce(5)
        .balance(250)
        .esdt_balance("str:TOKEN-123456", "2");

    world
        .account(sixth)
        .nonce(6)
        .balance(600)
        .esdt_balance("str:TOKEN-123456", "60");

    world
        .check_account(sixth)
        .nonce(6)
        .balance(600)
        .esdt_balance("str:TOKEN-123456", "60");
}
