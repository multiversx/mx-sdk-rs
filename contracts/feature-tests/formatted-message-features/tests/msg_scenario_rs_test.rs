use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "mxsc:output/formatted-message-features.mxsc.json",
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
