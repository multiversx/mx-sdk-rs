use multiversx_sc::{
    codec::test_util::{check_dep_encode_decode, check_top_encode_decode},
    types::ManagedVecItemPayload,
};

multiversx_sc::derive_imports!();

// to test, run the following command in the crate folder:
// cargo expand --test derive_managed_vec_item_struct_1_test > expanded.rs

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug,
)]
pub struct Struct1 {
    pub u_8: u8,
    pub u_16: u16,
    pub u_32: u32,
    pub u_64: u64,
    pub bool_field: bool,
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn struct_1_static() {
    assert_eq!(
        <Struct1 as multiversx_sc::types::ManagedVecItem>::payload_size(),
        16
    );
    assert!(<Struct1 as multiversx_sc::types::ManagedVecItem>::SKIPS_RESERIALIZATION);
}

/// The reason we are including a codec test here is that because of the SKIPS_RESERIALIZATION flag,
/// serialization uses the payload as-is.
#[test]
fn struct_1_encode_decode_skips_reserialization() {
    let s = Struct1 {
        u_8: 1u8,
        u_16: 2u16,
        u_32: 3u32,
        u_64: 4u64,
        bool_field: true,
    };

    #[rustfmt::skip]
    let bytes_1 = &[
        /* u_8 */  0x01,
        /* u_16 */ 0x00, 0x02,
        /* u_32 */ 0x00, 0x00, 0x00, 0x03,
        /* u_64 */ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
        /* bool */ 0x01,
    ];

    check_top_encode_decode(s.clone(), bytes_1);
    check_dep_encode_decode(s, bytes_1);
}

#[test]
fn struct_1_to_bytes_writer() {
    let s = Struct1 {
        u_8: 1u8,
        u_16: 2u16,
        u_32: 3u32,
        u_64: 4u64,
        bool_field: true,
    };

    let mut payload = <Struct1 as multiversx_sc::types::ManagedVecItem>::PAYLOAD::new_buffer();
    let payload_slice = payload.payload_slice_mut();

    <Struct1 as multiversx_sc::types::ManagedVecItem>::to_byte_writer(&s, |bytes| {
        payload_slice.copy_from_slice(bytes);
        assert_eq!(
            payload_slice,
            [
                0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x04, 0x01,
            ]
        );
    });
}

#[test]
fn struct_1_from_bytes_reader() {
    let s = Struct1 {
        u_8: 1u8,
        u_16: 2u16,
        u_32: 3u32,
        u_64: 4u64,
        bool_field: false,
    };
    let arr: [u8; 16] = [
        0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
        0x00,
    ];

    let struct_from_bytes =
        <Struct1 as multiversx_sc::types::ManagedVecItem>::from_byte_reader(|bytes| {
            bytes.copy_from_slice(
                &arr[0..<Struct1 as multiversx_sc::types::ManagedVecItem>::payload_size()],
            );
        });
    assert_eq!(s, struct_from_bytes);
}
