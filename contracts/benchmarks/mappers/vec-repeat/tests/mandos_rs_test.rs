use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/vec-repeat");

    blockchain.register_contract(
        "file:output/vec-repeat.wasm",
        Box::new(|context| Box::new(vec_repeat::contract_obj(context))),
    );
    blockchain
}

#[test]
fn vec_repeat_struct_rs() {
    elrond_wasm_debug::mandos_rs("mandos/vec_repeat_struct.scen.json", world());
}

#[test]
fn vec_repeat_rs() {
    elrond_wasm_debug::mandos_rs("mandos/vec_repeat.scen.json", world());
}
