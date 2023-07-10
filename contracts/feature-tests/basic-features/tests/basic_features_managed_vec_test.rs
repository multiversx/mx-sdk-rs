use multiversx_sc::types::{BigUint, ManagedVec};
use multiversx_sc_scenario::{api::StaticApi, *};

use basic_features::managed_vec_features::ManagedVecFeatures;

#[test]
fn test_managed_vec_new() {
    let bf = basic_features::contract_obj::<StaticApi>();
    let result = bf.managed_vec_new();
    assert_eq!(ManagedVec::new(), result);
}

#[test]
fn test_managed_vec_eq() {
    let bf = basic_features::contract_obj::<StaticApi>();

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

#[test]
fn test_managed_vec_set() {
    let bf = basic_features::contract_obj::<StaticApi>();

    let mut mv1 = ManagedVec::new();
    mv1.push(BigUint::from(1u32));
    mv1.push(BigUint::from(2u32));
    mv1.push(BigUint::from(3u32));
    let mut mv2 = ManagedVec::new();
    mv2.push(BigUint::from(1u32));
    mv2.push(BigUint::from(5u32));
    mv2.push(BigUint::from(3u32));
    assert_eq!(bf.managed_vec_set(mv1, 1, &BigUint::from(5u64)), mv2);
}
