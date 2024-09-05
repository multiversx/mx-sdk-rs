use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "mxsc:output/esdt-system-sc-mock.mxsc.json",
        esdt_system_sc_mock::ContractBuilder,
    );
    blockchain
}

#[test]
fn esdt_system_sc_rs() {
    world().run("scenarios/esdt_system_sc.scen.json");
}
