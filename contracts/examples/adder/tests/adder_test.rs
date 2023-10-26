use adder::*;
use multiversx_sc::types::BaseBigUint;

#[test]
#[cfg_attr(not(feature = "single-tx-api"), ignore)]
fn adder_unit_test() {
    let adder = adder::contract_obj();

    adder.init(BaseBigUint::from(5u32));
    assert_eq!(BaseBigUint::from(5u32), adder.sum().get());

    adder.add(BaseBigUint::from(7u32));
    assert_eq!(BaseBigUint::from(12u32), adder.sum().get());

    adder.add(BaseBigUint::from(1u32));
    assert_eq!(BaseBigUint::from(13u32), adder.sum().get());
}
