use crate::api::{ErrorApi, ManagedTypeApi, ManagedTypeErrorApi};

use super::CallbackClosureForDeser;

/// Used internally between the `callback` and `callback_selector` methods.
/// It is likely to be removed in the future.
pub enum CallbackSelectorResult<A>
where
    A: ManagedTypeErrorApi,
{
    Processed,
    NotProcessed(CallbackClosureForDeser<A>),
}
