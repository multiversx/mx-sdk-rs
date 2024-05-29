use crate::{
    api::{const_handles, use_raw_handle, BlockchainApi, BlockchainApiImpl, CallTypeApi},
    contract_base::BlockchainWrapper,
    types::{
        AnnotatedValue, ManagedAddress, ManagedBuffer, ManagedType, TxScEnv, TxTo, TxToSpecified,
    },
};

/// Indicates that transaction should be sent to the caller (the sender of the current transaction).
pub struct ToCaller;

impl<Api> AnnotatedValue<TxScEnv<Api>, ManagedAddress<Api>> for ToCaller
where
    Api: CallTypeApi + BlockchainApi,
{
    fn annotation(&self, env: &TxScEnv<Api>) -> ManagedBuffer<Api> {
        self.with_address_ref(env, |addr_ref| addr_ref.hex_expr())
    }

    fn to_value(&self, _env: &TxScEnv<Api>) -> ManagedAddress<Api> {
        BlockchainWrapper::<Api>::new().get_caller()
    }

    fn with_value_ref<F, R>(&self, _env: &TxScEnv<Api>, f: F) -> R
    where
        F: FnOnce(&ManagedAddress<Api>) -> R,
    {
        let caller_handle: Api::ManagedBufferHandle = use_raw_handle(const_handles::ADDRESS_CALLER);
        Api::blockchain_api_impl().load_caller_managed(caller_handle.clone());
        f(&ManagedAddress::from_handle(caller_handle))
    }
}

impl<Api> TxTo<TxScEnv<Api>> for ToCaller where Api: CallTypeApi + BlockchainApi {}
impl<Api> TxToSpecified<TxScEnv<Api>> for ToCaller where Api: CallTypeApi + BlockchainApi {}
