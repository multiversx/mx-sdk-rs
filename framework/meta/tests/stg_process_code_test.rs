use multiversx_sc_meta::scen_test_gen::{format_test_fn_go, process_code};

const STABLE_GO_1: &str = r#"#[test]
fn test_1_go() {
    multiversx_sc_scenario::run_go("scenarios/test1.scen.json");
}
"#;

const EXAMPLE_GO_OUT_2: &str = r#"#[test]
fn test_0_go() {
    multiversx_sc_scenario::run_go("scenarios/test0.scen.json");
}

#[test]
fn test_1_go() {
    multiversx_sc_scenario::run_go("scenarios/test1.scen.json");
}

#[test]
fn test_2_go() {
    multiversx_sc_scenario::run_go("scenarios/test2.scen.json");
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
    let new_code = process_code(input, &scenario_names, format_test_fn_go);
    assert_eq!(new_code.as_str(), expected_out);
}

#[test]
fn process_code_go_1() {
    check_transformation(STABLE_GO_1, ["test1"], STABLE_GO_1);
}

#[test]
fn process_code_go_2() {
    check_transformation(STABLE_GO_1, ["test0", "test1", "test2"], EXAMPLE_GO_OUT_2);
}
