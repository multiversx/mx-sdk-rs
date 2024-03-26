use crate::{api::CallTypeApi, types::ManagedAddress};

pub trait TxEnv: Sized {
    type Api: CallTypeApi;

    /// Type built by result handlers that translates into the "expect" section in scenarios.
    type RHExpect: Default;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api>;

    fn default_gas(&self) -> u64;
}
