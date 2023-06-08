use std::ops::Deref;

use multiversx_sc::types::{BigUint, ManagedVec};
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn test_managed_vec_iter_rev() {
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
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
fn test_managed_vec_from_iterator_trait() {
    let mut managed_vec = ManagedVec::<StaticApi, i32>::new();
    for i in 1..=10 {
        managed_vec.push(i);
    }
    let mut expected_vec = ManagedVec::<StaticApi, i32>::new();
    for i in 1..=5 {
        expected_vec.push(i);
    }

    let collected_vec = managed_vec
        .iter()
        .filter(|x| x <= &5)
        .collect::<ManagedVec<StaticApi, i32>>();

    assert_eq!(collected_vec, expected_vec);
}

#[test]
fn test_managed_vec_extend_trait() {
    let mut managed_vec = ManagedVec::<StaticApi, i32>::new();
    for i in 1..=10 {
        managed_vec.push(i);
    }
    let mut expected_vec1 = ManagedVec::<StaticApi, i32>::new();
    for i in 1..=5 {
        expected_vec1.push(i);
    }
    let mut expected_vec2 = ManagedVec::<StaticApi, i32>::new();
    for i in 6..=10 {
        expected_vec2.push(i);
    }

    let (collected_vec1, collected_vec2) = managed_vec
        .iter()
        .partition::<ManagedVec<StaticApi, i32>, _>(|x| x <= &5);

    assert_eq!(collected_vec1, expected_vec1);
    assert_eq!(collected_vec2, expected_vec2);
}

#[test]
fn test_managed_vec_iter_exact_size_trait() {
    let mut managed_vec = ManagedVec::<StaticApi, i32>::new();
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
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in 20u64..=30u64 {
        let biguint = BigUint::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sort_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in (20u64..=30u64).rev() {
        managed_vec.push(i);
        vec.push(i);
    }

    assert!(!managed_vec.is_sorted());
    assert!(managed_vec.is_sorted_by(|a, b| Some(a.cmp(&(b * 10u64)))));
    assert!(managed_vec.is_sorted_by_key(|d| d / 10u64 > 1u64));
    managed_vec.sort();
    vec.sort();

    assert!(managed_vec.is_sorted());
    managed_vec.sort();
    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sort_biguint() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in (20u64..=30u64).rev() {
        let biguint = BigUint::<StaticApi>::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }
    assert!(!managed_vec.is_sorted());
    managed_vec.sort();
    vec.sort();
    assert!(managed_vec.is_sorted());

    assert_eq!(vec, managed_vec.into_vec());
}

// flips a two-digit number
fn flip(n: &u64) -> u64 {
    n / 10u64 + n % 10u64 * 10
}

#[test]
fn test_sort_by_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in (20u64..=30u64).rev() {
        managed_vec.push(i);
        vec.push(i);
    }

    assert!(!managed_vec.is_sorted());

    assert!(!managed_vec.is_sorted_by(|a, b| Some(flip(a).cmp(&flip(b)))));
    managed_vec.sort_by(|a, b| flip(a).cmp(&flip(b)));
    vec.sort_by_key(flip);

    assert!(!managed_vec.is_sorted());

    assert!(managed_vec.is_sorted_by(|a, b| Some(flip(a).cmp(&flip(b)))));
    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sort_by_biguint() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in (20u64..=30u64).rev() {
        let biguint = BigUint::<StaticApi>::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }

    assert!(!managed_vec.is_sorted());

    assert!(!managed_vec.is_sorted_by(|a, b| {
        let a_u64 = a.to_u64().unwrap();
        let b_u64 = b.to_u64().unwrap();
        Some(flip(&a_u64).cmp(&flip(&b_u64)))
    }));
    managed_vec.sort_by(|a, b| {
        let a_u64 = a.to_u64().unwrap();
        let b_u64 = b.to_u64().unwrap();
        flip(&a_u64).cmp(&flip(&b_u64))
    });
    vec.sort_by(|a, b| {
        let a_u64 = a.to_u64().unwrap();
        let b_u64 = b.to_u64().unwrap();
        flip(&a_u64).cmp(&flip(&b_u64))
    });

    assert!(!managed_vec.is_sorted());

    assert!(managed_vec.is_sorted_by(|a, b| {
        let a_u64 = a.to_u64().unwrap();
        let b_u64 = b.to_u64().unwrap();
        Some(flip(&a_u64).cmp(&flip(&b_u64)))
    }));
    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sort_by_key_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in [4444u64, 333u64, 1u64, 22u64] {
        managed_vec.push(i);
        vec.push(i);
    }

    assert!(!managed_vec.is_sorted());

    assert!(!managed_vec.is_sorted_by_key(|a| a.to_string().len()));
    managed_vec.sort_by_key(|a| a.to_string().len());
    vec.sort_by_key(|a| a.to_string().len());

    assert!(managed_vec.is_sorted());

    assert!(managed_vec.is_sorted_by_key(|a| a.to_string().len()));
    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sort_by_key_biguint() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in [4444u64, 333u64, 1u64, 22u64] {
        let biguint = BigUint::<StaticApi>::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }

    assert!(!managed_vec.is_sorted());

    assert!(!managed_vec.is_sorted_by_key(|a| a.to_u64().unwrap().to_string().len()));
    managed_vec.sort_by_key(|a| a.to_u64().unwrap().to_string().len());
    vec.sort_by_key(|a| a.to_u64().unwrap().to_string().len());

    assert!(managed_vec.is_sorted());

    assert!(managed_vec.is_sorted_by_key(|a| a.to_u64().unwrap().to_string().len()));
    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sort_by_cached_key_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in [4u64, 33u64, 222u64, 1111u64] {
        managed_vec.push(i);
        vec.push(i);
    }

    managed_vec.sort_by_cached_key(|a| a.to_string());
    vec.sort_by_cached_key(|a| a.to_string());
    assert!(managed_vec.is_sorted_by_key(|a| a.to_string()));
    let managed_vec_as_vec = managed_vec.into_vec();
    assert_eq!(managed_vec_as_vec, [1111u64, 222u64, 33u64, 4u64]);
    assert_eq!(vec, managed_vec_as_vec);
}

#[test]
fn test_sort_by_cached_key_biguint() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in [4u64, 33u64, 222u64, 1111u64] {
        let biguint = BigUint::<StaticApi>::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }

    managed_vec.sort_by_cached_key(|a| a.to_u64().unwrap().to_string());
    vec.sort_by_cached_key(|a| a.to_u64().unwrap().to_string());
    assert!(managed_vec.is_sorted_by_key(|a| a.to_u64().unwrap().to_string()));
    let managed_vec_as_vec = managed_vec.into_vec();
    assert_eq!(managed_vec_as_vec, [1111u64, 222u64, 33u64, 4u64]);
    assert_eq!(vec, managed_vec_as_vec);
}

