use multiversx_sc::{
    api::ManagedTypeApi,
    codec,
    codec::derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    derive::ManagedVecItem,
    types::{BigUint, ManagedType},
};
use multiversx_sc_scenario::api::StaticApi;

// to test, run the following command in the crate folder:
// cargo expand --test derive_managed_vec_item_biguint_test > expanded.rs

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug,
)]
pub struct ManagedStructWithBigUint<M: ManagedTypeApi> {
    pub big_uint: multiversx_sc::types::BigUint<M>,
    pub num: u32,
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn struct_with_numbers_static() {
    assert_eq!(
        <ManagedStructWithBigUint<StaticApi> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE,
        8
    );
    assert!(
        !<ManagedStructWithBigUint<StaticApi> as multiversx_sc::types::ManagedVecItem>::SKIPS_RESERIALIZATION
    );
}

#[test]
fn managed_struct_to_bytes_writer() {
    let fortytwo = 42u64;
    let s = ManagedStructWithBigUint::<StaticApi> {
        big_uint: BigUint::from(fortytwo),
        num: 0x12345,
    };
    let mut arr: [u8; 8] = [0u8;
        <ManagedStructWithBigUint<StaticApi> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE];

    let handle_bytes = s.big_uint.get_handle().to_be_bytes();
    let expected = [0xff, 0xff, 0xff, handle_bytes[3], 0x00, 0x01, 0x23, 0x45];

    <ManagedStructWithBigUint<StaticApi> as multiversx_sc::types::ManagedVecItem>::to_byte_writer(
        &s,
        |bytes| {
            arr[0..<ManagedStructWithBigUint::<StaticApi> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE].copy_from_slice(bytes);

            assert_eq!(arr, expected);
        },
    );
}

#[test]
fn managed_struct_from_bytes_reader() {
    let s = ManagedStructWithBigUint::<StaticApi> {
        big_uint: BigUint::from(42u64),
        num: 0x12345,
    };
    let handle_bytes = s.big_uint.get_handle().to_be_bytes();
    let arr: [u8; 8] = [0xff, 0xff, 0xff, handle_bytes[3], 0x00, 0x01, 0x23, 0x45];

    let struct_from_bytes =
        <ManagedStructWithBigUint<StaticApi> as multiversx_sc::types::ManagedVecItem>::from_byte_reader(
            |bytes| {
                bytes.copy_from_slice(
                    &arr
                        [0
                            ..<ManagedStructWithBigUint::<StaticApi> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE],
                );
            },
        );
    assert_eq!(s, struct_from_bytes);
}
