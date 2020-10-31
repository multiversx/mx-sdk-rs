use elrond_wasm::Box;
use elrond_wasm::elrond_codec::*;

const ARRAY_SIZE: usize = 512;

pub struct LargeBoxedByteArray(Box<[u8; ARRAY_SIZE]>);

impl NestedEncode for LargeBoxedByteArray {
    #[inline]
    fn dep_encode_to<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.0.dep_encode_to(dest)
    }
}

impl TopEncode for LargeBoxedByteArray {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.0.top_encode(output)
    }
}

impl NestedDecode for LargeBoxedByteArray {
    #[inline]
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(LargeBoxedByteArray(Box::<[u8; ARRAY_SIZE]>::dep_decode(input)?))
    }
}

impl TopDecode for LargeBoxedByteArray {
    #[inline]
    fn top_decode<I: TopDecodeInput, R, F: FnOnce(Result<Self, DecodeError>) -> R>(input: I, f: F) -> R {
        Box::<[u8; ARRAY_SIZE]>::top_decode(input, |res| f(res.map(|v| LargeBoxedByteArray(v))))
    }
}
