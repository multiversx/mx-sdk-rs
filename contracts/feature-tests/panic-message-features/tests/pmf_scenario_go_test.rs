use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn panic_message_go() {
    world().run("scenarios/panic-message.scen.json");
}
