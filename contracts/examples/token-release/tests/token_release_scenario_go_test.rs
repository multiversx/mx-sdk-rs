use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn test_add_group_go() {
    world().run("scenarios/test-add-group.scen.json");
}

#[test]
fn test_add_user_go() {
    world().run("scenarios/test-add-user.scen.json");
}

#[test]
fn test_change_user_go() {
    world().run("scenarios/test-change-user.scen.json");
}

#[test]
fn test_claim_go() {
    world().run("scenarios/test-claim.scen.json");
}

#[test]
fn test_end_setup_go() {
    world().run("scenarios/test-end-setup.scen.json");
}

#[test]
fn test_init_go() {
    world().run("scenarios/test-init.scen.json");
}
