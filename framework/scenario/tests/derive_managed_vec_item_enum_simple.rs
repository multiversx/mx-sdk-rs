use multiversx_sc::{
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    },
    derive::ManagedVecItem,
    types::{ManagedVecItemPayload, ManagedVecItemPayloadBuffer},
};

// to test, run the following command in the crate folder:
// cargo expand --test derive_managed_vec_item_simple_enum > expanded.rs

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug,
)]
enum SimpleEnum {
    Variant1,
    Variant2,
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn enum_static() {
    assert_eq!(
        <SimpleEnum as multiversx_sc::types::ManagedVecItem>::payload_size(),
        1
    );
    assert!(<SimpleEnum as multiversx_sc::types::ManagedVecItem>::SKIPS_RESERIALIZATION);
}

#[test]
fn enum_to_bytes_writer() {
    let mut payload = ManagedVecItemPayloadBuffer::new_buffer();
    <SimpleEnum as multiversx_sc::types::ManagedVecItem>::save_to_payload(
        SimpleEnum::Variant1,
        &mut payload,
    );

    assert_eq!(payload.into_array(), [0]);
}

#[test]
fn enum_from_bytes_reader() {

    <SimpleEnum as multiversx_sc::types::ManagedVecItem>::temp_decode(
        &[1u8].into(),
        |enum_from_bytes| {
            assert_eq!(enum_from_bytes, &SimpleEnum::Variant2);
        },
    );
}
