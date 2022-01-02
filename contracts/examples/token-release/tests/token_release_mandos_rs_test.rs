use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/token-release");

    blockchain.register_contract_builder(
        "file:output/token-release.wasm",
        token_release::contract_builder,
    );
    blockchain
}

#[test]
fn token_release_init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/test-init.scen.json", world());
}
