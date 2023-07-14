use multiversx_sc_meta::cmd::standalone::scen_test_gen::{
    format_test_fn_go, process_code, DEFAULT_SETUP_GO,
};

const GO_TEST_1: &str = r#"use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn test_1_go() {
    world().run("scenarios/test1.scen.json");
}
"#;

const GO_TEST_0_1_2: &str = r#"use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn test_0_go() {
    world().run("scenarios/test0.scen.json");
}

#[test]
fn test_1_go() {
    world().run("scenarios/test1.scen.json");
}

#[test]
fn test_2_go() {
    world().run("scenarios/test2.scen.json");
}
"#;

fn check_transformation(
    input: &str,
    scenario_names_list: impl IntoIterator<Item = &'static str>,
    expected_out: &str,
) {
    let scenario_names = scenario_names_list
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let new_code = process_code(input, &scenario_names, format_test_fn_go, DEFAULT_SETUP_GO);
    assert_eq!(new_code.as_str(), expected_out);
}

#[test]
fn process_code_go_1() {
    check_transformation(GO_TEST_1, ["test1"], GO_TEST_1);
}

#[test]
fn process_code_go_2() {
    check_transformation(GO_TEST_1, ["test0", "test1", "test2"], GO_TEST_0_1_2);
}

#[test]
fn process_new_code_go() {
    check_transformation("", ["test1"], GO_TEST_1);
}
