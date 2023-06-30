use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/big-float-features");

    blockchain.register_contract(
        "file:output/big-float-features.wasm",
        big_float_features::ContractBuilder,
    );

    blockchain
}

#[test]
fn big_float_new_from_big_int_rs() {
    world().run("scenarios/big_float_new_from_big_int.scen.json");
}

#[test]
fn big_float_new_from_big_uint_rs() {
    world().run("scenarios/big_float_new_from_big_uint.scen.json");
}

#[test]
fn big_float_new_from_frac_rs() {
    world().run("scenarios/big_float_new_from_frac.scen.json");
}

#[test]
fn big_float_new_from_int_rs() {
    world().run("scenarios/big_float_new_from_int.scen.json");
}

#[test]
#[ignore]
fn big_float_new_from_managed_buffer_rs() {
    world().run("scenarios/big_float_new_from_managed_buffer.scen.json");
}

#[test]
fn big_float_new_from_parts_rs() {
    world().run("scenarios/big_float_new_from_parts.scen.json");
}

#[test]
fn big_float_new_from_sci_rs() {
    world().run("scenarios/big_float_new_from_sci.scen.json");
}

#[test]
#[ignore]
fn big_float_operator_checks_rs() {
    world().run("scenarios/big_float_operator_checks.scen.json");
}

#[test]
fn big_float_operators_rs() {
    world().run("scenarios/big_float_operators.scen.json");
}
