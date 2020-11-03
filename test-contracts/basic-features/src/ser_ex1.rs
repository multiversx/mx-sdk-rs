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
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.int.dep_encode(dest)?;
        self.seq.dep_encode(dest)?;
        self.another_byte.dep_encode(dest)?;
        self.uint_32.dep_encode(dest)?;
        self.uint_64.dep_encode(dest)?;
        Ok(())
    }
}

impl TopEncode for SerExample1 {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        top_encode_from_nested(self, output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(&self, output: O, c: ExitCtx, exit: fn(ExitCtx, EncodeError) -> !) {
        top_encode_from_nested_or_exit(self, output, c, exit);
    }
}

impl NestedDecode for SerExample1 {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
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
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        top_decode_from_nested(input)
    }
}
