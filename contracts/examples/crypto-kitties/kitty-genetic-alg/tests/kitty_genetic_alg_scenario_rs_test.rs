use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "file:output/kitty-genetic-alg.wasm",
        kitty_genetic_alg::ContractBuilder,
    );
    blockchain
}

#[test]
fn generate_kitty_genes_rs() {
    world().run("scenarios/generate-kitty-genes.scen.json");
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}
