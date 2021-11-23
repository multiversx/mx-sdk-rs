use adder::*;
use elrond_wasm::types::BigInt;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_add() {
    let adder = adder::contract_obj(DebugApi::dummy());

    adder.init(BigInt::from(5));
    assert_eq!(BigInt::from(5), adder.sum().get());

    let _ = adder.add(BigInt::from(7));
    assert_eq!(BigInt::from(12), adder.sum().get());

    let _ = adder.add(BigInt::from(1));
    assert_eq!(BigInt::from(13), adder.sum().get());
}
