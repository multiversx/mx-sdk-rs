use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract_builder(
        "file:output/send-tx-repeat.wasm",
        send_tx_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn send_tx_repeat_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/send_tx_repeat.scen.json", world());
}
