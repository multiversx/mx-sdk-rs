use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract_builder(
        "file:output/kitty-genetic-alg.wasm",
        kitty_genetic_alg::contract_builder,
    );
    blockchain
}

#[test]
fn generate_kitty_genes_rs() {
    elrond_wasm_debug::mandos_rs("mandos/generate-kitty-genes.scen.json", world());
}

#[test]
fn init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/init.scen.json", world());
}
