use multiversx_sc_snippets::multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn deploy_go() {
    world().run("scenarios/forwarder_deploy_scenario.scen.json");
}

#[test]
#[ignore = "missing 'newTokenIdentifiers' syntax"]
fn builtin_func_go() {
    world().run("scenarios/forwarder_builtin_scenario.scen.json");
}

#[test]
#[ignore = "missing 'newTokenIdentifiers' syntax"]
fn change_to_dynamic_go() {
    world().run("scenarios/forwarder_change_to_dynamic_scenario.scen.json");
}

#[test]
#[ignore = "missing 'newTokenIdentifiers' syntax"]
fn update_token_go() {
    world().run("scenarios/forwarder_update_token_scenario.scen.json");
}

#[test]
#[ignore = "missing 'newTokenIdentifiers' syntax"]
fn modify_creator_go() {
    world().run("scenarios/forwarder_modify_creator_scenario.scen.json");
}
