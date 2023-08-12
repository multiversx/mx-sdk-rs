use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn generate_kitty_genes_go() {
    world().run("scenarios/generate-kitty-genes.scen.json");
}

#[test]
fn init_go() {
    world().run("scenarios/init.scen.json");
}
