use elrond_wasm::types::{BoxedBytes, ManagedAddress, ManagedBuffer};
use elrond_wasm_debug::*;

use basic_features::managed_buffer_features::ManagedBufferFeatures;

#[test]
fn test_managed_buffer_new_empty() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.mbuffer_new();
    assert_eq!(ManagedBuffer::new(), result);
}

#[test]
fn test_managed_buffer_from() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.mbuffer_from_slice(&[1, 2, 3][..]);
    assert_eq!(ManagedBuffer::from(&[1, 2, 3][..]), result);
    let result = bf.mbuffer_from_boxed_bytes(BoxedBytes::from(&[4, 5, 6][..]));
    assert_eq!(ManagedBuffer::from(&[4, 5, 6][..]), result);
}

#[test]
fn test_managed_address_zero() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.managed_address_zero();
    assert_eq!(ManagedAddress::zero(), result);
}

#[test]
fn test_managed_address_from() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    assert_eq!(
        bf.managed_address_zero(),
        bf.managed_address_from(&[0u8; 32])
    );
}
