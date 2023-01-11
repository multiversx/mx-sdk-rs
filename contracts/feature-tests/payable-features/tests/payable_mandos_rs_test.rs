use mx_sc_scenario::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/payable-features");
    blockchain.register_contract(
        "file:output/payable-features.wasm",
        payable_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn call_value_check_rs() {
    mx_sc_scenario::scenario_rs("scenarios/call-value-check.scen.json", world());
}

#[test]
fn payable_multiple_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_multiple.scen.json", world());
}

#[test]
fn payable_any_1_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_any_1.scen.json", world());
}

#[test]
fn payable_any_2_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_any_2.scen.json", world());
}

#[test]
fn payable_any_3_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_any_3.scen.json", world());
}

#[test]
fn payable_any_4_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_any_4.scen.json", world());
}

#[test]
fn payable_egld_1_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_egld_1.scen.json", world());
}

#[test]
fn payable_egld_2_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_egld_2.scen.json", world());
}

#[test]
fn payable_egld_3_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_egld_3.scen.json", world());
}

#[test]
fn payable_egld_4_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_egld_4.scen.json", world());
}

#[test]
fn payable_multi_array_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_multi_array.scen.json", world());
}

#[test]
fn payable_token_1_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_token_1.scen.json", world());
}

#[test]
fn payable_token_2_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_token_2.scen.json", world());
}

#[test]
fn payable_token_3_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_token_3.scen.json", world());
}

#[test]
fn payable_token_4_rs() {
    mx_sc_scenario::scenario_rs("scenarios/payable_token_4.scen.json", world());
}
