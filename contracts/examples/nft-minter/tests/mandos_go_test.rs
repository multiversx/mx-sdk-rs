#[test]
fn init_go() {
    mx_sc_debug::mandos_go("scenarios/init.scen.json");
}

#[test]
fn create_nft_go() {
    mx_sc_debug::mandos_go("scenarios/create_nft.scen.json");
}

#[test]
fn buy_nft_go() {
    mx_sc_debug::mandos_go("scenarios/buy_nft.scen.json");
}
