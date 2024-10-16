use multiversx_sc_scenario::imports::*;

const SCENARIO_TESTER_PATH_EXPR: &str = "mxsc:output/scenario-tester.mxsc.json";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(SCENARIO_TESTER_PATH_EXPR, scenario_tester::ContractBuilder);
    blockchain
}

#[test]
fn scenario_tester_blackbox_raw() {
    let mut world = world();
    let scenario_tester_code = world.code_expression(SCENARIO_TESTER_PATH_EXPR);

    world
        .set_state_step(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:scenario-tester"),
        )
        .sc_deploy(
            ScDeployStep::new()
                .from("address:owner")
                .code(scenario_tester_code)
                .argument("5")
                .expect(TxExpect::ok().result("str:init-result")),
        )
        .sc_query(
            ScQueryStep::new()
                .to("sc:scenario-tester")
                .function("getSum")
                .expect(TxExpect::ok().result("5")),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:scenario-tester")
                .function("add")
                .argument("3")
                .expect(TxExpect::ok().no_result()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account("address:owner", CheckAccount::new())
                .put_account(
                    "sc:scenario-tester",
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        );
}
