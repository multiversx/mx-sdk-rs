use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn call_value_check_go() {
    world().run("scenarios/call-value-check.scen.json");
}

#[test]
fn payable_any_1_go() {
    world().run("scenarios/payable_any_1.scen.json");
}

#[test]
fn payable_any_2_go() {
    world().run("scenarios/payable_any_2.scen.json");
}

#[test]
fn payable_any_3_go() {
    world().run("scenarios/payable_any_3.scen.json");
}

#[test]
fn payable_any_4_go() {
    world().run("scenarios/payable_any_4.scen.json");
}

#[test]
fn payable_egld_1_go() {
    world().run("scenarios/payable_egld_1.scen.json");
}

#[test]
fn payable_egld_2_go() {
    world().run("scenarios/payable_egld_2.scen.json");
}

#[test]
fn payable_egld_3_go() {
    world().run("scenarios/payable_egld_3.scen.json");
}

#[test]
fn payable_egld_4_go() {
    world().run("scenarios/payable_egld_4.scen.json");
}

#[test]
fn payable_multi_array_go() {
    world().run("scenarios/payable_multi_array.scen.json");
}

#[test]
fn payable_multiple_go() {
    world().run("scenarios/payable_multiple.scen.json");
}

#[test]
fn payable_token_1_go() {
    world().run("scenarios/payable_token_1.scen.json");
}

#[test]
fn payable_token_2_go() {
    world().run("scenarios/payable_token_2.scen.json");
}

#[test]
fn payable_token_3_go() {
    world().run("scenarios/payable_token_3.scen.json");
}

#[test]
fn payable_token_4_go() {
    world().run("scenarios/payable_token_4.scen.json");
}
