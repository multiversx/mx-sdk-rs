use elrond_wasm::types::BigUint;
use elrond_wasm_debug::DebugApi;
use factorial::*;

#[test]
fn test_factorial() {
    let factorial = factorial::contract_obj(DebugApi::dummy());

    assert_eq!(
        BigUint::<DebugApi>::from(1u32),
        factorial.factorial(0u32.into())
    );
    assert_eq!(
        BigUint::<DebugApi>::from(1u32),
        factorial.factorial(1u32.into())
    );
    assert_eq!(
        BigUint::<DebugApi>::from(2u32),
        factorial.factorial(2u32.into())
    );
    assert_eq!(
        BigUint::<DebugApi>::from(6u32),
        factorial.factorial(3u32.into())
    );
    assert_eq!(
        BigUint::<DebugApi>::from(24u32),
        factorial.factorial(4u32.into())
    );
    assert_eq!(
        BigUint::<DebugApi>::from(120u32),
        factorial.factorial(5u32.into())
    );
}
