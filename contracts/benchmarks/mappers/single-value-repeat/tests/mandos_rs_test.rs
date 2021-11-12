use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/single-value-repeat");

    blockchain.register_contract(
        "file:output/single-value-repeat.wasm",
        Box::new(|context| Box::new(single_value_repeat::contract_obj(context))),
    );
    blockchain
}

#[test]
fn single_value_repeat_struct_rs() {
    elrond_wasm_debug::mandos_rs("mandos/single_value_repeat_struct.scen.json", world());
}

#[test]
fn single_value_repeat_rs() {
    elrond_wasm_debug::mandos_rs("mandos/single_value_repeat.scen.json", world());
}
