use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/esdt-system-sc-mock");

    blockchain.register_contract(
        "file:output/esdt-system-sc-mock.wasm",
        esdt_system_sc_mock::ContractBuilder,
    );
    blockchain
}

#[test]
fn esdt_system_sc_rs() {
    world().run("scenarios/esdt_system_sc.scen.json");
}
