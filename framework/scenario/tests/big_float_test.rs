use multiversx_sc::types::{BigFloat, BigInt, BigUint};
use multiversx_sc_scenario::api::StaticApi;

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
        .to_managed_decimal_signed(17usize);
    let third = third_float.into_raw_units();

    let forth_float = BigFloat::<StaticApi>::from_sci(1_005, -3)
        .pow(exp)
        .to_managed_decimal_signed(16usize);
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
        &BigInt::from(9223372036854775807i64)
    );

    assert_eq!(forth, &BigInt::from(2184473079534488064i64));
}

#[test]
fn big_float_ln_test_rs() {
    let x = BigFloat::<StaticApi>::from(23i64);
    let ln_x = x.ln().unwrap();
    assert_eq!(
        ln_x.to_managed_decimal_signed(9usize).to_string(),
        "3.135514648"
    );
    assert!(ln_x.is_close(
        &BigFloat::from_frac(3135514648, 1_000_000_000), // 3.135514648
        &BigFloat::from_frac(1, 1_000_000_000)
    ));

    let big = BigFloat::<StaticApi>::from(382747812i64);
    let ln_big = big.ln().unwrap();
    assert_eq!(
        ln_big.to_managed_decimal_signed(9usize).to_string(),
        "19.762913880"
    );
    assert!(ln_big.is_close(
        &BigFloat::from_frac(19762913880, 1_000_000_000), // 19.762913880
        &BigFloat::from_frac(1, 1_000_000_000)
    ));

    let biggest = BigFloat::<StaticApi>::from(999999999i64);
    let ln_biggest = biggest.ln().unwrap();
    assert_eq!(
        ln_biggest.to_managed_decimal_signed(9usize).to_string(),
        "20.723319778"
    );
    assert!(ln_biggest.is_close(
        &BigFloat::from_frac(20723319778, 1_000_000_000), // 20.723319778
        &BigFloat::from_frac(1, 1_000_000_000)
    ));

    let small = BigFloat::<StaticApi>::from_frac(3i64, 2i64);
    let ln_small = small.ln().unwrap();
    assert_eq!(
        ln_small.to_managed_decimal_signed(9usize).to_string(),
        "0.405448248"
    );
    assert!(ln_small.is_close(
        &BigFloat::from_frac(405448248, 1_000_000_000), // 0.405448248
        &BigFloat::from_frac(1, 1_000_000_000)
    ));

    let smallest = BigFloat::<StaticApi>::from(1i64);
    let ln_smallest = smallest.ln().unwrap();
    assert_eq!(
        ln_smallest.to_managed_decimal_signed(9usize).to_string(),
        "0.000000000"
    );
    assert!(ln_smallest.is_close(
        &BigFloat::from(0i64), // 0.0
        &BigFloat::from_frac(1, 100_000_000)
    ));

    let y = BigFloat::<StaticApi>::from_frac(11i64, 10i64);
    let ln_y = y.ln().unwrap();
    assert_eq!(
        ln_y.to_managed_decimal_signed(9usize).to_string(),
        "0.095251830"
    );
    assert!(ln_y.is_close(
        &BigFloat::from_frac(95251830, 1_000_000_000), // 0.095310179
        &BigFloat::from_frac(1, 1_000_000_000)
    ));
}
