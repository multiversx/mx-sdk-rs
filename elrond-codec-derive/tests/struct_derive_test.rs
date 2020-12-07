extern crate elrond_codec_derive;
use elrond_codec_derive::*;

use elrond_codec::*;

// to test, run the following command in elrond-codec-derive folder:
// cargo expand --test struct_derive_test > expanded.rs

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct SerExample1 {
	pub int: u16,
	pub seq: Vec<u8>,
	pub another_byte: u8,
	pub uint_32: u32,
	pub uint_64: u64,
}

trait SimpleTrait {
    fn simple_function(&self);
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
struct StructWithGeneric<ST: SimpleTrait> 
    where ST:NestedEncode+NestedDecode+TopEncode+TopDecode {

    data: u64,
    trait_stuff: ST,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
struct TupleStruct(u8, u16, u32);

#[test]
fn ser_example_1_test() {
    let ex = SerExample1 {
        int: 0x42,
        seq: vec![0x1, 0x2, 0x3, 0x4, 0x5],
        another_byte: 0x6,
        uint_32: 0x12345,
        uint_64: 0x123456789,
    };

    // ???
}