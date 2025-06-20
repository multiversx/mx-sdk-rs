use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
#[ignore = "TODO: awaiting for Barnard release"]
fn block_info_ms_go() {
    world().run("scenarios/block_info_ms.scen.json");
}

#[test]
#[ignore = "TODO: awaiting for Barnard release"]
fn code_hash_go() {
    world().run("scenarios/code_hash.scen.json");
}
