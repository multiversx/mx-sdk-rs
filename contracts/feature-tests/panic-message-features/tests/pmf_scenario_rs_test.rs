use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_partial_contract::<panic_message_features::AbiProvider, _>(
        "mxsc:output/panic-message-features.mxsc.json",
        panic_message_features::ContractBuilder,
        "panic-message-features",
    );

    blockchain
}

#[ignore = "`internalVMErrors` logs not implemented"]
#[test]
fn panic_after_log_rs() {
    world().run("scenarios/panic-after-log.scen.json");
}

#[ignore = "PanicInfo currently not available, TODO: use std::panic::set_hook"]
#[test]
fn panic_message_rs() {
    world().run("scenarios/panic-message.scen.json");
}
