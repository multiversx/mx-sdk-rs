#[test]
fn managed_error_message_go() {
    multiversx_sc_scenario::run_go("scenarios/managed_error_message.scen.json");
}

#[test]
fn sc_format_go() {
    multiversx_sc_scenario::run_go("scenarios/sc_format.scen.json");
}
