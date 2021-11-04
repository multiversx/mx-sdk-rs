use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/queue-repeat");

    blockchain.register_contract(
        "file:output/queue-repeat.wasm",
        Box::new(|context| Box::new(queue_repeat::contract_obj(context))),
    );
    blockchain
}

#[test]
fn queue_repeat_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/queue_repeat.scen.json", world());
}
