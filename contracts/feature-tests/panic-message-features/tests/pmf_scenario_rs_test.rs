use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/panic-message-features");
    blockchain.register_partial_contract::<panic_message_features::AbiProvider, _>(
        "mxsc:output/panic-message-features.mxsc.json",
        panic_message_features::ContractBuilder,
        "panic-message-features",
    );

    blockchain
}

#[test]
#[ignore = "`internalVMErrors` logs not implemented"]
fn panic_after_log_rs() {
    world().run("scenarios/panic-after-log.scen.json");
}

#[test]
#[ignore = "PanicInfo currently not available, TODO: use std::panic::set_hook"]
fn panic_message_rs() {
    world().run("scenarios/panic-message.scen.json");
}

#[test]
#[ignore = "PanicInfo currently not available, TODO: use std::panic::set_hook"]
fn panic_message_std_rs() {
    world().run("scenarios/panic-message-std.scen.json");
}

#[test]
#[should_panic]
fn should_panic_call_rs() {
    world().run("scenarios/should-panic-call.scen.json");
}

#[test]
#[should_panic]
fn should_panic_deploy_rs() {
    world().run("scenarios/should-panic-deploy.scen.json");
}

#[test]
#[should_panic]
fn should_panic_query_rs() {
    world().run("scenarios/should-panic-query.scen.json");
}
