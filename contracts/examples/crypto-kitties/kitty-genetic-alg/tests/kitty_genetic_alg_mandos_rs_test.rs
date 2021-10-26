use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/kitty-genetic-alg.wasm",
        Box::new(|context| Box::new(kitty_genetic_alg::contract_obj(context))),
    );
    blockchain
}

#[test]
fn generate_kitty_genes_rs() {
    elrond_wasm_debug::mandos_rs("mandos/generate-kitty-genes.scen.json", contract_map());
}

#[test]
fn init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/init.scen.json", contract_map());
}
