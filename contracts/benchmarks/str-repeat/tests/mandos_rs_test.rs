use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract("file:output/str-repeat.wasm", str_repeat::ContractBuilder);
    blockchain
}

#[test]
fn test_str_repeat_mandos_rs() {
    mx_sc_debug::scenario_rs("scenarios/str_repeat.scen.json", world());
}
