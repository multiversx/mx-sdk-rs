use crate::{
    abi::TypeAbi,
    api::{EndpointFinishApi, ManagedTypeApi},
    io::{ArgId, ContractCallArg, DynArg, DynArgInput},
    DynArgOutput, EndpointResult,
};
use alloc::string::String;

/// Structure that allows taking a variable number of arguments,
/// but does nothing with them, not even deserialization.
#[derive(Default, Clone)]
pub struct IgnoreVarArgs;

impl DynArg for IgnoreVarArgs {
    fn dyn_load<I: DynArgInput>(loader: &mut I, _arg_id: ArgId) -> Self {
        loader.flush_ignore();
        IgnoreVarArgs
    }
}

impl EndpointResult for IgnoreVarArgs {
    type DecodeAs = IgnoreVarArgs;

    #[inline]
    fn finish<FA>(&self, _api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
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
