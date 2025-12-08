use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::Experimental);

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/tiny");
    blockchain
}

#[test]
fn tiny_rs() {
    world().run("scenarios/tiny.scen.json");
}
