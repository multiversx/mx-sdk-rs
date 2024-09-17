#![allow(deprecated)]

use multiversx_sc_scenario::imports::*;
use scenario_tester::*;

const ADDER_PATH_EXPR: &str = "mxsc:output/scenario-tester.mxsc.json";

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
    let st_whitebox = WhiteboxContract::new("sc:adder", scenario_tester::contract_obj);
    let st_code = world.code_expression(ADDER_PATH_EXPR);

    world
        .set_state_step(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:adder"),
        )
        .whitebox_deploy(
            &st_whitebox,
            ScDeployStep::new().from("address:owner").code(st_code),
            |sc| {
                sc.init(5u32.into());
            },
        )
        .whitebox_query(&st_whitebox, |sc| {
            let sum_value = sc.sum();
            assert_eq!(sum_value.get(), 5u32);
        })
        .whitebox_call(
            &st_whitebox,
            ScCallStep::new().from("address:owner"),
            |sc| sc.add(3u32.into()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account("address:owner", CheckAccount::new())
                .put_account(
                    "sc:adder",
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        );
}
