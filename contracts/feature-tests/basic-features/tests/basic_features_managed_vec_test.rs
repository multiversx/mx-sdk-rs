use elrond_wasm::types::{BigUint, ManagedFrom, ManagedVec};
use elrond_wasm_debug::*;

use basic_features::managed_vec_features::ManagedVecFeatures;

#[test]
fn test_managed_vec_new() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.managed_vec_new();
    assert_eq!(ManagedVec::new(context), result);
}

#[test]
fn test_managed_vec_eq() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());

    let mut mv1 = ManagedVec::new(context.clone());
    mv1.push(BigUint::managed_from(context.clone(), 1u32));
    mv1.push(BigUint::managed_from(context.clone(), 2u32));
    assert!(bf.managed_vec_biguint_eq(&mv1, &mv1));

    let mut mv2 = ManagedVec::new(context.clone());
    mv2.push(BigUint::managed_from(context.clone(), 1u32));
    mv2.push(BigUint::managed_from(context.clone(), 2u32));
    assert!(bf.managed_vec_biguint_eq(&mv1, &mv2));

    mv2.push(BigUint::managed_from(context.clone(), 3u32));
    assert!(!bf.managed_vec_biguint_eq(&mv1, &mv2));

    let mut mv3 = ManagedVec::new(context.clone());
    mv3.push(BigUint::managed_from(context.clone(), 1u32));
    mv3.push(BigUint::managed_from(context.clone(), 7u32));
    assert!(!bf.managed_vec_biguint_eq(&mv1, &mv3));
}
