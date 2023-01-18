use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/ping-pong-egld");

    blockchain.register_contract(
        "file:output/ping-pong-egld.wasm",
        ping_pong_egld::ContractBuilder,
    );
    blockchain
}

#[test]
fn ping_pong_call_get_user_addresses_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/ping-pong-call-get-user-addresses.scen.json",
        world(),
    );
}

#[test]
fn ping_pong_call_ping_rs() {
    multiversx_sc_scenario::run_rs("scenarios/ping-pong-call-ping.scen.json", world());
}

#[test]
fn ping_pong_call_ping_after_deadline_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/ping-pong-call-ping-after-deadline.scen.json",
        world(),
    );
}

#[test]
fn ping_pong_call_ping_before_activation_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/ping-pong-call-ping-before-activation.scen.json",
        world(),
    );
}

#[test]
fn ping_pong_call_ping_second_user_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/ping-pong-call-ping-second-user.scen.json",
        world(),
    );
}

#[test]
fn ping_pong_call_ping_twice_rs() {
    multiversx_sc_scenario::run_rs("scenarios/ping-pong-call-ping-twice.scen.json", world());
}

#[test]
fn ping_pong_call_ping_wrong_ammount_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/ping-pong-call-ping-wrong-ammount.scen.json",
        world(),
    );
}

#[test]
fn ping_pong_call_pong_rs() {
    multiversx_sc_scenario::run_rs("scenarios/ping-pong-call-pong.scen.json", world());
}

#[test]
fn ping_pong_call_pong_all_rs() {
    multiversx_sc_scenario::run_rs("scenarios/ping-pong-call-pong-all.scen.json", world());
}

#[test]
fn ping_pong_call_pong_all_after_pong_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/ping-pong-call-pong-all-after-pong.scen.json",
        world(),
    );
}

#[test]
fn ping_pong_call_pong_before_deadline_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/ping-pong-call-pong-before-deadline.scen.json",
        world(),
    );
}

#[test]
fn ping_pong_call_pong_twice_rs() {
    multiversx_sc_scenario::run_rs("scenarios/ping-pong-call-pong-twice.scen.json", world());
}

#[test]
fn ping_pong_call_pong_without_ping_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/ping-pong-call-pong-without-ping.scen.json",
        world(),
    );
}

#[test]
fn ping_pong_init_rs() {
    multiversx_sc_scenario::run_rs("scenarios/ping-pong-init.scen.json", world());
}
