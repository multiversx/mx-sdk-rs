/// Used internally between the `callback` and `callback_selector` methods.
/// It is likely to be removed in the future.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CallbackSelectorResult {
    Processed,
    NotProcessed,
}

impl CallbackSelectorResult {
    pub fn is_processed(self) -> bool {
        matches!(self, CallbackSelectorResult::Processed)
    }
}
