use multiversx_sc::{
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    },
    derive::ManagedVecItem,
    types::{ManagedVecItemPayload, ManagedVecItemPayloadBuffer},
};

// to test, run the following command in the crate folder:
// cargo expand --test derive_managed_vec_item_enum_1 > expanded.rs

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug,
)]
enum EnumWithFields {
    Variant1(u32),
    Variant2,
    Variant3(i64),
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn enum_static() {
    assert_eq!(
        <EnumWithFields as multiversx_sc::types::ManagedVecItem>::payload_size(),
        9
    );
    assert!(!<EnumWithFields as multiversx_sc::types::ManagedVecItem>::SKIPS_RESERIALIZATION);
}

#[test]
fn enum_to_bytes_writer_variant_1() {
    let mut payload = ManagedVecItemPayloadBuffer::new_buffer();
    <EnumWithFields as multiversx_sc::types::ManagedVecItem>::save_to_payload(
        EnumWithFields::Variant1(7),
        &mut payload,
    );
    assert_eq!(payload.into_array(), [0, 0, 0, 0, 7, 0, 0, 0, 0]);
}

#[test]
fn enum_to_bytes_writer_variant_2() {
    let mut payload = ManagedVecItemPayloadBuffer::new_buffer();
    <EnumWithFields as multiversx_sc::types::ManagedVecItem>::save_to_payload(
        EnumWithFields::Variant2,
        &mut payload,
    );
    assert_eq!(payload.into_array(), [1, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn enum_to_bytes_writer_variant_3() {
    let mut payload = ManagedVecItemPayloadBuffer::new_buffer();
    <EnumWithFields as multiversx_sc::types::ManagedVecItem>::save_to_payload(
        EnumWithFields::Variant3(-3),
        &mut payload,
    );
    assert_eq!(
        payload.into_array(),
        [2, 255, 255, 255, 255, 255, 255, 255, 253]
    );
}

#[test]
fn enum_from_bytes_reader_variant_1() {
    let payload = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let enum_from_bytes =
        <EnumWithFields as multiversx_sc::types::ManagedVecItem>::read_from_payload(
            &payload.into(),
        );
    assert_eq!(enum_from_bytes, EnumWithFields::Variant1(0));
}

#[test]
fn enum_from_bytes_reader_variant_2() {
    let payload = [1, 0, 0, 0, 0, 0, 0, 0, 0];
    let enum_from_bytes =
        <EnumWithFields as multiversx_sc::types::ManagedVecItem>::read_from_payload(
            &payload.into(),
        );
    assert_eq!(enum_from_bytes, EnumWithFields::Variant2);
}

#[test]
fn enum_from_bytes_reader_variant_3() {
    let payload = [2, 0, 0, 0, 0, 0, 0, 0, 4];
    let enum_from_bytes =
        <EnumWithFields as multiversx_sc::types::ManagedVecItem>::read_from_payload(
            &payload.into(),
        );
    assert_eq!(enum_from_bytes, EnumWithFields::Variant3(4));
}
