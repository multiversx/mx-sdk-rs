use core::marker::PhantomData;

use crate::{
    abi::{TypeAbi, TypeName},
    api::{CallTypeApi, ManagedTypeApi},
    types::{EsdtTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec, MultiValueEncoded},
};

use super::{ContractCallNoPayment, FunctionCall, ManagedArgBuffer};

/// Encodes a function call on the blockchain, together with the original result type.
///
/// Can be used as a multi-argument, to embed a call within a call.
pub struct TypedFunctionCall<Api, OriginalResult>
where
    Api: ManagedTypeApi,
{
    pub function_call: FunctionCall<Api>,
    _return_type: PhantomData<OriginalResult>,
}

impl<Api, OriginalResult> From<FunctionCall<Api>> for TypedFunctionCall<Api, OriginalResult>
where
    Api: ManagedTypeApi,
{
    fn from(function_call: FunctionCall<Api>) -> Self {
        TypedFunctionCall {
            function_call,
            _return_type: PhantomData,
        }
    }
}
