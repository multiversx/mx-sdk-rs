use std::ops::Deref;

use multiversx_sc::types::{BigUint, ManagedBuffer, ManagedRef, ManagedVec};
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
fn test_to_array_of_refs() {
    let mut vec = ManagedVec::<StaticApi, i32>::new();
    for i in 0..10 {
        vec.push(i);
    }

    let refs: Option<[i32; 20]> = vec.to_array_of_refs();
    assert!(refs.is_none());

    let refs: Option<[i32; 10]> = vec.to_array_of_refs();
    assert!(refs.is_some());

    let refs = refs.unwrap();
    for (i, &item) in refs.iter().enumerate() {
        assert_eq!(item, i as i32);
    }
}

#[test]
fn test_take_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in 20u64..=30u64 {
        managed_vec.push(i);
        vec.push(i);
    }

    assert_eq!(managed_vec.len(), 11);
    assert_eq!(managed_vec.take(4), 24u64);
    assert_eq!(managed_vec.len(), 10);
}

#[test]
fn test_take_biguint() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in 20u64..=30u64 {
        let biguint = BigUint::<StaticApi>::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }
    assert_eq!(managed_vec.len(), 11);
    assert_eq!(managed_vec.take(4), BigUint::<StaticApi>::from(24u64));
    assert_eq!(managed_vec.len(), 10);
}

#[test]
#[allow(deprecated)]
fn test_sort_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in (20u64..=30u64).rev() {
        managed_vec.push(i);
        vec.push(i);
    }

    assert!(!managed_vec.is_sorted());
    managed_vec.sort();
    vec.sort();

    assert!(managed_vec.is_sorted());
    managed_vec.sort();
    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
#[allow(deprecated)]
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
#[allow(deprecated)]
fn test_sort_by_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in (20u64..=30u64).rev() {
        managed_vec.push(i);
        vec.push(i);
    }

    assert!(!managed_vec.is_sorted());

    managed_vec.sort_by(|a, b| flip(a).cmp(&flip(b)));
    vec.sort_by_key(flip);

    assert!(!managed_vec.is_sorted());

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
#[allow(deprecated)]
fn test_sort_by_biguint() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in (20u64..=30u64).rev() {
        let biguint = BigUint::<StaticApi>::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }

    assert!(!managed_vec.is_sorted());

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

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
#[allow(deprecated)]
fn test_sort_by_key_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in [4444u64, 333u64, 1u64, 22u64] {
        managed_vec.push(i);
        vec.push(i);
    }

    assert!(!managed_vec.is_sorted());

    managed_vec.sort_by_key(|a| a.to_string().len());
    vec.sort_by_key(|a| a.to_string().len());

    assert!(managed_vec.is_sorted());

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
#[allow(deprecated)]
fn test_sort_by_key_biguint() {
    let mut vec = Vec::<BigUint<StaticApi>>::new();
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in [4444u64, 333u64, 1u64, 22u64] {
        let biguint = BigUint::<StaticApi>::from(i);
        managed_vec.push(biguint.clone());
        vec.push(biguint);
    }

    assert!(!managed_vec.is_sorted());

    managed_vec.sort_by_key(|a| a.to_u64().unwrap().to_string().len());
    vec.sort_by_key(|a| a.to_u64().unwrap().to_string().len());

    assert!(managed_vec.is_sorted());

    assert_eq!(vec, managed_vec.into_vec());
}

#[test]
#[allow(deprecated)]
fn test_sort_by_cached_key_u64() {
    let mut vec = Vec::<u64>::new();
    let mut managed_vec = ManagedVec::<StaticApi, u64>::new();
    for i in [4u64, 33u64, 222u64, 1111u64] {
        managed_vec.push(i);
        vec.push(i);
    }

    managed_vec.sort_by_cached_key(|a| a.to_string());
    vec.sort_by_cached_key(|a| a.to_string());
    let managed_vec_as_vec = managed_vec.into_vec();
    assert_eq!(managed_vec_as_vec, [1111u64, 222u64, 33u64, 4u64]);
    assert_eq!(vec, managed_vec_as_vec);
}

#[test]
#[allow(deprecated)]
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
    let managed_vec_as_vec = managed_vec.into_vec();
    assert_eq!(managed_vec_as_vec, [1111u64, 222u64, 33u64, 4u64]);
    assert_eq!(vec, managed_vec_as_vec);
}

#[test]
#[allow(deprecated)]
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

    managed_vec.sort_unstable_by(|a, b| flip(a).cmp(&flip(b)));
    vec.sort_unstable_by_key(flip);

    assert!(!managed_vec.is_sorted());

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

    managed_vec.sort_unstable_by_key(|a| a.to_string().len());
    vec.sort_unstable_by_key(|a| a.to_string().len());

    assert!(managed_vec.is_sorted());

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

    managed_vec.sort_unstable_by_key(|a| a.to_u64().unwrap().to_string().len());
    vec.sort_unstable_by_key(|a| a.to_u64().unwrap().to_string().len());

    assert!(managed_vec.is_sorted());

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
#[allow(deprecated)]
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
#[allow(deprecated)]
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