#[test]
fn test_sort_unstable_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in (20u64..=30u64).rev() {
        managed_vec.push(i);
        vec.push(i);
    }
    managed_vec.sort_unstable();
    vec.sort_unstable();

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sort_unstable_biguint() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in (20u64..=30u64).rev() {
        let biguint = BigUint::<StaticApi>::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }
    managed_vec.sort_unstable();
    vec.sort_unstable();

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sort_unstable_by_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in (20u64..=30u64).rev() {
        managed_vec.push(i);
        vec.push(i);
    }

    assert!(!managed_vec.is_sorted());

    assert!(!managed_vec.is_sorted_by(|a, b| Some(flip(a).cmp(&flip(b)))));
    managed_vec.sort_unstable_by(|a, b| flip(a).cmp(&flip(b)));
    vec.sort_unstable_by_key(flip);

    assert!(!managed_vec.is_sorted());

    assert!(managed_vec.is_sorted_by(|a, b| Some(flip(a).cmp(&flip(b)))));
    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sort_unstable_by_biguint() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in (20u64..=30u64).rev() {
        let biguint = BigUint::<StaticApi>::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }

    assert!(!managed_vec.is_sorted());

    assert!(!managed_vec.is_sorted_by(|a, b| {
        let a_u64 = a.to_u64().unwrap();
        let b_u64 = b.to_u64().unwrap();
        Some(flip(&a_u64).cmp(&flip(&b_u64)))
    }));
    managed_vec.sort_unstable_by(|a, b| {
        let a_u64 = a.to_u64().unwrap();
        let b_u64 = b.to_u64().unwrap();
        flip(&a_u64).cmp(&flip(&b_u64))
    });
    vec.sort_unstable_by(|a, b| {
        let a_u64 = a.to_u64().unwrap();
        let b_u64 = b.to_u64().unwrap();
        flip(&a_u64).cmp(&flip(&b_u64))
    });

    assert!(!managed_vec.is_sorted());

    assert!(managed_vec.is_sorted_by(|a, b| {
        let a_u64 = a.to_u64().unwrap();
        let b_u64 = b.to_u64().unwrap();
        Some(flip(&a_u64).cmp(&flip(&b_u64)))
    }));
    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sort_unstable_by_key_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in [4444u64, 333u64, 1u64, 22u64] {
        managed_vec.push(i);
        vec.push(i);
    }

    assert!(!managed_vec.is_sorted());

    assert!(!managed_vec.is_sorted_by_key(|a| a.to_string().len()));
    managed_vec.sort_unstable_by_key(|a| a.to_string().len());
    vec.sort_unstable_by_key(|a| a.to_string().len());

    assert!(managed_vec.is_sorted());

    assert!(managed_vec.is_sorted_by_key(|a| a.to_string().len()));
    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sort_unstable_by_key_biguint() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in [4444u64, 333u64, 1u64, 22u64] {
        let biguint = BigUint::<StaticApi>::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }

    assert!(!managed_vec.is_sorted());

    assert!(!managed_vec.is_sorted_by_key(|a| a.to_u64().unwrap().to_string().len()));
    managed_vec.sort_unstable_by_key(|a| a.to_u64().unwrap().to_string().len());
    vec.sort_unstable_by_key(|a| a.to_u64().unwrap().to_string().len());

    assert!(managed_vec.is_sorted());

    assert!(managed_vec.is_sorted_by_key(|a| a.to_u64().unwrap().to_string().len()));
    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_dedup_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in [2u64, 3u64, 2u64, 2u64, 6u64, 3u64, 5u64, 2u64] {
        managed_vec.push(i);
        vec.push(i);
    }
    managed_vec.dedup();
    vec.dedup();

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_dedup_biguint() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in [2u64, 3u64, 2u64, 2u64, 6u64, 3u64, 5u64, 2u64] {
        let biguint = BigUint::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }
    managed_vec.dedup();
    vec.dedup();

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sorted_dedup_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in [2u64, 3u64, 2u64, 2u64, 6u64, 3u64, 5u64, 2u64] {
        managed_vec.push(i);
        vec.push(i);
    }
    managed_vec.sort();
    vec.sort();
    managed_vec.dedup();
    vec.dedup();

    assert_eq!(4, managed_vec.len());
    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_sorted_dedup_biguint() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in [2u64, 3u64, 2u64, 2u64, 6u64, 3u64, 5u64, 2u64] {
        let biguint = BigUint::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }
    managed_vec.sort();
    vec.sort();
    managed_vec.dedup();
    vec.dedup();

    assert_eq!(4, managed_vec.len());
    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_with_self_as_vec() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
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
    let mut vec = Vec::new();
    for i in 20u64..=30u64 {
        let biguint = BigUint::from(i);
        vec.push(biguint);
    }

    let managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::from(vec.clone());

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_append_vec() {
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

    let managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::from(vec.clone());
    let mut managed_vec1 = ManagedVec::<StaticApi, BigUint<StaticApi>>::from(vec1.clone());
    let managed_vec2 = ManagedVec::<StaticApi, BigUint<StaticApi>>::from(vec2.clone());

    managed_vec1.append_vec(managed_vec2);
    assert_eq!(managed_vec, managed_vec1);
}

#[test]
fn test_overwrite_with_single_item() {
    let mut vec = Vec::new();
    for i in 20u64..=30u64 {
        let biguint = BigUint::from(i);
        vec.push(biguint);
    }

    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::from(vec.clone());
    assert_eq!(vec, managed_vec.clone().into_vec());

    let single_elem = BigUint::from(100u64);
    managed_vec.overwrite_with_single_item(single_elem.clone());
    vec = vec![single_elem];

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
fn test_managed_vec_get_mut() {
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    managed_vec.push(BigUint::from(100u32));
    managed_vec.push(BigUint::from(100u32));

    {
        let mut first_elem = managed_vec.get_mut(0);
        *first_elem += 100u32;
    }
    assert_eq!(*managed_vec.get(0), 200u32);
    assert_eq!(*managed_vec.get(1), 100u32);

    {
        let first_elem = managed_vec.get(0).deref().clone();
        let mut second_elem = managed_vec.get_mut(1);
        *second_elem += first_elem;
    }

    assert_eq!(*managed_vec.get(0), 200u32);
    assert_eq!(*managed_vec.get(1), 300u32);
}
