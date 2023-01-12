use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/panic-message-features");

    blockchain.register_partial_contract::<panic_message_features::AbiProvider, _>(
        "file:output/panic-message-features.wasm",
        panic_message_features::ContractBuilder,
        "panic-message-features",
    );

    blockchain
}

#[test]
fn panic_message_rs() {
    multiversx_sc_scenario::run_rs("scenarios/panic-message.scen.json", world());
}
