use crate::HexCallDataDeserializer;

/// Used internally between the `callback` and `callback_selector` methods.
/// It is likely to be removed in the future.
pub enum CallbackSelectorResult<'a> {
	Processed,
	NotProcessed(HexCallDataDeserializer<'a>),
}
