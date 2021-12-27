use super::{ErrorApi, ManagedTypeApi, SendApi};

/// Provided for convenience.
/// Designed to be used in any types that send tokens or calls.
pub trait CallTypeApi: SendApi + ManagedTypeApi + ErrorApi {}
