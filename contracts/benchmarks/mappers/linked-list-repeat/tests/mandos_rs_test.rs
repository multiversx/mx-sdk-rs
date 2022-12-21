use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/linked-list-repeat");

    blockchain.register_contract(
        "file:output/linked-list-repeat.wasm",
        linked_list_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn linked_list_repeat_struct_rs() {
    mx_sc_debug::mandos_rs("mandos/linked_list_repeat_struct.scen.json", world());
}

#[test]
fn linked_list_repeat_rs() {
    mx_sc_debug::mandos_rs("mandos/linked_list_repeat.scen.json", world());
}
