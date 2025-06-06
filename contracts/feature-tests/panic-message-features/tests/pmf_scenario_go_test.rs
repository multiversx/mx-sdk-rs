use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn panic_after_log_go() {
    world().run("scenarios/panic-after-log.scen.json");
}

#[test]
fn panic_message_go() {
    world().run("scenarios/panic-message.scen.json");
}

#[test]
#[ignore = "not testing panic in mx-scenario-go here"]
fn should_panic_call_go() {
    world().run("scenarios/should-panic-call.scen.json");
}

#[test]
#[ignore = "not testing panic in mx-scenario-go here"]
fn should_panic_deploy_go() {
    world().run("scenarios/should-panic-deploy.scen.json");
}

#[test]
#[ignore = "not testing panic in mx-scenario-go here"]
fn should_panic_query_go() {
    world().run("scenarios/should-panic-query.scen.json");
}
