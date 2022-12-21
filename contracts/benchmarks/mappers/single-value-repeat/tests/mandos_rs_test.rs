use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/single-value-repeat");

    blockchain.register_contract(
        "file:output/single-value-repeat.wasm",
        single_value_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn single_value_repeat_struct_rs() {
    mx_sc_debug::mandos_rs("mandos/single_value_repeat_struct.scen.json", world());
}

#[test]
fn single_value_repeat_rs() {
    mx_sc_debug::mandos_rs("mandos/single_value_repeat.scen.json", world());
}
