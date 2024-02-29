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
    world().run("scenarios/managed_error_message.scen.json");
}

#[test]
fn sc_format_rs() {
    world().run("scenarios/sc_format.scen.json");
}
