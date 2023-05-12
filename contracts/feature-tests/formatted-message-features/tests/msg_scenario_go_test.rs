use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn managed_error_message_go() {
    world().run("scenarios/managed_error_message.scen.json");
}

#[test]
fn sc_format_go() {
    world().run("scenarios/sc_format.scen.json");
}
