use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::new()
}

#[test]
fn local_path_test() {
    multiversx_sc_scenario::run_rs("tests/scenarios-self/path_test.scen.json", world());
}

#[test]
fn nested_path_test() {
    multiversx_sc_scenario::run_rs(
        "tests/scenarios-self/external_steps/external_path_test.scen.json",
        world(),
    );
}
