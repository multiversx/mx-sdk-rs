use crate::{api::CallTypeApi, proxy_imports::ManagedBuffer, types::ManagedAddress};

pub trait TxEnv: Sized {
    type Api: CallTypeApi;

    /// Type built by result handlers that translates into the "expect" section in scenarios.
    type RHExpect: Default;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api>;

    fn default_gas_annotation(&self) -> ManagedBuffer<Self::Api>;

    fn default_gas_value(&self) -> u64;
}
