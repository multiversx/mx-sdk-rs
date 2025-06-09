use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/examples/factorial");
    blockchain.register_contract(
        "mxsc:output/factorial.mxsc.json",
        factorial::ContractBuilder,
    );
    blockchain
}

#[test]
fn factorial_rs() {
    world().run("scenarios/factorial.scen.json");
}
