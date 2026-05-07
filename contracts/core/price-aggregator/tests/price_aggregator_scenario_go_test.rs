use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
#[ignore = "gas refund issue, fix mandos-go"]
fn stress_submit_test_go() {
    world().run("scenarios/stress_submit_test.scen.json");
}

#[test]
fn stress_submit_with_gas_schedule_test_go() {
    world().run("scenarios/stress_submit_with_gas_schedule_test.scen.json");
}
