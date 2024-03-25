use crate::{
    api::{const_handles, use_raw_handle, BlockchainApi, BlockchainApiImpl, CallTypeApi},
    contract_base::BlockchainWrapper,
    types::{AnnotatedValue, ManagedAddress, ManagedBuffer, ManagedType, TxScEnv},
};

use super::{TxTo, TxToSpecified};

/// Indicates that transaction should be sent to itself.
pub struct ToSelf;

impl<Api> AnnotatedValue<TxScEnv<Api>, ManagedAddress<Api>> for ToSelf
where
    Api: CallTypeApi + BlockchainApi,
{
    fn annotation(&self, env: &TxScEnv<Api>) -> ManagedBuffer<Api> {
        self.with_address_ref(env, |addr_ref| addr_ref.hex_expr())
    }

    fn into_value(self, _env: &TxScEnv<Api>) -> ManagedAddress<Api> {
        BlockchainWrapper::<Api>::new().get_sc_address()
    }
}

impl<Api> TxTo<TxScEnv<Api>> for ToSelf where Api: CallTypeApi + BlockchainApi {}
impl<Api> TxToSpecified<TxScEnv<Api>> for ToSelf
where
    Api: CallTypeApi + BlockchainApi,
{
    fn with_address_ref<F, R>(&self, env: &TxScEnv<Api>, f: F) -> R
    where
        F: FnOnce(&ManagedAddress<Api>) -> R,
    {
        let sc_address_handle: Api::ManagedBufferHandle =
            use_raw_handle(const_handles::ADDRESS_CALLER);
        Api::blockchain_api_impl().load_sc_address_managed(sc_address_handle.clone());
        f(&ManagedAddress::from_handle(sc_address_handle))
    }
}
