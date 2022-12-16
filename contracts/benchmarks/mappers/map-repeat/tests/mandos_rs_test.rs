use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/map-repeat");

    blockchain.register_contract("file:output/map-repeat.wasm", map_repeat::ContractBuilder);
    blockchain
}

#[test]
fn map_repeat_struct_rs() {
    mx_sc_debug::mandos_rs("mandos/map_repeat_struct.scen.json", world());
}

#[test]
fn map_repeat_rs() {
    mx_sc_debug::mandos_rs("mandos/map_repeat.scen.json", world());
}
