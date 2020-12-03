extern crate elrond_codec_derive;
use elrond_codec_derive::*;

use elrond_codec::*;

// to test, run the following command in elrond-codec-derive folder:
// cargo expand --test struct_derive_test > expanded.rs

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
struct SimpleStruct {
    x: u8,
    y: u16,
    z: Vec<u8>
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
struct ComplexStruct {
    complex_name: Vec<u8>,
    complex_value: u64,
    complex_boolean: bool,
    complex_struct_field: SimpleStruct
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
