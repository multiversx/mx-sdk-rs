use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn test_big_uint_ln() {
    // ln(23) = 3.1354942159291497
    let x = BigUint::<StaticApi>::from(23u32);
    let ln_x = x.ln();
    assert_eq!(ln_x.to_string(), "3.135514649"); // first 6 decimals are ok
}
