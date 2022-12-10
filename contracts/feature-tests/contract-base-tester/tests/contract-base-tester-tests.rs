use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract("file:output/adder.wasm", contract_base_tester::ContractBuilder);
    blockchain
}

#[test]
fn adder_rs() {
    elrond_wasm_debug::mandos_rs("mandos/adder.scen.json", world());
}
