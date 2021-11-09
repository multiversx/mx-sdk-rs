use adder::*;
use elrond_wasm::{
    api::PrintApi,
    contract_base::ContractBase,
    types::{BigInt, BigUint, ManagedFrom},
};
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

    let biguint = BigUint::managed_from(adder.type_manager(), 0u64);
    DebugApi::dummy().print_biguint(&biguint);

    let _ = adder.add(BigInt::from_i64(adder.type_manager(), 1));
    assert_eq!(
        BigInt::from_i64(adder.type_manager(), 13),
        adder.sum().get()
    );
}
