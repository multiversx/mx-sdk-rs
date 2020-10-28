use elrond_wasm::Vec;
use elrond_wasm::elrond_codec::*;

/// Copied from elrond-wasm serialization tests. 
pub struct SerExample1 {
    pub int: u16,
    pub seq: Vec<u8>,
    pub another_byte: u8,
    pub uint_32: u32,
    pub uint_64: u64,
}

impl NestedEncode for SerExample1 {
    fn dep_encode_to<O: OutputBuffer>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.int.dep_encode_to(dest)?;
        self.seq.dep_encode_to(dest)?;
        self.another_byte.dep_encode_to(dest)?;
        self.uint_32.dep_encode_to(dest)?;
        self.uint_64.dep_encode_to(dest)?;
        Ok(())
    }
}

impl TopEncode for SerExample1 {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_slice_u8(dep_encode_to_vec(self)?.as_slice());
        Ok(())
    }
}

impl NestedDecode for SerExample1 {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(SerExample1{
            int: u16::dep_decode(input)?,
            seq: Vec::<u8>::dep_decode(input)?,
            another_byte: u8::dep_decode(input)?,
            uint_32: u32::dep_decode(input)?,
            uint_64: u64::dep_decode(input)?,
        })
    }
}

impl TopDecode for SerExample1 {
    fn top_decode<I: TopDecodeInput, R, F: FnOnce(Result<Self, DecodeError>) -> R>(input: I, f: F) -> R {
        top_decode_from_nested(input, f)
    }
}
