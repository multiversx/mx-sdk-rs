use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn price_aggregator_stress_submit_go() {
    world().run("scenarios/stress_submit_with_test.scen.json");
}
