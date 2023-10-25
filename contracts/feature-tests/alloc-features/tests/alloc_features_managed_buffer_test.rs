use multiversx_sc::types::{BoxedBytes, ManagedAddress, ManagedBuffer};

use alloc_features::managed_buffer_features_alloc::ManagedBufferFeatures;

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
#[should_panic]
fn test_managed_buffer_set_slice_should_panic() {
    let bf = alloc_features::contract_obj();
    let buffer = bf.mbuffer_from_slice(&[1, 2, 3][..]);
    let _ = bf.mbuffer_set_slice(buffer, 4, &[5]);
}

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
fn test_managed_buffer_set_slice() {
    let bf = alloc_features::contract_obj();
    let buffer = bf.mbuffer_from_slice(&[1, 2, 3][..]);

    let set_result = bf.mbuffer_set_slice(buffer, 1, &[5][..]);
    let expected_set_result = bf.mbuffer_from_slice(&[1, 5, 3][..]);
    assert_eq!(expected_set_result, set_result);
}

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
fn test_managed_buffer_from() {
    let bf = alloc_features::contract_obj();
    let result = bf.mbuffer_from_slice(&[1, 2, 3][..]);
    assert_eq!(ManagedBuffer::from(&[1, 2, 3][..]), result);
    let result = bf.mbuffer_from_boxed_bytes(BoxedBytes::from(&[4, 5, 6][..]));
    assert_eq!(ManagedBuffer::from(&[4, 5, 6][..]), result);
}

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
fn test_managed_address_from() {
    let bf = alloc_features::contract_obj();
    assert_eq!(ManagedAddress::zero(), bf.managed_address_from(&[0u8; 32]));
}
