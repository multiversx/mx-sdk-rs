#[test]
fn test_go() {
    multiversx_sc_scenario::run_go("scenarios/test.scen.json");
}

#[test]
fn test_esdt_generation_go() {
    multiversx_sc_scenario::run_go("scenarios/test_esdt_generation.scen.json");
}

#[test]
fn test_multiple_sc_go() {
    multiversx_sc_scenario::run_go("scenarios/test_multiple_sc.scen.json");
}

#[test]
#[ignore = "not supported"]
fn trace_deploy_go() {
    multiversx_sc_scenario::run_go("scenarios/trace-deploy.scen.json");
}
