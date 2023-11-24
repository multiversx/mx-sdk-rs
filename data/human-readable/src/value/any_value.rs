use multiversx_sc_scenario::multiversx_sc::codec::{
    EncodeErrorHandler, NestedEncode, NestedEncodeOutput, TopEncode, TopEncodeOutput,
};

use crate::{EnumVariant, SingleValue, StructValue};

pub enum AnyValue {
    SingleValue(SingleValue),
    Struct(StructValue),
    Enum(Box<EnumVariant>),
}

impl NestedEncode for AnyValue {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        match self {
            AnyValue::SingleValue(sv) => sv.dep_encode_or_handle_err(dest, h),
            AnyValue::Struct(s) => s.dep_encode_or_handle_err(dest, h),
            AnyValue::Enum(_) => todo!(),
        }
    }
}

impl TopEncode for AnyValue {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        match self {
            AnyValue::SingleValue(sv) => sv.top_encode_or_handle_err(output, h),
            AnyValue::Struct(s) => s.top_encode_or_handle_err(output, h),
            AnyValue::Enum(_) => todo!(),
        }
    }
}
