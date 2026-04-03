use multiversx_sc::{
    typenum::U27,
    types::{BigUint, ConstDecimals, ManagedDecimal, NumDecimals},
};
use multiversx_sc_scenario::api::StaticApi;

// ── compute_compounded_interest ───────────────────────────────────────────────
// Uses a 5-term Taylor series: e^x ≈ 1 + x + x²/2! + x³/3! + x⁴/4! + x⁵/5!

const RAY_PRECISION_NUM: usize = 27;
const RAY_PRECISION_CONST: ConstDecimals<U27> = ConstDecimals::new();
// "1" at RAY precision: 10^27
const RAY: u128 = 1_000_000_000_000_000_000_000_000_000;

fn ray_dec(raw: u128) -> ManagedDecimal<StaticApi, NumDecimals> {
    ManagedDecimal::from_raw_units(BigUint::from(raw), RAY_PRECISION_NUM)
}

#[test]
fn test_compounded_interest_zero_expiration() {
    // e^(rate * 0) = 1 regardless of rate
    let rate = ray_dec(50_000_000_000_000_000_000_000_000u128); // 0.05 at RAY precision
    let result = rate.compounded_interest(0, RAY_PRECISION_NUM);
    assert_eq!(result.scale(), RAY_PRECISION_NUM);
    assert_eq!(result.into_raw_units(), &BigUint::from(RAY));
}

#[test]
fn test_compounded_interest_zero_rate() {
    // e^(0 * t) = 1
    let rate = ray_dec(0);
    let result = rate.compounded_interest(100, RAY_PRECISION_NUM);
    // x = 0, all powers of x are 0, result = 1
    assert_eq!(result.into_raw_units(), &BigUint::from(RAY));
}

#[test]
fn test_compounded_interest_small_rate() {
    // rate = 0.05 (5 %), expiration = 1 second
    // x = 0.05 * 1 = 0.05
    // e^0.05 ≈ 1.05127109637602412428...
    // 5-term Taylor: 1 + 0.05 + 0.0025/2 + 0.000125/6 + 0.000006.../24 + ...
    //              = 1 + 0.05 + 0.00125 + 0.000020833... + 0.000000260... + 0.0000000026...
    //              ≈ 1.051271096 (9dp)
    // At RAY (27dp) precision:
    // 1.051271096376024124... * 10^27 ≈ 1_051_271_096_376_024_124_000_000_000
    let rate = ray_dec(50_000_000_000_000_000_000_000_000u128); // 0.05 * 10^27
    let result = rate.compounded_interest(1, RAY_PRECISION_NUM);

    // First 18 significant digits should be accurate (5-term Taylor error for x=0.05 is ~x^6/720)
    // Verify it's strictly > 1 (growth factor > 1)
    assert!(*result.into_raw_units() > BigUint::from(RAY));
    // Verify it matches the known 5-term approximation to sufficient precision
    // 5-term Taylor for x=0.05: 1 + 0.05 + 0.00125 + 0.0000208333 + 0.0000002604 + 0.0000000026
    //                          = 1.0512710963...
    // 5-term Taylor exact value: 1 + 0.05 + 0.00125 + 0.000020833... + 0.000000260416... + ...
    // = 1.051271096354166666... → at 27dp with half-up rounding:
    let expected = BigUint::from(1_051_271_096_354_166_666_666_666_667u128);
    assert_eq!(result.into_raw_units(), &expected);
}

#[test]
fn test_compounded_interest_one_percent_per_second() {
    // rate = 0.01, expiration = 1
    // e^0.01 ≈ 1.01005016708...
    // 5-term Taylor: 1 + 0.01 + 0.00005 + 0.000000166... + 0.000000000416... + ~0
    //              ≈ 1.010050167
    let rate = ray_dec(10_000_000_000_000_000_000_000_000u128); // 0.01 * 10^27
    let result = rate.compounded_interest(1, RAY_PRECISION_NUM);
    assert!(*result.into_raw_units() > BigUint::from(RAY));
    // 5-term Taylor: 1 + 0.01 + 0.00005 + 0.000000166... + 0.000000000416... + ...
    // = 1.010050167083... → at 27dp with half-up rounding:
    let expected = BigUint::from(1_010_050_167_084_166_666_666_666_667u128);
    assert_eq!(result.into_raw_units(), &expected);
}

#[test]
fn test_compounded_interest_expiration_scaling() {
    // rate = 0.001 (0.1%), expiration = 10
    // x = 0.001 * 10 = 0.01
    // Should equal the previous test (rate=0.01, exp=1) since x is the same
    let rate = ray_dec(1_000_000_000_000_000_000_000_000u128); // 0.001 * 10^27
    let result = rate.compounded_interest(10, RAY_PRECISION_NUM);
    // x = 0.001 * 10 = 0.01 — same x as above, must give the same result
    let expected = BigUint::from(1_010_050_167_084_166_666_666_666_667u128);
    assert_eq!(result.into_raw_units(), &expected);
}

fn ray_dec_const(raw: u128) -> ManagedDecimal<StaticApi, ConstDecimals<U27>> {
    ManagedDecimal::const_decimals_from_raw(BigUint::from(raw))
}

#[test]
fn test_compounded_interest_const_zero_expiration() {
    let rate = ray_dec_const(50_000_000_000_000_000_000_000_000u128); // 0.05
    let result = rate.compounded_interest(0, RAY_PRECISION_CONST);
    assert_eq!(result.scale(), 27);
    assert_eq!(result.into_raw_units(), &BigUint::from(RAY));
}

#[test]
fn test_compounded_interest_const_zero_rate() {
    let rate = ray_dec_const(0);
    let result = rate.compounded_interest(100, RAY_PRECISION_CONST);
    assert_eq!(result.into_raw_units(), &BigUint::from(RAY));
}

#[test]
fn test_compounded_interest_const_matches_num_small_rate() {
    // ConstDecimals<U27> and NumDecimals=27 must produce identical results
    let rate_const = ray_dec_const(50_000_000_000_000_000_000_000_000u128);
    let rate_num = ray_dec(50_000_000_000_000_000_000_000_000u128);

    let result_const = rate_const.compounded_interest(1, RAY_PRECISION_CONST);
    let result_num = rate_num.compounded_interest(1, RAY_PRECISION_NUM);

    assert_eq!(result_const.into_raw_units(), result_num.into_raw_units());
    let expected = BigUint::from(1_051_271_096_354_166_666_666_666_667u128);
    assert_eq!(result_const.into_raw_units(), &expected);
}

#[test]
fn test_compounded_interest_const_matches_num_one_percent() {
    let rate_const = ray_dec_const(10_000_000_000_000_000_000_000_000u128);
    let rate_num = ray_dec(10_000_000_000_000_000_000_000_000u128);

    let result_const = rate_const.compounded_interest(1, RAY_PRECISION_CONST);
    let result_num = rate_num.compounded_interest(1, RAY_PRECISION_NUM);

    assert_eq!(result_const.into_raw_units(), result_num.into_raw_units());
    let expected = BigUint::from(1_010_050_167_084_166_666_666_666_667u128);
    assert_eq!(result_const.into_raw_units(), &expected);
}
