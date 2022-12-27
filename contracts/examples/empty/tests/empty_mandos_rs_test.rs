use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract("file:output/empty.wasm", empty::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    mx_sc_debug::mandos_rs("scenarios/empty.scen.json", world());
}
