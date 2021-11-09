use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/linked-list-repeat");

    blockchain.register_contract(
        "file:output/linked-list-repeat.wasm",
        Box::new(|context| Box::new(linked_list_repeat::contract_obj(context))),
    );
    blockchain
}

#[test]
fn linked_list_repeat_struct_rs() {
    elrond_wasm_debug::mandos_rs("mandos/linked_list_repeat_struct.scen.json", world());
}

#[test]
fn linked_list_repeat_rs() {
    elrond_wasm_debug::mandos_rs("mandos/linked_list_repeat.scen.json", world());
}
