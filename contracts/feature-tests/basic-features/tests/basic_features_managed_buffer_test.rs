use multiversx_sc::types::{ManagedAddress, ManagedBuffer};
use multiversx_sc_scenario::*;

use basic_features::managed_buffer_features::ManagedBufferFeatures;

#[test]
fn test_managed_buffer_new_empty() {
    let _ = DebugApi::dummy();
    let bf = basic_features::contract_obj::<DebugApi>();
    let result = bf.mbuffer_new();
    assert_eq!(ManagedBuffer::new(), result);
}

#[test]
fn test_managed_address_zero() {
    let _ = DebugApi::dummy();
    let bf = basic_features::contract_obj::<DebugApi>();
    let result = bf.managed_address_zero();
    assert_eq!(ManagedAddress::zero(), result);
}
