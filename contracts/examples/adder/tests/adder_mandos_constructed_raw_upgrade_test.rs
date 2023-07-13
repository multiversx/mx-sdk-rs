use multiversx_sc_scenario::{scenario_model::*, *};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract("file:output/adder.wasm", adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_mandos_constructed_raw() {
    let mut world = world();
    let ic = world.interpreter_context();
    world
        .set_state_step(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:adder"),
        )
        .sc_deploy_step(
            ScDeployStep::new()
                .from("address:owner")
                .contract_code("file:output/adder.wasm", &ic)
                .argument("5")
                .gas_limit("5,000,000")
                .expect(TxExpect::ok().no_result()),
        )
        .sc_call_step(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:adder")
                .function("upgradeContract")
                .argument("file:output/adder.wasm") // code 
                .argument("0x0502")                 // codeMetadata
                .argument("8")                      // contract argument
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
