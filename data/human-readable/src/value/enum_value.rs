use multiversx_sc_scenario::multiversx_sc::codec::{
    EncodeErrorHandler, NestedEncode, NestedEncodeOutput, TopEncode, TopEncodeOutput,
};

use crate::AnyValue;

pub struct EnumVariant {
    pub discriminant: usize,
    pub value: AnyValue,
}

impl NestedEncode for EnumVariant {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        (self.discriminant as u8).dep_encode_or_handle_err(dest, h)?;
        self.value.dep_encode_or_handle_err(dest, h)?;
        Ok(())
    }
}

impl TopEncode for EnumVariant {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        let mut buffer = output.start_nested_encode();
        self.dep_encode_or_handle_err(&mut buffer, h)?;
        output.finalize_nested_encode(buffer);
        Ok(())
    }
}
