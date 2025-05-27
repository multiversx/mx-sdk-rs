use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn mmap_get_go() {
    world().run("scenarios/mmap_get.scen.json");
}

#[test]
fn mmap_remove_go() {
    world().run("scenarios/mmap_remove.scen.json");
}
