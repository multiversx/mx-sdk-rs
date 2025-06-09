#![allow(deprecated)]

use multiversx_sc_scenario::imports::*;
use num_bigint::BigUint;

use scenario_tester::*;

const OWNER: TestAddress = TestAddress::new("owner");
const CODE_EXPR: MxscPath = MxscPath::new("output/scenario-tester.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/scenario-tester");
    blockchain.register_contract(CODE_EXPR, scenario_tester::ContractBuilder);
    blockchain
}

#[test]
fn st_blackbox_legacy_proxy() {
    let mut world = world();
    let owner_address = "address:owner";
    let mut st_contract = ContractInfo::<scenario_tester::Proxy<StaticApi>>::new("sc:adder");

    world.start_trace();

    world.set_state_step(
        SetStateStep::new()
            .put_account(owner_address, Account::new().nonce(1))
            .new_address(owner_address, 1, "sc:adder"),
    );

    world
        .tx()
        .from(OWNER)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .init(5u32)
        .code(CODE_EXPR)
        .with_result(WithNewAddress::new(|new_address| {
            assert_eq!(new_address.to_address(), st_contract.to_address());
        }))
        .run();

    world.sc_query(
        ScQueryStep::new()
            .to(&st_contract)
            .call(st_contract.sum())
            .expect_value(SingleValue::from(BigUint::from(5u32))),
    );

    let value = world
        .query()
        .legacy_proxy_call(st_contract.sum())
        .returns(ReturnsResultAs::<SingleValue<BigUint>>::new())
        .run();
    assert_eq!(value.into(), BigUint::from(5u32));

    world
        .tx()
        .from(OWNER)
        .legacy_proxy_call(st_contract.add(3u32))
        .with_result(WithRawTxResponse(|response| {
            assert!(response.tx_error.is_success());
        }))
        .run();

    world.check_state_step(
        CheckStateStep::new()
            .put_account(owner_address, CheckAccount::new())
            .put_account(
                &st_contract,
                CheckAccount::new().check_storage("str:sum", "8"),
            ),
    );

    world.write_scenario_trace("trace1.scen.json");
}