#[test]
fn test_eq_managed_buffer() {
    let make_vec = |items: &[&[u8]]| -> ManagedVec<StaticApi, ManagedBuffer<StaticApi>> {
        let mut v = ManagedVec::new();
        for &item in items {
            v.push(ManagedBuffer::new_from_bytes(item));
        }
        v
    };

    // equal vecs
    assert_eq!(make_vec(&[b"foo", b"bar"]), make_vec(&[b"foo", b"bar"]));

    // different content
    assert_ne!(make_vec(&[b"foo", b"bar"]), make_vec(&[b"foo", b"baz"]));

    // different length
    assert_ne!(make_vec(&[b"foo", b"bar"]), make_vec(&[b"foo"]));

    // both empty
    assert_eq!(make_vec(&[]), make_vec(&[]));
}

#[test]
fn test_eq_u32() {
    let make_vec = |items: &[u32]| -> ManagedVec<StaticApi, u32> {
        let mut v = ManagedVec::new();
        for &item in items {
            v.push(item);
        }
        v
    };

    // equal vecs
    assert_eq!(make_vec(&[1, 2, 3]), make_vec(&[1, 2, 3]));

    // different content
    assert_ne!(make_vec(&[1, 2, 3]), make_vec(&[1, 2, 4]));

    // different length
    assert_ne!(make_vec(&[1, 2, 3]), make_vec(&[1, 2]));

    // both empty
    assert_eq!(make_vec(&[]), make_vec(&[]));
}

#[test]
fn test_is_single_item() {
    let mut managed_vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    assert!(managed_vec.is_single_item().is_none());

    managed_vec.push(BigUint::<StaticApi>::from(1u32));
    assert_eq!(
        managed_vec.is_single_item(),
        Some(ManagedRef::new(&BigUint::<StaticApi>::from(1u32)))
    );

    managed_vec.push(BigUint::<StaticApi>::from(2u32));
    assert!(managed_vec.is_single_item().is_none());
}

#[test]
fn test_byte_len() {
    let mut vec = ManagedVec::<StaticApi, u32>::new();
    assert_eq!(vec.byte_len(), 0);
    vec.push(1u32);
    assert_eq!(vec.byte_len(), 4); // u32 is 4 bytes
    vec.push(2u32);
    assert_eq!(vec.byte_len(), 8);
    vec.push(3u32);
    assert_eq!(vec.byte_len(), 12);
}

#[test]
fn test_is_empty() {
    let mut vec = ManagedVec::<StaticApi, u32>::new();
    assert!(vec.is_empty());
    vec.push(42u32);
    assert!(!vec.is_empty());
}

#[test]
fn test_is_empty_biguint() {
    let mut vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    assert!(vec.is_empty());
    vec.push(BigUint::from(42u64));
    assert!(!vec.is_empty());
}

#[test]
fn test_try_get() {
    let mut vec = ManagedVec::<StaticApi, u32>::new();
    assert!(vec.try_get(0).is_none());
    vec.push(10u32);
    vec.push(20u32);
    assert_eq!(vec.try_get(0).unwrap(), 10u32);
    assert_eq!(vec.try_get(1).unwrap(), 20u32);
    assert!(vec.try_get(2).is_none());
}

#[test]
fn test_try_get_biguint() {
    let mut vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    assert!(vec.try_get(0).is_none());
    vec.push(BigUint::from(10u64));
    vec.push(BigUint::from(20u64));
    assert_eq!(*vec.try_get(0).unwrap(), BigUint::from(10u64));
    assert_eq!(*vec.try_get(1).unwrap(), BigUint::from(20u64));
    assert!(vec.try_get(2).is_none());
}

#[test]
fn test_set_u32() {
    let mut vec = ManagedVec::<StaticApi, u32>::new();
    vec.push(10u32);
    vec.push(20u32);
    vec.push(30u32);

    let old = vec.set(1, 99u32).unwrap();
    assert_eq!(old, 20u32);
    assert_eq!(vec.get(0), 10u32);
    assert_eq!(vec.get(1), 99u32);
    assert_eq!(vec.get(2), 30u32);
}

#[test]
fn test_set_biguint() {
    let mut vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    vec.push(BigUint::from(10u64));
    vec.push(BigUint::from(20u64));
    vec.push(BigUint::from(30u64));

    let old = vec.set(1, BigUint::from(99u64)).unwrap();
    assert_eq!(old, BigUint::from(20u64));
    assert_eq!(*vec.get(0), BigUint::from(10u64));
    assert_eq!(*vec.get(1), BigUint::from(99u64));
    assert_eq!(*vec.get(2), BigUint::from(30u64));
}

