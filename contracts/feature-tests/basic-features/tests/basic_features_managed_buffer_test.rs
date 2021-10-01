use elrond_wasm::types::{BoxedBytes, ManagedBuffer, ManagedFrom};
use elrond_wasm_debug::*;

use basic_features::managed_buffer_features::ManagedBufferFeatures;

#[test]
fn test_managed_buffer_new_empty() {
    let context = TxContext::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.mbuffer_new();
    assert_eq!(ManagedBuffer::new(context), result);
}

#[test]
fn test_managed_buffer_from() {
    let context = TxContext::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.mbuffer_from_slice(&[1, 2, 3][..]);
    assert_eq!(
        ManagedBuffer::managed_from(context.clone(), &[1, 2, 3][..]),
        result
    );
    let result = bf.mbuffer_from_boxed_bytes(BoxedBytes::from(&[4, 5, 6][..]));
    assert_eq!(
        ManagedBuffer::managed_from(context.clone(), &[4, 5, 6][..]),
        result
    );
}
