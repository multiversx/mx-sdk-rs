use elrond_wasm::types::{BigUint, ManagedVec};
use elrond_wasm_debug::*;

use basic_features::managed_vec_features::ManagedVecFeatures;

#[test]
fn test_managed_vec_new() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.managed_vec_new();
    assert_eq!(ManagedVec::new(), result);
}

#[test]
fn test_managed_vec_eq() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());

    let mut mv1 = ManagedVec::new();
    mv1.push(BigUint::from(1u32));
    mv1.push(BigUint::from(2u32));
    assert!(bf.managed_vec_biguint_eq(&mv1, &mv1));

    let mut mv2 = ManagedVec::new();
    mv2.push(BigUint::from(1u32));
    mv2.push(BigUint::from(2u32));
    assert!(bf.managed_vec_biguint_eq(&mv1, &mv2));

    mv2.push(BigUint::from(3u32));
    assert!(!bf.managed_vec_biguint_eq(&mv1, &mv2));

    let mut mv3 = ManagedVec::new();
    mv3.push(BigUint::from(1u32));
    mv3.push(BigUint::from(7u32));
    assert!(!bf.managed_vec_biguint_eq(&mv1, &mv3));
}
