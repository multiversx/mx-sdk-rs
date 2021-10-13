use elrond_wasm::contract_base::ContractBase;
use elrond_wasm_debug::DebugApi;
use factorial::*;

#[test]
fn test_factorial() {
    let factorial = factorial::contract_obj(DebugApi::dummy());

    assert_eq!(
        factorial.types().big_uint_from(1u32),
        factorial.factorial(factorial.types().big_uint_zero())
    );
    assert_eq!(
        factorial.types().big_uint_from(1u32),
        factorial.factorial(factorial.types().big_uint_from(1u32))
    );
    assert_eq!(
        factorial.types().big_uint_from(2u32),
        factorial.factorial(factorial.types().big_uint_from(2u32))
    );
    assert_eq!(
        factorial.types().big_uint_from(6u32),
        factorial.factorial(factorial.types().big_uint_from(3u32))
    );
    assert_eq!(
        factorial.types().big_uint_from(24u32),
        factorial.factorial(factorial.types().big_uint_from(4u32))
    );
    assert_eq!(
        factorial.types().big_uint_from(120u32),
        factorial.factorial(factorial.types().big_uint_from(5u32))
    );
}
