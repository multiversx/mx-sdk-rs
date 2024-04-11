use crate::api::{ErrorApi, ManagedTypeApi};

use super::CallbackClosureForDeser;

/// Used internally between the `callback` and `callback_selector` methods.
/// It is likely to be removed in the future.
pub enum CallbackSelectorResult<'a, A>
where
    A: ManagedTypeApi<'a> + ErrorApi,
{
    Processed,
    NotProcessed(CallbackClosureForDeser<'a, A>),
}
