use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/map-repeat");

    blockchain
        .register_contract_builder("file:output/map-repeat.wasm", map_repeat::ContractBuilder);
    blockchain
}

#[test]
fn map_repeat_struct_rs() {
    elrond_wasm_debug::mandos_rs("mandos/map_repeat_struct.scen.json", world());
}

#[test]
fn map_repeat_rs() {
    elrond_wasm_debug::mandos_rs("mandos/map_repeat.scen.json", world());
}
