use crate::{abi::TypeAbi, io::ContractCallArg, DynArgOutput};
use alloc::string::String;
use elrond_codec::{
    DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput, TopEncodeMulti,
    TopEncodeMultiOutput,
};

/// Structure that allows taking a variable number of arguments,
/// but does nothing with them, not even deserialization.
#[derive(Default, Clone)]
pub struct IgnoreVarArgs;

impl TopEncodeMulti for IgnoreVarArgs {
    type DecodeAs = Self;

    fn multi_encode_or_handle_err<O, H>(&self, _output: &mut O, _h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        Ok(())
    }
}

impl TopDecodeMulti for IgnoreVarArgs {
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        input.flush_ignore(h)?;
        Ok(IgnoreVarArgs)
    }
}

impl ContractCallArg for &IgnoreVarArgs {
    fn push_dyn_arg<O: DynArgOutput>(&self, _output: &mut O) {}
}

impl ContractCallArg for IgnoreVarArgs {
    fn push_dyn_arg<O: DynArgOutput>(&self, _output: &mut O) {}
}

impl TypeAbi for IgnoreVarArgs {
    fn type_name() -> String {
        String::from("ignore")
    }

    fn is_multi_arg_or_result() -> bool {
        true
    }
}
