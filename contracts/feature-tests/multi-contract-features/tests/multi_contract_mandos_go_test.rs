#[test]
fn external_pure_go() {
    mx_sc_debug::mandos_go("mandos/external-pure.scen.json");
}

#[test]
fn external_get_go() {
    mx_sc_debug::mandos_go("mandos/external-get.scen.json");
}
