use adder::*;
use mx_sc::types::BigUint;
use mx_sc_debug::DebugApi;

#[test]
fn test_add() {
    let _ = DebugApi::dummy();

    let adder = adder::contract_obj::<DebugApi>();

    adder.init(BigUint::from(5u32));
    assert_eq!(BigUint::from(5u32), adder.sum().get());

    adder.add(BigUint::from(7u32));
    assert_eq!(BigUint::from(12u32), adder.sum().get());

    adder.add(BigUint::from(1u32));
    assert_eq!(BigUint::from(13u32), adder.sum().get());
}
