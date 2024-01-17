use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/basic-features");

    blockchain.register_contract(
        "file:output/basic-features.wasm",
        basic_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn basic_features_new_address_rs_test() {
    world().run("scenarios/new_address.scen.json");
}
