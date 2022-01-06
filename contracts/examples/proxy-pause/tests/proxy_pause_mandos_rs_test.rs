use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/proxy-pause");

    blockchain.register_contract_builder(
        "file:output/proxy-pause.wasm",
        proxy_pause::contract_builder,
    );

    blockchain.register_contract_builder(
        "file:../../feature-tests/use-module/output/use-module.wasm",
        use_module::contract_builder,
    );
    blockchain
}

#[test]
fn pause_rs() {
    elrond_wasm_debug::mandos_rs("mandos/pause-and-unpause.scen.json", world());
}
