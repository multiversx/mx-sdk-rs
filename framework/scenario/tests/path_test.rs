use multiversx_sc_scenario::*;

fn world() -> BlockchainMock {
    BlockchainMock::new()
}

#[test]
fn local_path_test() {
    multiversx_sc_scenario::scenario_rs("tests/scenarios-self/path_test.scen.json", world());
}

#[test]
fn nested_path_test() {
    multiversx_sc_scenario::scenario_rs(
        "tests/scenarios-self/external_steps/external_path_test.scen.json",
        world(),
    );
}
