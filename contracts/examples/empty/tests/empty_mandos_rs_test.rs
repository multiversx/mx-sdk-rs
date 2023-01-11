use multiversx_sc_scenario::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract("file:output/empty.wasm", empty::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    multiversx_sc_scenario::scenario_rs("scenarios/empty.scen.json", world());
}
