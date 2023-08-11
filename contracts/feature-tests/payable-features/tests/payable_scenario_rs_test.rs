use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/payable-features");
    blockchain.register_contract(
        "file:output/payable-features.wasm",
        payable_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn call_value_check_rs() {
    world().run("scenarios/call-value-check.scen.json");
}

#[test]
fn payable_any_1_rs() {
    world().run("scenarios/payable_any_1.scen.json");
}

#[test]
fn payable_any_2_rs() {
    world().run("scenarios/payable_any_2.scen.json");
}

#[test]
fn payable_any_3_rs() {
    world().run("scenarios/payable_any_3.scen.json");
}

#[test]
fn payable_any_4_rs() {
    world().run("scenarios/payable_any_4.scen.json");
}

#[test]
fn payable_egld_1_rs() {
    world().run("scenarios/payable_egld_1.scen.json");
}

#[test]
fn payable_egld_2_rs() {
    world().run("scenarios/payable_egld_2.scen.json");
}

#[test]
fn payable_egld_3_rs() {
    world().run("scenarios/payable_egld_3.scen.json");
}

#[test]
fn payable_egld_4_rs() {
    world().run("scenarios/payable_egld_4.scen.json");
}

#[test]
fn payable_multi_array_rs() {
    world().run("scenarios/payable_multi_array.scen.json");
}

#[test]
fn payable_multiple_rs() {
    world().run("scenarios/payable_multiple.scen.json");
}

#[test]
fn payable_token_1_rs() {
    world().run("scenarios/payable_token_1.scen.json");
}

#[test]
fn payable_token_2_rs() {
    world().run("scenarios/payable_token_2.scen.json");
}

#[test]
fn payable_token_3_rs() {
    world().run("scenarios/payable_token_3.scen.json");
}

#[test]
fn payable_token_4_rs() {
    world().run("scenarios/payable_token_4.scen.json");
}
