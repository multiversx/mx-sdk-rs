use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn mmap_get_go() {
    world().run("scenarios/mmap_get.scen.json");
}

#[test]
fn mmap_key_mutability_go() {
    world().run("scenarios/mmap_key_mutability.scen.json");
}

#[test]
fn mmap_mutable_input_go() {
    world().run("scenarios/mmap_mutable_input.scen.json");
}

#[test]
fn mmap_remove_go() {
    world().run("scenarios/mmap_remove.scen.json");
}
