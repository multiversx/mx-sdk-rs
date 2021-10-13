use adder::*;
use elrond_wasm::{contract_base::ContractBase, types::BigInt};
use elrond_wasm_debug::DebugApi;

#[test]
fn test_add() {
    let adder = adder::contract_obj(DebugApi::dummy());

    adder.init(BigInt::from_i64(adder.type_manager(), 5));
    assert_eq!(BigInt::from_i64(adder.type_manager(), 5), adder.sum().get());

    let _ = adder.add(BigInt::from_i64(adder.type_manager(), 7));
    assert_eq!(
        BigInt::from_i64(adder.type_manager(), 12),
        adder.sum().get()
    );

    let _ = adder.add(BigInt::from_i64(adder.type_manager(), 1));
    assert_eq!(
        BigInt::from_i64(adder.type_manager(), 13),
        adder.sum().get()
    );
}
