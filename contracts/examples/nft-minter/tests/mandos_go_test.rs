#[test]
fn init_go() {
    mx_sc_debug::mandos_go("mandos/init.scen.json");
}

#[test]
fn create_nft_go() {
    mx_sc_debug::mandos_go("mandos/create_nft.scen.json");
}

#[test]
fn buy_nft_go() {
    mx_sc_debug::mandos_go("mandos/buy_nft.scen.json");
}
