use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/vec-repeat");

    blockchain.register_contract("file:output/vec-repeat.wasm", vec_repeat::ContractBuilder);
    blockchain
}

#[test]
fn vec_repeat_struct_rs() {
    mx_sc_debug::mandos_rs("mandos/vec_repeat_struct.scen.json", world());
}

#[test]
fn vec_repeat_rs() {
    mx_sc_debug::mandos_rs("mandos/vec_repeat.scen.json", world());
}
