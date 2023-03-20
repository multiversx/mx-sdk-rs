use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    todo!()
}

#[test]
#[ignore = "builtin SC not implemented"]
fn esdt_system_sc_rs() {
    multiversx_sc_scenario::run_rs("scenarios/esdt_system_sc.scen.json", world());
}
