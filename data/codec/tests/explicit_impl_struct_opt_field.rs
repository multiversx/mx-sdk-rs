use multiversx_sc_codec as codec;

// Some structures with explicit encode/decode, for testing.
use codec::{
    DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, test_util::check_top_encode_decode,
};
use core::fmt::Debug;

/// Example of how to have an optional, or variable-length field at the end of a structure.
///
/// Careful! This is non-standard and is not really friendly to nested encoding.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StructOptField {
    pub field: u32,
    pub opt_items: Vec<u8>,
}

impl TopEncode for StructOptField {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        let mut nested_buffer = output.start_nested_encode();
        self.field.dep_encode_or_handle_err(&mut nested_buffer, h)?;
        for opt_item in &self.opt_items {
            opt_item.dep_encode_or_handle_err(&mut nested_buffer, h)?;
        }
        output.finalize_nested_encode(nested_buffer);
        Ok(())
    }
}

impl TopDecode for StructOptField {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let mut nested_buffer = input.into_nested_buffer();
        let field = u32::dep_decode_or_handle_err(&mut nested_buffer, h)?;
        let mut opt_items = Vec::new();
        while !nested_buffer.is_depleted() {
            opt_items.push(u8::dep_decode_or_handle_err(&mut nested_buffer, h)?);
        }
        Ok(StructOptField { field, opt_items })
    }
}

#[test]
fn test_struct_opt_field_1() {
    let test = StructOptField {
        field: 0x01020304,
        opt_items: vec![],
    };
    check_top_encode_decode(test, &[1, 2, 3, 4]);
}

#[test]
fn test_struct_opt_field_2() {
    let test = StructOptField {
        field: 0x01020304,
        opt_items: vec![5, 6, 7],
    };
    check_top_encode_decode(test, &[1, 2, 3, 4, 5, 6, 7]);
}
