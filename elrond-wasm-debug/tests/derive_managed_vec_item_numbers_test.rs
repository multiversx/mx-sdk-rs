use elrond_wasm::{
    derive::ManagedVecItem,
    elrond_codec,
    elrond_codec::{
        elrond_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
        test_util::{check_dep_encode_decode, check_top_encode_decode},
    },
};
use elrond_wasm_debug::DebugApi;

// to test, run the following command in elrond-wasm-debug folder:
// cargo expand --test derive_managed_vec_item_numbers_test > expanded.rs

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Clone, Debug,
)]
pub struct StructWithNumbers {
    pub u_8: u8,
    pub u_16: u16,
    pub u_32: u32,
    pub u_64: u64,
}

#[test]
fn struct_with_numbers_static() {
    assert_eq!(
        <StructWithNumbers as elrond_wasm::types::ManagedVecItem<DebugApi>>::PAYLOAD_SIZE,
        15
    );
    assert!(
        <StructWithNumbers as elrond_wasm::types::ManagedVecItem<DebugApi>>::SKIPS_RESERIALIZATION
    );
}

#[test]
fn struct_named_fields_test() {
    let s = StructWithNumbers {
        u_8: 1u8,
        u_16: 2u16,
        u_32: 3u32,
        u_64: 4u64,
    };

    #[rustfmt::skip]
	let bytes_1 = &[
		/* u_8 */  0x01,
		/* u_16 */ 0x00, 0x02,
		/* u_32 */ 0x00, 0x00, 0x00, 0x03,
		/* u_64 */ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
	];

    check_top_encode_decode(s.clone(), bytes_1);
    check_dep_encode_decode(s.clone(), bytes_1);
}

#[test]
fn struct_to_bytes_writer() {
    let s = StructWithNumbers {
        u_8: 1u8,
        u_16: 2u16,
        u_32: 3u32,
        u_64: 4u64,
    };
    let mut arr: [u8; 15] =
        [0u8; <StructWithNumbers as elrond_wasm::types::ManagedVecItem<DebugApi>>::PAYLOAD_SIZE];

    <StructWithNumbers as elrond_wasm::types::ManagedVecItem<DebugApi>>::to_byte_writer(
        &s,
        |bytes| {
            arr[0..<StructWithNumbers as elrond_wasm::types::ManagedVecItem<DebugApi>>::PAYLOAD_SIZE].copy_from_slice(bytes);
            assert_eq!(
                arr,
                [
                    0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x04
                ]
            );
        },
    );
}

#[test]
fn struct_from_bytes_reader() {
    let s = StructWithNumbers {
        u_8: 1u8,
        u_16: 2u16,
        u_32: 3u32,
        u_64: 4u64,
    };
    let arr: [u8; 15] = [
        0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
    ];

    let struct_from_bytes =
        <StructWithNumbers as elrond_wasm::types::ManagedVecItem<DebugApi>>::from_byte_reader(
            DebugApi::dummy(),
            |bytes| {
                bytes.copy_from_slice(
                    &arr
                        [0
                            ..<StructWithNumbers as elrond_wasm::types::ManagedVecItem<
                                DebugApi,
                            >>::PAYLOAD_SIZE],
                );
            },
        );
    assert_eq!(s, struct_from_bytes);
}
