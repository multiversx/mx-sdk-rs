use core::marker::PhantomData;

use crate::types::{TxEmptyResultHandler, TxEnv, TxResultHandler};

/// Contains no data.
///
/// Indicates to the compiler the original result type expected from a transaction.
///
/// Note that the transaction result might be interpreted as a different type,
/// but the originally declared type is required to perform any type checking.
pub struct OriginalResultMarker<O> {
    _phantom: PhantomData<O>,
}

impl<O> Default for OriginalResultMarker<O> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<O> OriginalResultMarker<O> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Env, O> TxResultHandler<Env> for OriginalResultMarker<O>
where
    Env: TxEnv,
{
    type OriginalResult = O;
}

impl<Env, O> TxEmptyResultHandler<Env> for OriginalResultMarker<O> where Env: TxEnv {}
