use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::api::StaticApi;

fn assert_big_uint_ln(x: u32, ln_str: &str) {
    let x = BigUint::<StaticApi>::from(x);
    let ln_x = x.ln();
    assert_eq!(ln_x.unwrap().to_string(), ln_str);
}

fn assert_big_uint_proportion(x: u64, part: u64, total: u64, expected: u64) {
    let big = BigUint::<StaticApi>::from(x);
    let expected = BigUint::<StaticApi>::from(expected);
    assert_eq!(big.proportion(part, total), expected);
    assert_eq!(big.clone().into_proportion(part, total), expected);
}

#[test]
fn test_big_uint_ln() {
    // have tested this value during development
    assert_big_uint_ln(23, "3.135514649"); // vs. 3.1354942159291497 first 6 decimals are ok
    // small numbers
    assert_big_uint_ln(1, "0.000060599");
    assert_big_uint_ln(2, "0.693207779"); // vs. 0.6931471805599453
    assert_big_uint_ln(3, "1.098595430"); // vs. 1.0986122886681096
    assert_big_uint_ln(4, "1.386354959"); // vs. 1.3862943611198906
    assert_big_uint_ln(5, "1.609481340"); // vs. 1.6094379124341003
    assert_big_uint_ln(6, "1.791742610"); // vs. 1.791759469228055
    // large number
    assert_big_uint_ln(1000, "6.907784913"); // vs. 6.907755278982137
}

#[test]
fn test_big_uint_proportion_all() {
    // Test basic proportions
    assert_big_uint_proportion(1000, 0, 100, 0);
    assert_big_uint_proportion(1000, 25, 100, 250);
    assert_big_uint_proportion(1000, 50, 100, 500);
    assert_big_uint_proportion(1000, 75, 100, 750);
    assert_big_uint_proportion(1000, 100, 100, 1000);
    // Test with different total
    assert_big_uint_proportion(2000, 1, 4, 500);
    assert_big_uint_proportion(2000, 3, 4, 1500);
    // Test rounding behavior - should truncate
    assert_big_uint_proportion(100, 1, 3, 33); // 33.333... -> 33
    // Test zero and large proportions
    assert_big_uint_proportion(1000, 0, 100, 0);
    assert_big_uint_proportion(1000000, 999, 1000, 999000);
    assert_big_uint_proportion(100, 200, 100, 200); // 200% of 100 = 200

    // Test with total at i64::MAX
    let max_i64 = i64::MAX as u64;
    // 100% of 100 should be 100
    assert_big_uint_proportion(100, max_i64, max_i64, 100);
    // ~50% of 100, there are some rounding errors
    assert_big_uint_proportion(100, max_i64 / 2, max_i64, 49);
}

#[test]
#[should_panic = "StaticApi signal error: proportion overflow"]
fn test_big_uint_proportion_overflow() {
    let _ = BigUint::<StaticApi>::from(100u64).proportion(100, i64::MAX as u64 + 1);
}
