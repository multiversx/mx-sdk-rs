use multiversx_sc_scenario::multiversx_sc::codec::{
    EncodeErrorHandler, NestedEncode, NestedEncodeOutput, TopEncode, TopEncodeOutput,
};

use crate::AnyValue;

pub struct StructValue(Vec<StructField>);

pub struct StructField {
    pub name: String,
    pub value: AnyValue,
}

impl NestedEncode for StructValue {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        for field in &self.0 {
            field.value.dep_encode_or_handle_err(dest, h)?;
        }
        Ok(())
    }
}

impl TopEncode for StructValue {
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
