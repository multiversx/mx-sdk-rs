use multiversx_sc::types::{ManagedAddress, ManagedBuffer};

use basic_features::managed_buffer_features::ManagedBufferFeatures;

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
fn test_managed_buffer_new_empty() {
    let bf = basic_features::contract_obj();
    let result = bf.mbuffer_new();
    assert_eq!(ManagedBuffer::new(), result);
}

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
fn test_managed_address_zero() {
    let bf = basic_features::contract_obj();
    let result = bf.managed_address_zero();
    assert_eq!(ManagedAddress::zero(), result);
}
