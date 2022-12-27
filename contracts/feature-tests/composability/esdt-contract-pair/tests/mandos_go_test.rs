#[test]
fn init_go() {
    mx_sc_debug::mandos_go("scenarios/init.scen.json");
}

// #[test]
// fn reject_transfer_go() {
// 	mx_sc_debug::mandos_go("scenarios/reject_transfer.scen.json");
// }

#[test]
fn simple_transfer_full_go() {
    mx_sc_debug::mandos_go("scenarios/simple_transfer_full.scen.json");
}

#[test]
fn simple_transfer_full_wrong_token_go() {
    mx_sc_debug::mandos_go("scenarios/simple_transfer_full_wrong_token.scen.json");
}

#[test]
fn simple_transfer_half_go() {
    mx_sc_debug::mandos_go("scenarios/simple_transfer_half.scen.json");
}
