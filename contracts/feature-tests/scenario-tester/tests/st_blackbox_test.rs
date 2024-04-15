use multiversx_sc_scenario::imports::*;
use num_bigint::BigUint;

use scenario_tester::*;

const SC_SCENARIO_TESTER_PATH_EXPR: &str = "mxsc:output/scenario-tester.mxsc.json";

const OWNER: AddressExpr = AddressExpr("owner");
const OTHER: AddressExpr = AddressExpr("other");
const SC_SCENARIO_TESTER: ScExpr = ScExpr("scenario-tester");
const CODE_EXPR: MxscExpr = MxscExpr("output/scenario-tester.mxsc.json");

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
    let owner_address = "address:owner";
    let other_address = "address:other";

    let st_contract = ContractInfo::<scenario_tester::Proxy<StaticApi>>::new("sc:scenario-tester");

    world.start_trace();

    world
        .account(owner_address)
        .nonce(1)
        .balance("100")
        .account(other_address)
        .nonce(2)
        .balance("300")
        .esdt_balance("str:TOKEN-123456", "500")
        .commit();

    world
        .check_state_account(owner_address)
        .nonce("1")
        .balance("100")
        .check_state_account(other_address)
        .nonce("2")
        .balance("300")
        .esdt_balance("str:TOKEN-123456", "500")
        .commit();

    world.set_state_step(SetStateStep::new().new_address(owner_address, 1, "sc:scenario-tester"));

    let new_address = world
        .tx()
        .from(OWNER)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .init(5u32)
        .code(CODE_EXPR)
        .returns(ReturnsNewAddress)
        .run();
    assert_eq!(new_address, st_contract.to_address());

    let value = world
        .query()
        .to(SC_SCENARIO_TESTER)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .sum()
        .returns(ReturnsResultConv::<BigUint>::new())
        .run();
    assert_eq!(value, BigUint::from(5u32));

    world
        .tx()
        .from(OWNER)
        .to(SC_SCENARIO_TESTER)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .add(1u32)
        .run();

    world
        .check_state_account(owner_address)
        .nonce("3")
        .balance("100")
        .check_state_account(st_contract)
        .check_storage("str:sum", "6")
        .commit();

    world
        .tx()
        .from(OTHER)
        .to(SC_SCENARIO_TESTER)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .add(1u32)
        .run();

    world.write_scenario_trace("trace1.scen.json");
}

#[test]
fn set_state_test() {
    let mut world = world();
    let first = "address:first";
    let second = "address:second";
    let third = "address:third";
    let fourth = "address:fourth";
    let fifth = "address:fifth";
    let sixth = "address:sixth";

    world.start_trace();

    world
        .account(first)
        .nonce(1)
        .balance("100")
        .account(second)
        .nonce(2)
        .balance("300")
        .esdt_balance("str:TOKEN-123456", "500")
        .commit();

    world
        .check_state_account(first)
        .nonce(1)
        .balance("100")
        .check_state_account(second)
        .nonce(2)
        .balance("300")
        .esdt_balance("str:TOKEN-123456", "500")
        .commit();

    world
        .account(third)
        .nonce(3)
        .balance("50")
        .esdt_nft_balance("str:NFT-123456", "2", "1", Some(Vec::<u8>::new()))
        .commit();

    world
        .check_state_account(third)
        .nonce(3)
        .balance("50")
        .esdt_nft_balance_and_attributes("str:NFT-123456", "2", "1", Some(Vec::<u8>::new()))
        .commit();

    // using no commit should drop the value naturally
    world
        .account(fourth)
        .nonce(4)
        .balance("400")
        .account(fifth)
        .nonce(5)
        .balance("250")
        .esdt_balance("str:TOKEN-123456", "2");

    world
        .check_state_account(fourth)
        .nonce(4)
        .balance("400")
        .check_state_account(fifth)
        .nonce(5)
        .balance("250")
        .esdt_balance("str:TOKEN-123456", "2");

    world
        .account(sixth)
        .nonce(6)
        .balance("600")
        .esdt_balance("str:TOKEN-123456", "60");

    world
        .check_state_account(sixth)
        .nonce(6)
        .balance("600")
        .esdt_balance("str:TOKEN-123456", "60");
}
