#[test]
fn generate_kitty_genes_go() {
    elrond_wasm_debug::mandos_go("mandos/generate-kitty-genes.scen.json");
}

#[test]
fn init_go() {
    elrond_wasm_debug::mandos_go("mandos/init.scen.json");
}
