use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn send_tx_repeat_go() {
    world().run("scenarios/send_tx_repeat.scen.json");
}
