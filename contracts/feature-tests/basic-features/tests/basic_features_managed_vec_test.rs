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

#[test]
fn test_managed_vec_iter_rev() {
    let context = DebugApi::dummy();

    let mut managed_vec = ManagedVec::new(context.clone());
    for i in 20u64..=30u64 {
        managed_vec.push(BigUint::managed_from(context.clone(), i));
    }
    let numbers: Vec<u64> = managed_vec
        .iter()
        .map(|biguint| biguint.to_u64().unwrap())
        .collect();
    let expected_numbers: Vec<u64> = (20u64..=30u64).collect();
    assert_eq!(numbers, expected_numbers);

    let reversed_numbers: Vec<u64> = managed_vec
        .iter()
        .rev()
        .map(|biguint| biguint.to_u64().unwrap())
        .collect();
    let expected_reversed_numbers: Vec<u64> = (20u64..=30u64).rev().collect();
    assert_eq!(reversed_numbers, expected_reversed_numbers);
}
