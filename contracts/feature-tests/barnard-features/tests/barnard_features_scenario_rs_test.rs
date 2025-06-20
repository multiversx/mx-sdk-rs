use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/barnard-features");

    blockchain.register_contract(
        "mxsc:output/barnard-features.mxsc.json",
        barnard_features::ContractBuilder,
    );

    blockchain
}

#[test]
fn block_info_ms_rs() {
    world().run("scenarios/block_info_ms.scen.json");
}

#[test]
#[ignore = "not yet supported"]
fn code_hash_rs() {
    world().run("scenarios/code_hash.scen.json");
}
