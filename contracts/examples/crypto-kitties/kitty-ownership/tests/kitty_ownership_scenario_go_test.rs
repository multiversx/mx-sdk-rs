use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn approve_siring_go() {
    world().run("scenarios/approve_siring.scen.json");
}

#[test]
fn breed_ok_go() {
    world().run("scenarios/breed_ok.scen.json");
}

#[test]
fn give_birth_go() {
    world().run("scenarios/give_birth.scen.json");
}

#[test]
fn init_go() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn query_go() {
    world().run("scenarios/query.scen.json");
}

#[test]
fn setup_accounts_go() {
    world().run("scenarios/setup_accounts.scen.json");
}
