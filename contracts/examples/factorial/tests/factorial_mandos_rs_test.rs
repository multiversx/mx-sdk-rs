use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/factorial");

    blockchain.register_contract_builder("file:output/factorial.wasm", factorial::ContractBuilder);
    blockchain
}

#[test]
fn factorial_rs() {
    elrond_wasm_debug::mandos_rs("mandos/factorial.scen.json", world());
}
