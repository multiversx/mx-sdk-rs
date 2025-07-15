use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain
        .set_current_dir_from_workspace("contracts/examples/crypto-kitties/kitty-genetic-alg");
    blockchain.register_contract(
        "mxsc:output/kitty-genetic-alg.mxsc.json",
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
