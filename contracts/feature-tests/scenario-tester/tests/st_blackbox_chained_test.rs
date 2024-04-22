use multiversx_sc_scenario::imports::*;
use num_bigint::BigUint;

use scenario_tester::*;

const ADDER_PATH_EXPR: &str = "mxsc:output/scenario-tester.mxsc.json";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/scenario-tester");

    blockchain.register_contract(ADDER_PATH_EXPR, scenario_tester::ContractBuilder);
    blockchain
}

#[test]
fn st_blackbox_chained() {
    let mut world = world();
    let owner_address = "address:owner";
    let st_contract = ContractInfo::<scenario_tester::Proxy<StaticApi>>::new("sc:adder");

    world
        .start_trace()
        .set_state_step(
            SetStateStep::new()
                .put_account(owner_address, Account::new().nonce(1))
                .new_address(owner_address, 1, "sc:adder"),
        )
        .chain_deploy(|tx| {
            tx.from(AddressExpr("owner"))
                .typed(scenario_tester_proxy::ScenarioTesterProxy)
                .init(5u32)
                .code(MxscExpr("output/scenario-tester.mxsc.json"))
                .with_result(WithNewAddress::new(|new_address| {
                    assert_eq!(new_address.to_address(), st_contract.to_address());
                }))
        })
        .chain_query(|tx| {
            tx.to(ScExpr("adder"))
                .typed(scenario_tester_proxy::ScenarioTesterProxy)
                .sum()
                .with_result(WithResultConv::new(|value: BigUint| {
                    assert_eq!(value, BigUint::from(5u32));
                }))
        })
        .chain_call(|tx| {
            tx.from(AddressExpr("owner"))
                .to(ScExpr("adder"))
                .typed(scenario_tester_proxy::ScenarioTesterProxy)
                .add(3u32)
                .with_result(WithRawTxResponse(|response| {
                    assert!(response.tx_error.is_success());
                }))
        })
        .check_state_step(
            CheckStateStep::new()
                .put_account(owner_address, CheckAccount::new())
                .put_account(
                    &st_contract,
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        )
        .write_scenario_trace("trace2.scen.json");
}
