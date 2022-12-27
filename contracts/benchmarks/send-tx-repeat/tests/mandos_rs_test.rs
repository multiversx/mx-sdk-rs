use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/send-tx-repeat.wasm",
        send_tx_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn send_tx_repeat_mandos_rs() {
    mx_sc_debug::scenario_rs("scenarios/send_tx_repeat.scen.json", world());
}
