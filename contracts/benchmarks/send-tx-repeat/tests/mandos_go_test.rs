#[test]
fn send_tx_repeat_without_data_go() {
    elrond_wasm_debug::mandos_go("mandos/send_tx_repeat_without_data.scen.json");
}

#[test]
fn send_tx_repeat_with_data_go() {
    elrond_wasm_debug::mandos_go("mandos/send_tx_repeat_with_data.scen.json");
}
