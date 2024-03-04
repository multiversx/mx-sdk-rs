use crate::{api::CallTypeApi, types::ManagedAddress};

use super::AnnotatedValue;

pub trait TxEnv: Sized {
    type Api: CallTypeApi;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api>;

    fn default_gas(&self) -> u64;
}
