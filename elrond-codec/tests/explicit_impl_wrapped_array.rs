use elrond_codec::{
    test_util::check_top_encode_decode, top_decode_from_nested_or_handle_err, DecodeErrorHandler,
    EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode,
    TopDecodeInput, TopEncode, TopEncodeOutput,
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct WrappedArray(pub [u8; 5]);

impl NestedEncode for WrappedArray {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        dest.write(&self.0[..]);
        Ok(())
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        _: ExitCtx,
        _: fn(ExitCtx, EncodeError) -> !,
    ) {
        dest.write(&self.0[..]);
    }
}

impl TopEncode for WrappedArray {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_slice_u8(&self.0[..]);
        Ok(())
    }

    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        _: ExitCtx,
        _: fn(ExitCtx, EncodeError) -> !,
    ) {
        output.set_slice_u8(&self.0[..]);
    }
}

impl NestedDecode for WrappedArray {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let mut arr = [0u8; 5];
        input.read_into_or_handle_err(&mut arr, h)?;
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

    let mut v: Vec<WrappedArray> = Vec::new();
    v.push(wa);
    v.push(WrappedArray([6, 7, 8, 9, 0]));
    check_top_encode_decode(v, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
}
