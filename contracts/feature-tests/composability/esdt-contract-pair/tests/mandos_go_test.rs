#[test]
fn init_go() {
    elrond_wasm_debug::mandos_go("mandos/init.scen.json");
}

// #[test]
// fn reject_transfer_go() {
// 	elrond_wasm_debug::mandos_go("mandos/reject_transfer.scen.json");
// }

#[test]
fn simple_transfer_full_go() {
    elrond_wasm_debug::mandos_go("mandos/simple_transfer_full.scen.json");
}

#[test]
fn simple_transfer_full_wrong_token_go() {
    elrond_wasm_debug::mandos_go("mandos/simple_transfer_full_wrong_token.scen.json");
}

#[test]
fn simple_transfer_half_go() {
    elrond_wasm_debug::mandos_go("mandos/simple_transfer_half.scen.json");
}