#[test]
fn test_slice_u32() {
    let mut vec = ManagedVec::<StaticApi, u32>::new();
    for i in 1u32..=5u32 {
        vec.push(i);
    }

    let sliced = vec.slice(1, 4).unwrap();
    assert_eq!(sliced.len(), 3);
    assert_eq!(sliced.get(0), 2u32);
    assert_eq!(sliced.get(1), 3u32);
    assert_eq!(sliced.get(2), 4u32);

    // original is intact
    assert_eq!(vec.len(), 5);

    // empty slice (start == end)
    let empty = vec.slice(2, 2).unwrap();
    assert!(empty.is_empty());

    // out of range
    assert!(vec.slice(3, 10).is_none());

    // full slice
    let full = vec.slice(0, 5).unwrap();
    assert_eq!(full.len(), 5);
}

#[test]
fn test_slice_biguint() {
    let mut vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in 1u64..=5u64 {
        vec.push(BigUint::from(i));
    }

    let sliced = vec.slice(1, 4).unwrap();
    assert_eq!(sliced.len(), 3);
    assert_eq!(*sliced.get(0), BigUint::from(2u64));
    assert_eq!(*sliced.get(1), BigUint::from(3u64));
    assert_eq!(*sliced.get(2), BigUint::from(4u64));

    // original is intact after slicing (items were deep-copied)
    assert_eq!(vec.len(), 5);
    assert_eq!(*vec.get(0), BigUint::from(1u64));

    // out of range
    assert!(vec.slice(3, 10).is_none());
}

#[test]
fn test_remove_u32() {
    let mut vec = ManagedVec::<StaticApi, u32>::new();
    vec.push(10u32);
    vec.push(20u32);
    vec.push(30u32);

    vec.remove(1);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.get(0), 10u32);
    assert_eq!(vec.get(1), 30u32);

    vec.remove(0);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.get(0), 30u32);

    vec.remove(0);
    assert!(vec.is_empty());
}

#[test]
fn test_remove_biguint() {
    let mut vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    vec.push(BigUint::from(10u64));
    vec.push(BigUint::from(20u64));
    vec.push(BigUint::from(30u64));

    vec.remove(1);
    assert_eq!(vec.len(), 2);
    assert_eq!(*vec.get(0), BigUint::from(10u64));
    assert_eq!(*vec.get(1), BigUint::from(30u64));
}

#[test]
fn test_from_single_item_u32() {
    let vec = ManagedVec::<StaticApi, u32>::from_single_item(42u32);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.get(0), 42u32);
}

#[test]
fn test_from_single_item_biguint() {
    let vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::from_single_item(BigUint::from(42u64));
    assert_eq!(vec.len(), 1);
    assert_eq!(*vec.get(0), BigUint::from(42u64));
}

#[test]
fn test_clear_u32() {
    let mut vec = ManagedVec::<StaticApi, u32>::new();
    for i in 1u32..=5u32 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 5);
    vec.clear();
    assert!(vec.is_empty());
    // can still push after clear
    vec.push(1u32);
    assert_eq!(vec.len(), 1);
}

#[test]
fn test_clear_biguint() {
    let mut vec = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    for i in 1u64..=5u64 {
        vec.push(BigUint::from(i));
    }
    assert_eq!(vec.len(), 5);
    vec.clear();
    assert!(vec.is_empty());
    // can still push after clear
    vec.push(BigUint::from(1u64));
    assert_eq!(vec.len(), 1);
}

#[test]
fn test_find() {
    let mut vec = ManagedVec::<StaticApi, u32>::new();
    vec.push(10u32);
    vec.push(20u32);
    vec.push(30u32);
    vec.push(20u32); // duplicate

    assert_eq!(vec.find(&10u32), Some(0));
    assert_eq!(vec.find(&20u32), Some(1)); // first occurrence
    assert_eq!(vec.find(&30u32), Some(2));
    assert_eq!(vec.find(&99u32), None);

    let empty = ManagedVec::<StaticApi, u32>::new();
    assert_eq!(empty.find(&10u32), None);
}

#[test]
fn test_contains() {
    let mut vec = ManagedVec::<StaticApi, u32>::new();
    vec.push(10u32);
    vec.push(20u32);
    vec.push(30u32);

    assert!(vec.contains(&10u32));
    assert!(vec.contains(&20u32));
    assert!(vec.contains(&30u32));
    assert!(!vec.contains(&99u32));

    let empty = ManagedVec::<StaticApi, u32>::new();
    assert!(!empty.contains(&10u32));
}
