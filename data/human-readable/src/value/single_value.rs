use multiversx_sc_scenario::multiversx_sc::codec::{
    num_bigint::{BigInt, BigUint},
    EncodeErrorHandler, NestedEncode, NestedEncodeOutput, TopEncode, TopEncodeOutput,
};

pub enum SingleValue {
    UnsignedNumber(BigUint),
    SignedNumber(BigInt),
    Bytes(Box<[u8]>),
    Bool(bool),
}

impl NestedEncode for SingleValue {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        match self {
            SingleValue::UnsignedNumber(bu) => bu.dep_encode_or_handle_err(dest, h),
            SingleValue::SignedNumber(bi) => bi.dep_encode_or_handle_err(dest, h),
            SingleValue::Bytes(bytes) => bytes.dep_encode_or_handle_err(dest, h),
            SingleValue::Bool(b) => b.dep_encode_or_handle_err(dest, h),
        }
    }
}

impl TopEncode for SingleValue {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        match self {
            SingleValue::UnsignedNumber(bu) => bu.top_encode_or_handle_err(output, h),
            SingleValue::SignedNumber(bi) => bi.top_encode_or_handle_err(output, h),
            SingleValue::Bytes(bytes) => bytes.top_encode_or_handle_err(output, h),
            SingleValue::Bool(b) => b.top_encode_or_handle_err(output, h),
        }
    }
}
