use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/kitty-genetic-alg.wasm",
        kitty_genetic_alg::ContractBuilder,
    );
    blockchain
}

#[test]
fn generate_kitty_genes_rs() {
    mx_sc_debug::mandos_rs("mandos/generate-kitty-genes.scen.json", world());
}

#[test]
fn init_rs() {
    mx_sc_debug::mandos_rs("mandos/init.scen.json", world());
}
