use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/str-repeat.wasm",
        Box::new(|context| Box::new(str_repeat::contract_obj(context))),
    );
    blockchain
}

#[test]
fn test_str_repeat_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/str_repeat.scen.json", world());
}
