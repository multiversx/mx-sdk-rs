use adder::*;
use multiversx_sc_scenario::{scenario_model::*, *};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract("file:output/adder.wasm", adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_whitebox() {
    DebugApi::dummy();
    let adder_whitebox = WhiteboxContract::new("sc:adder", adder::contract_obj);

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
        .whitebox_query(&adder_whitebox, |sc| {
            let sum_value = sc.sum();
            assert_eq!(sum_value.get(), 5u32);
        })
        .whitebox_call(
            &adder_whitebox,
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
