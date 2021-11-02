use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/send-tx-repeat.wasm",
        Box::new(|context| Box::new(send_tx_repeat::contract_obj(context))),
    );
    blockchain
}

#[test]
fn test_send_tx_repeat_without_data_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/send_tx_repeat_without_data.scen.json", world());
}

#[test]
fn test_send_tx_repeat_with_data_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/send_tx_repeat_with_data.scen.json", world());
}
