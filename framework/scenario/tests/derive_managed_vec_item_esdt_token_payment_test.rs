use multiversx_sc::{
    api::ManagedTypeApi,
    codec,
    codec::derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    derive::ManagedVecItem,
    types::{BigUint, EsdtTokenPayment, ManagedByteArray, ManagedType, TokenIdentifier},
};
use multiversx_sc_scenario::api::StaticApi;

// to test, run the following command in the crate folder:
// cargo expand --test derive_managed_vec_item_esdt_token_payment_test > expanded.rs

const ETH_ADDR_WIDTH: usize = 20;

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug,
)]
pub struct ManagedStructWithToken<M: ManagedTypeApi> {
    pub token: multiversx_sc::types::EsdtTokenPayment<M>,
    pub num: u32,
    pub eth_address_1: ManagedByteArray<M, ETH_ADDR_WIDTH>,
    pub eth_address_2: ManagedByteArray<M, 20>, // const generic also works
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn struct_with_numbers_static() {
    assert_eq!(
        <ManagedStructWithToken<StaticApi> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE,
        28
    );
    assert!(
        !<ManagedStructWithToken<StaticApi> as multiversx_sc::types::ManagedVecItem>::SKIPS_RESERIALIZATION
    );
}

#[test]
fn struct_to_bytes_writer() {
    let s = ManagedStructWithToken::<StaticApi> {
        token: EsdtTokenPayment::new(
            TokenIdentifier::from("MYTOKEN-12345"),
            0u64,
            BigUint::from(42u64),
        ),
        num: 0x12345,
        eth_address_1: ManagedByteArray::new_from_bytes(&[1u8; 20]),
        eth_address_2: ManagedByteArray::new_from_bytes(&[2u8; 20]),
    };
    let mut arr: [u8; 28] = [0u8;
        <ManagedStructWithToken<StaticApi> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE];

    let handle1 = s.token.token_identifier.get_handle().to_be_bytes();
    let handle2 = s.token.amount.get_handle().to_be_bytes();
    let handle3 = s.eth_address_1.get_handle().to_be_bytes();
    let handle4 = s.eth_address_2.get_handle().to_be_bytes();
    let expected = [
        handle1[0], handle1[1], handle1[2], handle1[3], 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, handle2[0], handle2[1], handle2[2], handle2[3], 0x00, 0x01, 0x23, 0x45, handle3[0],
        handle3[1], handle3[2], handle3[3], handle4[0], handle4[1], handle4[2], handle4[3],
    ];

    <ManagedStructWithToken<StaticApi> as multiversx_sc::types::ManagedVecItem>::to_byte_writer(
        &s,
        |bytes| {
            arr[0
                ..<ManagedStructWithToken<StaticApi> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE]
                .copy_from_slice(bytes);

            assert_eq!(arr, expected);
        },
    );
}

#[test]
fn struct_from_bytes_reader() {
    let s = ManagedStructWithToken::<StaticApi> {
        token: EsdtTokenPayment::new(TokenIdentifier::from("MYTOKEN-12345"), 0u64, 42u64.into()),
        num: 0x12345,
        eth_address_1: ManagedByteArray::new_from_bytes(&[1u8; 20]),
        eth_address_2: ManagedByteArray::new_from_bytes(&[2u8; 20]),
    };

    let handle1 = s.token.token_identifier.get_handle().to_be_bytes();
    let handle2 = s.token.amount.get_handle().to_be_bytes();
    let handle3 = s.eth_address_1.get_handle().to_be_bytes();
    let handle4 = s.eth_address_2.get_handle().to_be_bytes();
    let arr: [u8; 28] = [
        handle1[0], handle1[1], handle1[2], handle1[3], 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, handle2[0], handle2[1], handle2[2], handle2[3], 0x00, 0x01, 0x23, 0x45, handle3[0],
        handle3[1], handle3[2], handle3[3], handle4[0], handle4[1], handle4[2], handle4[3],
    ];

    let struct_from_bytes =
        <ManagedStructWithToken<StaticApi> as multiversx_sc::types::ManagedVecItem>::from_byte_reader(
            |bytes| {
                bytes.copy_from_slice(
                    &arr
                        [0
                            ..<ManagedStructWithToken::<StaticApi> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE],
                );
            },
        );

    assert_eq!(s, struct_from_bytes);
}
