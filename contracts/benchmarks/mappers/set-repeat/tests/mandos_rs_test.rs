use elrond_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/set-repeat");

    blockchain.register_contract(
        "file:output/set-repeat.wasm",
        Box::new(|context| Box::new(set_repeat::contract_obj(context))),
    );
    blockchain
}

#[test]
fn set_repeat_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/set_repeat.scen.json", contract_map());
}
