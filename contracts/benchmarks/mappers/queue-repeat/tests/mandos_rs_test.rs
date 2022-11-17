use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/queue-repeat");

    blockchain.register_contract_builder(
        "file:output/queue-repeat.wasm",
        queue_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn queue_repeat_struct_rs() {
    elrond_wasm_debug::mandos_rs("mandos/queue_repeat_struct.scen.json", world());
}

#[test]
fn queue_repeat_rs() {
    elrond_wasm_debug::mandos_rs("mandos/queue_repeat.scen.json", world());
}
