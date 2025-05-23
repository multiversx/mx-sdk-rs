use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain =
        ScenarioWorld::new().executor_config(scenario::run_vm::ExecutorConfig::WasmerProd);

    blockchain.set_current_dir_from_workspace("contracts/examples/factorial");
    blockchain.register_contract(
        "mxsc:output/factorial.mxsc.json",
        factorial::ContractBuilder,
    );
    blockchain
}

#[test]
#[cfg_attr(not(feature = "compiled-sc-tests"), ignore)]
fn factorial_wasmer_rs() {
    world().run("scenarios/factorial.scen.json");
}
