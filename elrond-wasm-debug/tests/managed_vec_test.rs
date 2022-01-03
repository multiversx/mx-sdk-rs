use elrond_wasm::types::{BigUint, ManagedVec};
use elrond_wasm_debug::DebugApi;

#[test]
fn test_managed_vec_iter_rev() {
    let _ = DebugApi::dummy();

    let mut managed_vec = ManagedVec::<DebugApi, BigUint<DebugApi>>::new();
    for i in 20u64..=30u64 {
        managed_vec.push(BigUint::from(i));
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
fn test_managed_vec_iter_exact_size_trait() {
    let _ = DebugApi::dummy();

    let mut managed_vec = ManagedVec::<DebugApi, i32>::new();
    for i in 1..=10 {
        managed_vec.push(i);
    }
    assert!(managed_vec.len() == 10);
    let it = managed_vec.iter();
    assert!(it.size_hint() == (10, Some(10)));
    assert!(it.len() == 10);
    let it2 = it.skip(2);
    assert!(it2.len() == 8);
    let it3 = it2.clone();
    assert!(it2.skip(3).len() == 5);
    assert!(it3.skip(5).len() == 3);
}

#[test]
fn test_into_vec() {
    let _ = DebugApi::dummy();

    let mut vec = Vec::<BigUint<DebugApi>>::new();
    let mut managed_vec = ManagedVec::<DebugApi, BigUint<DebugApi>>::new();
    for i in 20u64..=30u64 {
        let biguint = BigUint::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_with_self_as_vec() {
    let _ = DebugApi::dummy();

    let mut vec = Vec::<BigUint<DebugApi>>::new();
    let mut managed_vec = ManagedVec::<DebugApi, BigUint<DebugApi>>::new();
    for i in 20u64..=30u64 {
        let biguint = BigUint::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }
    let item_to_remove = 25u64;
    if let Some(pos_to_remove) = vec.iter().position(|value| *value == item_to_remove) {
        let _ = vec.swap_remove(pos_to_remove);
    }

    managed_vec.with_self_as_vec(|t_vec| {
        if let Some(signer_pos) = t_vec.iter().position(|value| *value == item_to_remove) {
            let _ = t_vec.swap_remove(signer_pos);
        }
    });

    assert_eq!(managed_vec.into_vec(), vec);
}

#[test]
fn test_managed_from() {
    let _ = DebugApi::dummy();

    let mut vec = Vec::new();
    for i in 20u64..=30u64 {
        let biguint = BigUint::from(i);
        vec.push(biguint);
    }

    let managed_vec = ManagedVec::<DebugApi, BigUint<DebugApi>>::from(vec.clone());

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_append_vec() {
    let _ = DebugApi::dummy();

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut vec = Vec::new();

    for i in 20u64..=30u64 {
        let biguint = BigUint::from(i);
        vec.push(biguint.clone());
        vec1.push(biguint);
    }

    for i in 20u64..=30u64 {
        let biguint = BigUint::from(i);
        vec.push(biguint.clone());
        vec2.push(biguint);
    }

    let managed_vec = ManagedVec::<DebugApi, BigUint<DebugApi>>::from(vec.clone());
    let mut managed_vec1 = ManagedVec::<DebugApi, BigUint<DebugApi>>::from(vec1.clone());
    let managed_vec2 = ManagedVec::<DebugApi, BigUint<DebugApi>>::from(vec2.clone());

    managed_vec1.append_vec(managed_vec2);
    assert_eq!(managed_vec, managed_vec1);
}

#[test]
fn test_overwrite_with_single_item() {
    let _ = DebugApi::dummy();

    let mut vec = Vec::new();
    for i in 20u64..=30u64 {
        let biguint = BigUint::from(i);
        vec.push(biguint);
    }

    let mut managed_vec = ManagedVec::<DebugApi, BigUint<DebugApi>>::from(vec.clone());
    assert_eq!(vec, managed_vec.clone().into_vec());

    let single_elem = BigUint::from(100u64);
    managed_vec.overwrite_with_single_item(single_elem.clone());
    vec = vec![single_elem];

    assert_eq!(vec, managed_vec.into_vec());
}
