use crate::api::ManagedTypeApi;

use super::CallbackClosure;

/// Used internally between the `callback` and `callback_selector` methods.
/// It is likely to be removed in the future.
pub enum CallbackSelectorResult<A>
where
    A: ManagedTypeApi,
{
    Processed,
    NotProcessed(CallbackClosure<A>),
}
