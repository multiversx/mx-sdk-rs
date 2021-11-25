use elrond_wasm::elrond_codec::test_util::{check_dep_encode_decode, check_top_encode_decode};
use elrond_wasm_debug::DebugApi;

elrond_wasm::derive_imports!();

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
    pub bool_field: bool,
}

#[test]
fn struct_with_numbers_static() {
    assert_eq!(
        <StructWithNumbers as elrond_wasm::types::ManagedVecItem>::PAYLOAD_SIZE,
        16
    );
    assert!(<StructWithNumbers as elrond_wasm::types::ManagedVecItem>::SKIPS_RESERIALIZATION);
}

#[test]
fn struct_named_fields_test() {
    let s = StructWithNumbers {
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
    check_dep_encode_decode(s.clone(), bytes_1);
}

#[test]
fn struct_to_bytes_writer() {
    let s = StructWithNumbers {
        u_8: 1u8,
        u_16: 2u16,
        u_32: 3u32,
        u_64: 4u64,
        bool_field: true,
    };
    let mut arr: [u8; 16] =
        [0u8; <StructWithNumbers as elrond_wasm::types::ManagedVecItem>::PAYLOAD_SIZE];

    <StructWithNumbers as elrond_wasm::types::ManagedVecItem>::to_byte_writer(&s, |bytes| {
        arr[0..<StructWithNumbers as elrond_wasm::types::ManagedVecItem>::PAYLOAD_SIZE]
            .copy_from_slice(bytes);
        assert_eq!(
            arr,
            [
                0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x04, 0x01,
            ]
        );
    });
}

#[test]
fn struct_from_bytes_reader() {
    let _ = DebugApi::dummy();
    let s = StructWithNumbers {
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
        <StructWithNumbers as elrond_wasm::types::ManagedVecItem>::from_byte_reader(|bytes| {
            bytes.copy_from_slice(
                &arr[0..<StructWithNumbers as elrond_wasm::types::ManagedVecItem>::PAYLOAD_SIZE],
            );
        });
    assert_eq!(s, struct_from_bytes);
}
