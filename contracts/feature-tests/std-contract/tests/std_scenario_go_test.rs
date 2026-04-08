use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn std_deploy_go() {
    world().run("scenarios/std-deploy.scen.json");
}
