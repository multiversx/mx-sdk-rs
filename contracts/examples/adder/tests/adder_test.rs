use adder::*;
use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::api::SingleTxApi;

#[test]
fn adder_unit_test() {
    let adder = adder::contract_obj::<SingleTxApi>();

    adder.init(BigUint::from(5u32));
    assert_eq!(BigUint::from(5u32), adder.sum().get());

    adder.add(BigUint::from(7u32));
    assert_eq!(BigUint::from(12u32), adder.sum().get());

    adder.add(BigUint::from(1u32));
    assert_eq!(BigUint::from(13u32), adder.sum().get());
}
