use elrond_wasm::derive::ManagedVecItem;
use elrond_wasm_debug::DebugApi;

// to test, run the following command in elrond-wasm-debug folder:
// cargo expand --test derive_managed_vec_item_test > expanded.rs

#[derive(ManagedVecItem)]
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

// #[test]
// fn struct_named_fields_test() {
//     let s = Struct {
//         int: 0x42,
//         seq: vec![0x1, 0x2, 0x3, 0x4, 0x5],
//         another_byte: 0x6,
//         uint_32: 0x12345,
//         uint_64: 0x123456789,
//     };

//     #[rustfmt::skip]
// 	let bytes_1 = &[
// 		/* int */ 0, 0x42,
// 		/* seq length */ 0, 0, 0, 5,
// 		/* seq contents */ 1, 2, 3, 4, 5,
// 		/* another_byte */ 6,
// 		/* uint_32 */ 0x00, 0x01, 0x23, 0x45,
// 		/* uint_64 */ 0x00, 0x00, 0x00, 0x01, 0x23, 0x45, 0x67, 0x89,
// 	];

//     check_top_encode_decode(s.clone(), bytes_1);
//     check_dep_encode_decode(s.clone(), bytes_1);
// }
