use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/payable-features");
    blockchain.register_contract(
        "mxsc:output/payable-features.mxsc.json",
        payable_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn call_value_check_rs() {
    world().run("scenarios/call-value-check.scen.json");
}

#[test]
fn call_value_check_multi_egld_rs() {
    world().run("scenarios/call-value-check-multi-egld.scen.json");
}

#[test]
fn payable_all_transfers_1_rs() {
    world().run("scenarios/payable_all_transfers_1.scen.json");
}

#[test]
fn payable_all_transfers_2_rs() {
    world().run("scenarios/payable_all_transfers_2.scen.json");
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
fn payable_multi_array_egld_rs() {
    world().run("scenarios/payable_multi_array_egld.scen.json");
}

#[test]
fn payable_multiple_rs() {
    world().run("scenarios/payable_multiple.scen.json");
}

#[test]
fn payable_multiple_egld_rs() {
    world().run("scenarios/payable_multiple_egld.scen.json");
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

#[test]
fn payable_token_5_rs() {
    world().run("scenarios/payable_token_5.scen.json");
}
