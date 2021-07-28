use elrond_wasm_debug::api::RustBigUint;
use elrond_wasm_debug::TxContext;
use factorial::*;

#[test]
fn test_add() {
    let factorial = factorial::contract_obj(TxContext::dummy());

    assert_eq!(
        RustBigUint::from(1u32),
        factorial.factorial(RustBigUint::from(0u32))
    );
    assert_eq!(
        RustBigUint::from(1u32),
        factorial.factorial(RustBigUint::from(1u32))
    );
    assert_eq!(
        RustBigUint::from(2u32),
        factorial.factorial(RustBigUint::from(2u32))
    );
    assert_eq!(
        RustBigUint::from(6u32),
        factorial.factorial(RustBigUint::from(3u32))
    );
    assert_eq!(
        RustBigUint::from(24u32),
        factorial.factorial(RustBigUint::from(4u32))
    );
    assert_eq!(
        RustBigUint::from(120u32),
        factorial.factorial(RustBigUint::from(5u32))
    );
}
