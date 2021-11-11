use elrond_wasm::{
    api::ManagedTypeApi,
    derive::ManagedVecItem,
    elrond_codec,
    elrond_codec::elrond_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    types::{BigUint, ManagedFrom, ManagedType},
};
use elrond_wasm_debug::DebugApi;

// to test, run the following command in elrond-wasm-debug folder:
// cargo expand --test derive_managed_vec_item_2_test > expanded.rs

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Clone, Debug,
)]
pub struct ManagedStruct<M: ManagedTypeApi> {
    pub big_uint: elrond_wasm::types::BigUint<M>,
    pub num: u32,
}

#[test]
fn struct_with_numbers_static() {
    assert_eq!(
        <ManagedStruct<DebugApi> as elrond_wasm::types::ManagedVecItem<DebugApi>>::PAYLOAD_SIZE,
        8
    );
    assert!(
        !<ManagedStruct<DebugApi> as elrond_wasm::types::ManagedVecItem<DebugApi>>::SKIPS_RESERIALIZATION
    );
}

#[test]
fn struct_to_bytes_writer() {
    let s = ManagedStruct::<DebugApi> {
        big_uint: BigUint::managed_from(DebugApi::dummy(), 42u64),
        num: 0x12345,
    };
    let mut arr: [u8; 8] = [0u8; <ManagedStruct<DebugApi> as elrond_wasm::types::ManagedVecItem<
        DebugApi,
    >>::PAYLOAD_SIZE];

    <ManagedStruct<DebugApi> as elrond_wasm::types::ManagedVecItem<DebugApi>>::to_byte_writer(
        &s,
        |bytes| {
            arr[0..<ManagedStruct::<DebugApi> as elrond_wasm::types::ManagedVecItem<DebugApi>>::PAYLOAD_SIZE].copy_from_slice(bytes);

            assert_eq!(arr, [0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x23, 0x45]);
        },
    );

    assert_eq!(
        BigUint::<DebugApi>::from_raw_handle(DebugApi::dummy(), 0x0),
        42u64
    );
}

#[test]
fn struct_from_bytes_reader() {
    let s = ManagedStruct::<DebugApi> {
        big_uint: BigUint::managed_from(DebugApi::dummy(), 42u64),
        num: 0x12345,
    };
    let arr: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x23, 0x45];

    let struct_from_bytes = <ManagedStruct<DebugApi> as elrond_wasm::types::ManagedVecItem<
        DebugApi,
    >>::from_byte_reader(DebugApi::dummy(), |bytes| {
        bytes.copy_from_slice(
                    &arr
                        [0
                            ..<ManagedStruct::<DebugApi> as elrond_wasm::types::ManagedVecItem<
                                DebugApi,
                            >>::PAYLOAD_SIZE],
                );
    });
    assert_eq!(s.num, struct_from_bytes.num);
    assert_eq!(s.big_uint, struct_from_bytes.big_uint);
}
