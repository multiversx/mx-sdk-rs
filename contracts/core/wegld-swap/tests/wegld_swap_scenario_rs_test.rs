use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/core/wegld-swap");

    blockchain.register_contract(
        "file:output/multiversx-wegld-swap-sc.wasm",
        multiversx_wegld_swap_sc::ContractBuilder,
    );
    blockchain
}

#[test]
fn unwrap_egld_rs() {
    multiversx_sc_scenario::run_rs("scenarios/unwrap_egld.scen.json", world());
}

#[test]
fn wrap_egld_rs() {
    multiversx_sc_scenario::run_rs("scenarios/wrap_egld.scen.json", world());
}
