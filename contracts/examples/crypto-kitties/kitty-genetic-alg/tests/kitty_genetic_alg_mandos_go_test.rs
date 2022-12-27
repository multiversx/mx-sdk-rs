#[test]
fn generate_kitty_genes_go() {
    mx_sc_debug::mandos_go("scenarios/generate-kitty-genes.scen.json");
}

#[test]
fn init_go() {
    mx_sc_debug::mandos_go("scenarios/init.scen.json");
}
