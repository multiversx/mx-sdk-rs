use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn multi_transfer_with_egld_test() {
    world().run("tests/scenarios-self/multi_transfer_with_egld.scen.json");
}
