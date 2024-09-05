use multiversx_sc_scenario::imports::*;
use scenario_tester::*;

const ST_PATH_EXPR: MxscPath = MxscPath::new("mxsc:output/scenario-tester.mxsc.json");
const OWNER: TestAddress = TestAddress::new("owner");
const SCENARIO_TESTER: TestSCAddress = TestSCAddress::new("scenario-tester");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "mxsc:output/scenario-tester.mxsc.json",
        scenario_tester::ContractBuilder,
    );
    blockchain
}

#[test]
fn st_whitebox() {
    let mut world = world();

    world.account(OWNER).nonce(1);

    let new_address = world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(ST_PATH_EXPR)
        .new_address(SCENARIO_TESTER)
        .returns(ReturnsNewBech32Address)
        .whitebox(scenario_tester::contract_obj, |sc| {
            sc.init(BigUint::from(5u64));
        });

    assert_eq!(new_address.to_address(), SCENARIO_TESTER.to_address());

    world
        .query()
        .to(SCENARIO_TESTER)
        .whitebox(scenario_tester::contract_obj, |sc| {
            let sum_value = sc.sum();
            assert_eq!(sum_value.get(), BigUint::from(5u32));
        });

    world
        .tx()
        .from(OWNER)
        .to(SCENARIO_TESTER)
        .whitebox(scenario_tester::contract_obj, |sc| sc.add(3u32.into()));

    world
        .check_account(SCENARIO_TESTER)
        .check_storage("str:sum", "8");
}
