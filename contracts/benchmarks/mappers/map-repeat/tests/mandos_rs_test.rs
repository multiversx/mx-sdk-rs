use elrond_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/map-repeat");

    blockchain.register_contract(
        "file:output/map-repeat.wasm",
        Box::new(|context| Box::new(map_repeat::contract_obj(context))),
    );
    blockchain
}

#[test]
fn map_repeat_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/map_repeat.scen.json", contract_map());
}
