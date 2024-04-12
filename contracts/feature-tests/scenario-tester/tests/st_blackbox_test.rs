use multiversx_sc_scenario::{imports::*, scenario_model::U64Value};
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

    world.check_state_step(
        CheckStateStep::new()
            .put_account(
                owner_address,
                CheckAccount::new()
                    .nonce(U64Value::from(1u64))
                    .balance("100"),
            )
            .put_account(
                other_address,
                CheckAccount::new()
                    .nonce(U64Value::from(2u64))
                    .balance("300")
                    .esdt_balance("str:TOKEN-123456", "500"),
            ),
    );

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

    world.check_state_step(
        CheckStateStep::new()
            .put_account(owner_address, CheckAccount::new())
            .put_account(
                &st_contract,
                CheckAccount::new().check_storage("str:sum", "6"),
            ),
    );

    world
        .tx()
        .from(OTHER)
        .to(SC_SCENARIO_TESTER)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .add(1u32)
        .run();

    world.write_scenario_trace("trace1.scen.json");
}
