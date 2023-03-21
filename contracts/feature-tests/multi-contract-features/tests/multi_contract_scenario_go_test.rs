#[test]
fn mcf_example_feature_go() {
    multiversx_sc_scenario::run_go("scenarios/mcf-example-feature.scen.json");
}

#[test]
fn mcf_external_get_go() {
    multiversx_sc_scenario::run_go("scenarios/mcf-external-get.scen.json");
}

#[test]
fn mcf_external_pure_go() {
    multiversx_sc_scenario::run_go("scenarios/mcf-external-pure.scen.json");
}
