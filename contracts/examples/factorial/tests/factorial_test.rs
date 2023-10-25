use factorial::*;
use multiversx_sc::types::BaseBigUint;
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn test_factorial() {
    let factorial = factorial::contract_obj();

    assert_eq!(
        BaseBigUint::<StaticApi>::from(1u32),
        factorial.factorial(0u32.into())
    );
    assert_eq!(
        BaseBigUint::<StaticApi>::from(1u32),
        factorial.factorial(1u32.into())
    );
    assert_eq!(
        BaseBigUint::<StaticApi>::from(2u32),
        factorial.factorial(2u32.into())
    );
    assert_eq!(
        BaseBigUint::<StaticApi>::from(6u32),
        factorial.factorial(3u32.into())
    );
    assert_eq!(
        BaseBigUint::<StaticApi>::from(24u32),
        factorial.factorial(4u32.into())
    );
    assert_eq!(
        BaseBigUint::<StaticApi>::from(120u32),
        factorial.factorial(5u32.into())
    );
}
