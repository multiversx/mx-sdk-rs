use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn mcf_alt_init_go() {
    world().run("scenarios/mcf-alt-init.scen.json");
}

#[test]
fn mcf_example_feature_go() {
    world().run("scenarios/mcf-example-feature.scen.json");
}

#[test]
fn mcf_external_get_go() {
    world().run("scenarios/mcf-external-get.scen.json");
}

#[test]
fn mcf_external_pure_go() {
    world().run("scenarios/mcf-external-pure.scen.json");
}
