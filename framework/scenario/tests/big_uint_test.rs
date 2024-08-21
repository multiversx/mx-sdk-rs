use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::api::StaticApi;

fn assert_big_uint_ln(x: u32, ln_str: &str) {
    let x = BigUint::<StaticApi>::from(x);
    let ln_x = x.ln();
    assert_eq!(ln_x.unwrap().to_string(), ln_str);
}

#[test]
fn test_big_uint_ln() {
    assert_big_uint_ln(23, "3.135514649"); // vs. 3.1354942159291497 first 6 decimals are ok

    assert_big_uint_ln(1, "0.000060599");
    assert_big_uint_ln(2, "0.693207779"); // vs. 0.6931471805599453
    assert_big_uint_ln(3, "1.098595430"); // vs. 1.0986122886681096
    assert_big_uint_ln(4, "1.386354959"); // vs. 1.3862943611198906
    assert_big_uint_ln(5, "1.609481340"); // vs. 1.6094379124341003
    assert_big_uint_ln(6, "1.791742610"); // vs. 1.791759469228055

    assert_big_uint_ln(1000, "6.907784913"); // vs. 6.907755278982137
}
