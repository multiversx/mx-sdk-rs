use core::marker::PhantomData;

use crate::{
    api::ManagedTypeApi,
    formatter::SCLowerHex,
    types::{ManagedBuffer, ManagedBufferCachedBuilder, ManagedVec},
};

use super::{FunctionCall, TxData, TxEnv, TxResultHandler, TxReturn};

pub struct ReturnTypeMarker<OriginalResult> {
    _return_type: PhantomData<OriginalResult>,
}

impl<OriginalResult> Default for ReturnTypeMarker<OriginalResult> {
    fn default() -> Self {
        Self {
            _return_type: PhantomData,
        }
    }
}

impl<Env, OriginalResult> TxResultHandler<Env> for ReturnTypeMarker<OriginalResult>
where
    Env: TxEnv,
{
    type OriginalResult = OriginalResult;
}
