use multiversx_sc_scenario::{scenario_model::*, *};

const ADDER_PATH_EXPR: &str = "file:output/adder.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract(ADDER_PATH_EXPR, adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_blackbox_raw() {
    let mut world = world();
    let adder_code = world.code_expression(ADDER_PATH_EXPR);

    world
        .set_state_step(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:adder"),
        )
        .sc_deploy(
            ScDeployStep::new()
                .from("address:owner")
                .code(adder_code)
                .argument("5")
                .expect(TxExpect::ok().no_result()),
        )
        .sc_query(
            ScQueryStep::new()
                .to("sc:adder")
                .function("getSum")
                .expect(TxExpect::ok().result("5")),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:adder")
                .function("add")
                .argument("3")
                .expect(TxExpect::ok().no_result()),
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
