use multiversx_sc::types::{BigFloat, BigUint};
use multiversx_sc_scenario::{api::StaticApi, *};

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
fn big_float_overflow_test_rs() {
    let exp = 1_080i32;

    let first = BigFloat::<StaticApi>::from_sci(1_005, -3)
        .pow(exp)
        .to_fixed_point(&(100_000_000_000_000_000i64.into()))
        .into_big_uint();

    let second = BigFloat::<StaticApi>::from_sci(1_005, -3)
        .pow(exp)
        .to_fixed_point(&(10_000_000_000_000_000i64.into()))
        .into_big_uint();

    let third_float = BigFloat::<StaticApi>::from_sci(1_005, -3)
        .pow(exp)
        .to_managed_decimal(17usize);
    let third = third_float.into_raw_units();

    let forth_float = BigFloat::<StaticApi>::from_sci(1_005, -3)
        .pow(exp)
        .to_managed_decimal(16usize);
    let forth = forth_float.into_raw_units();

    assert_eq!(
        first.unwrap_or_sc_panic("unwrap failed"),
        /* overflow */
        BigUint::from(9223372036854775807u64)
    );

    assert_eq!(
        second.unwrap_or_sc_panic("unwrap failed"),
        BigUint::from(2184473079534488064u64)
    );

    assert_eq!(
        third,
        /* overflow */
        &BigUint::from(9223372036854775807u64)
    );

    assert_eq!(forth, &BigUint::from(2184473079534488064u64));
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
