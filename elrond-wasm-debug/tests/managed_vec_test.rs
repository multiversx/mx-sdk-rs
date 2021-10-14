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
fn test_into_vec() {
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

#[test]
fn test_managed_from() {
    let context = DebugApi::dummy();

    let mut vec = Vec::new();
    for i in 20u64..=30u64 {
        let biguint = BigUint::managed_from(context.clone(), i);
        vec.push(biguint);
    }

    let managed_vec =
        ManagedVec::<DebugApi, BigUint<DebugApi>>::managed_from(context.clone(), vec.clone());

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_append_vec() {
    let context = DebugApi::dummy();

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut vec = Vec::new();

    for i in 20u64..=30u64 {
        let biguint = BigUint::managed_from(context.clone(), i);
        vec.push(biguint.clone());
        vec1.push(biguint);
    }

    for i in 20u64..=30u64 {
        let biguint = BigUint::managed_from(context.clone(), i);
        vec.push(biguint.clone());
        vec2.push(biguint);
    }

    let managed_vec =
        ManagedVec::<DebugApi, BigUint<DebugApi>>::managed_from(context.clone(), vec.clone());
    let mut managed_vec1 =
        ManagedVec::<DebugApi, BigUint<DebugApi>>::managed_from(context.clone(), vec1.clone());
    let managed_vec2 =
        ManagedVec::<DebugApi, BigUint<DebugApi>>::managed_from(context.clone(), vec2.clone());

    managed_vec1.append_vec(managed_vec2);
    assert_eq!(managed_vec, managed_vec1);
}

#[test]
fn test_overwrite_with_single_item() {
    let context = DebugApi::dummy();

    let mut vec = Vec::new();
    for i in 20u64..=30u64 {
        let biguint = BigUint::managed_from(context.clone(), i);
        vec.push(biguint);
    }

    let mut managed_vec =
        ManagedVec::<DebugApi, BigUint<DebugApi>>::managed_from(context.clone(), vec.clone());
    assert_eq!(vec, managed_vec.clone().into_vec());

    let single_elem = BigUint::managed_from(context.clone(), 100u64);
    managed_vec.overwrite_with_single_item(single_elem.clone());
    vec = vec![single_elem];

    assert_eq!(vec, managed_vec.into_vec());
}
