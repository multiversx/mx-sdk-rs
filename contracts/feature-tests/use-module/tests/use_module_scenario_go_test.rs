use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn use_module_claim_developer_rewards_go() {
    world().run("scenarios/use_module_claim_developer_rewards.scen.json");
}

#[test]
fn use_module_dns_register_go() {
    world().run("scenarios/use_module_dns_register.scen.json");
}

#[test]
fn use_module_features_go() {
    world().run("scenarios/use_module_features.scen.json");
}

#[test]
fn use_module_internal_go() {
    world().run("scenarios/use_module_internal.scen.json");
}

#[test]
fn use_module_no_endpoint_go() {
    world().run("scenarios/use_module_no_endpoint.scen.json");
}

#[test]
fn use_module_ongoing_operation_example_go() {
    world().run("scenarios/use_module_ongoing_operation_example.scen.json");
}

#[test]
fn use_module_only_admin_go() {
    world().run("scenarios/use_module_only_admin.scen.json");
}

#[test]
fn use_module_only_owner_go() {
    world().run("scenarios/use_module_only_owner.scen.json");
}

#[test]
fn use_module_pause_go() {
    world().run("scenarios/use_module_pause.scen.json");
}
