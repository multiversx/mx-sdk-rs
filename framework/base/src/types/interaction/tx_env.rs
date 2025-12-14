use crate::{
    api::CallTypeApi,
    types::{ManagedAddress, ManagedBuffer, heap::H256},
};

use super::{AnnotatedValue, TxFromSpecified};

pub trait TxEnv: Sized {
    type Api: CallTypeApi;

    /// Type built by result handlers that translates into the "expect" section in scenarios.
    type RHExpect: Default;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api>;

    fn default_gas_annotation(&self) -> ManagedBuffer<Self::Api>;

    fn default_gas_value(&self) -> u64;
}

pub trait TxEnvMockDeployAddress: TxEnv {
    fn mock_deploy_new_address<From, NA>(&mut self, from: &From, new_address: NA)
    where
        From: TxFromSpecified<Self>,
        NA: AnnotatedValue<Self, ManagedAddress<Self::Api>>;
}

pub trait TxEnvWithTxHash: TxEnv {
    fn set_tx_hash(&mut self, tx_hash: H256);

    /// Retrieves current tx hash, while resetting it in self.
    fn take_tx_hash(&mut self) -> Option<H256>;
}
