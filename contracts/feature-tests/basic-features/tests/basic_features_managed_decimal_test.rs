use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/basic-features");
    blockchain.register_contract(
        "mxsc:output/basic-features.mxsc.json",
        basic_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn managed_decimal_test() {
    world().run("scenarios/managed_decimal.scen.json");
}

#[test]
fn managed_decimal_logarithm_test() {
    world().run("scenarios/managed_decimal_logarithm.scen.json");
}
