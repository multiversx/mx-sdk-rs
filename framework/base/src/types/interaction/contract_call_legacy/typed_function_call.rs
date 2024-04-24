use core::marker::PhantomData;

use crate::{api::ManagedTypeApi, types::FunctionCall};

/// Old attempt at grouping FunctionCall + OriginalTypeMarker.
#[deprecated(
    since = "0.49.0",
    note = "Not clear if it still used anywhere, will delete soon."
)]
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
