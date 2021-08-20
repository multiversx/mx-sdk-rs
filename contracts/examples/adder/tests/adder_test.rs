use adder::*;
use elrond_wasm::{api::ContractBase, types::BigInt};
use elrond_wasm_debug::TxContext;

#[test]
fn test_add() {
    let adder = adder::contract_obj(TxContext::dummy());

    adder.init(BigInt::from_i64(5, adder.type_manager()));
    assert_eq!(BigInt::from_i64(5, adder.type_manager()), adder.sum().get());

    let _ = adder.add(BigInt::from_i64(7, adder.type_manager()));
    assert_eq!(
        BigInt::from_i64(12, adder.type_manager()),
        adder.sum().get()
    );

    let _ = adder.add(BigInt::from_i64(1, adder.type_manager()));
    assert_eq!(
        BigInt::from_i64(13, adder.type_manager()),
        adder.sum().get()
    );
}
