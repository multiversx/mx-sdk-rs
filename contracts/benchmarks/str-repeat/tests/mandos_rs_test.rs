use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain
        .register_contract_builder("file:output/str-repeat.wasm", str_repeat::ContractBuilder);
    blockchain
}

#[test]
fn test_str_repeat_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/str_repeat.scen.json", world());
}
