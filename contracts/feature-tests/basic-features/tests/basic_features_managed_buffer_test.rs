use multiversx_sc::types::{ManagedAddress, ManagedBuffer};
use multiversx_sc_scenario::{api::StaticApi, *};

use basic_features::managed_buffer_features::ManagedBufferFeatures;

#[test]
fn test_managed_buffer_new_empty() {
    let bf = basic_features::contract_obj::<StaticApi>();
    let result = bf.mbuffer_new();
    assert_eq!(ManagedBuffer::new(), result);
}

#[test]
fn test_managed_buffer_from_big_float() {
    let bf = basic_features::contract_obj::<StaticApi>();
    let big_float = multiversx_sc::types::BigFloat::from_frac(3, 2);
    let result = bf.mbuffer_from_big_float(big_float);
    assert_eq!(ManagedBuffer::from("1.5"), result);
}

#[test]
fn test_managed_address_zero() {
    let bf = basic_features::contract_obj::<StaticApi>();
    let result = bf.managed_address_zero();
    assert_eq!(ManagedAddress::zero(), result);
}
