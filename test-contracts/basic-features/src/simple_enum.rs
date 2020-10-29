use elrond_wasm::elrond_codec::*;

/// Copied from elrond-wasm serialization tests.
pub enum SimpleEnum {
    Variant0,
    Variant1,
    Variant2,
}

impl SimpleEnum {
    fn to_i64(&self) -> i64 {
        match self {
            SimpleEnum::Variant0 => 0,
            SimpleEnum::Variant1 => 1,
            SimpleEnum::Variant2 => 2,
        } 
    }

    fn from_i64(i: i64) -> Result<Self, DecodeError> {
        match i {
            0 => Ok(SimpleEnum::Variant0),
            1 => Ok(SimpleEnum::Variant1),
            2 => Ok(SimpleEnum::Variant2),
            _ => Err(DecodeError::INPUT_OUT_OF_RANGE),
        }
    }
}

impl NestedEncode for SimpleEnum {
    fn dep_encode_to<O: OutputBuffer>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.to_i64().dep_encode_to(dest)?;
        Ok(())
    }
}

impl TopEncode for SimpleEnum {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_i64(self.to_i64());
        Ok(())
    }
}

impl NestedDecode for SimpleEnum {
    fn dep_decode_to<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        SimpleEnum::from_i64(i64::dep_decode_to(input)?)
    }
}

impl TopDecode for SimpleEnum {
    fn top_decode<I: TopDecodeInput, R, F: FnOnce(Result<Self, DecodeError>) -> R>(input: I, f: F) -> R {
        top_decode_from_nested(input, f)
    }
}
