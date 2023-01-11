use multiversx_sc_codec as codec;

use codec::{
    test_util::check_top_encode_decode, top_decode_from_nested_or_handle_err, DecodeErrorHandler,
    EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct WrappedArray(pub [u8; 5]);

impl NestedEncode for WrappedArray {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, _h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        dest.write(&self.0[..]);
        Ok(())
    }
}

impl TopEncode for WrappedArray {
    fn top_encode_or_handle_err<O, H>(&self, output: O, _h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        output.set_slice_u8(&self.0[..]);
        Ok(())
    }
}

impl NestedDecode for WrappedArray {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let mut arr = [0u8; 5];
        input.read_into(&mut arr, h)?;
        Ok(WrappedArray(arr))
    }
}

impl TopDecode for WrappedArray {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        top_decode_from_nested_or_handle_err(input, h)
    }
}

#[test]
fn test_top() {
    let wa = WrappedArray([1, 2, 3, 4, 5]);
    check_top_encode_decode(wa, &[1, 2, 3, 4, 5]);

    let v: Vec<WrappedArray> = vec![wa, WrappedArray([6, 7, 8, 9, 0])];
    check_top_encode_decode(v, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
}
