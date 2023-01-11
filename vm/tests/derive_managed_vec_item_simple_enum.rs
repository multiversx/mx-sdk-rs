use multiversx_sc::{
    codec,
    codec::derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    derive::ManagedVecItem,
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
        <SimpleEnum as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE,
        1
    );
    assert!(<SimpleEnum as multiversx_sc::types::ManagedVecItem>::SKIPS_RESERIALIZATION);
}

#[test]
fn enum_to_bytes_writer() {
    <SimpleEnum as multiversx_sc::types::ManagedVecItem>::to_byte_writer(
        &SimpleEnum::Variant1,
        |bytes| {
            assert_eq!(bytes.len(), 1);
            assert_eq!(bytes[0], 0);
        },
    );
}

#[test]
fn enum_from_bytes_reader() {
    let enum_from_bytes =
        <SimpleEnum as multiversx_sc::types::ManagedVecItem>::from_byte_reader(|bytes| {
            assert_eq!(bytes.len(), 1);
            bytes[0] = 1;
        });
    assert_eq!(enum_from_bytes, SimpleEnum::Variant2);
}
