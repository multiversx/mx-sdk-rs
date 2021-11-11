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
    pub int1: u32,
    pub int2: u32,
}

#[test]
fn struct_with_numbers_static() {
    assert_eq!(
        <StructWithNumbers as elrond_wasm::types::ManagedVecItem<DebugApi>>::PAYLOAD_SIZE,
        8
    );
    assert!(
        <StructWithNumbers as elrond_wasm::types::ManagedVecItem<DebugApi>>::SKIPS_RESERIALIZATION
    );
}

#[test]
fn struct_named_fields_test() {
    let s = StructWithNumbers {
        int1: 0x42,
        int2: 0x42,
    };

    #[rustfmt::skip]
	let bytes_1 = &[
		/* int1 */ 0, 0, 0, 0x42,
		/* int2 */ 0, 0, 0, 0x42,
	];

    check_top_encode_decode(s.clone(), bytes_1);
    check_dep_encode_decode(s.clone(), bytes_1);
}

#[test]
fn struct_to_bytes_writer() {
    let s = StructWithNumbers {
        int1: 0x42,
        int2: 0x42,
    };
    let mut arr: [u8; 8] =
        [0u8; <StructWithNumbers as elrond_wasm::types::ManagedVecItem<DebugApi>>::PAYLOAD_SIZE];

    <StructWithNumbers as elrond_wasm::types::ManagedVecItem<DebugApi>>::to_byte_writer(
        &s,
        |bytes| {
            arr[0..<StructWithNumbers as elrond_wasm::types::ManagedVecItem<DebugApi>>::PAYLOAD_SIZE].copy_from_slice(bytes);
            assert_eq!(arr, [0x00, 0x00, 0x00, 0x42, 0x00, 0x00, 0x00, 0x42]);
        },
    );
}

#[test]
fn struct_from_bytes_reader() {
    let s = StructWithNumbers {
        int1: 0x42,
        int2: 0x42,
    };
    let arr: [u8; 8] = [0x00, 0x00, 0x00, 0x42, 0x00, 0x00, 0x00, 0x42];

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
