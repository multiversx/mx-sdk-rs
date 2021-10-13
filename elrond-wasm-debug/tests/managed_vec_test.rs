use elrond_wasm::types::{BigUint, ManagedFrom, ManagedVec};
use elrond_wasm_debug::DebugApi;

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

#[test]
fn test_managed_into_vec() {
    let context = DebugApi::dummy();

    let mut vec = Vec::new();
    let mut managed_vec = ManagedVec::new(context.clone());
    for i in 20u64..=30u64 {
        let biguint = BigUint::managed_from(context.clone(), i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }

    assert_eq!(vec, managed_vec.into_vec());
}
