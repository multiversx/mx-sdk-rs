use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/formatted-message-features");

    blockchain.register_contract(
        "file:output/formatted-message-features.wasm",
        formatted_message_features::ContractBuilder,
    );

    blockchain
}

#[test]
fn managed_error_message_rs() {
    multiversx_sc_scenario::run_rs("scenarios/managed_error_message.scen.json", world());
}

#[test]
fn sc_format_rs() {
    multiversx_sc_scenario::run_rs("scenarios/sc_format.scen.json", world());
}
