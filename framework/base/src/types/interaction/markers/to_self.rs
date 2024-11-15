use crate::{
    api::{const_handles, BlockchainApi, BlockchainApiImpl, CallTypeApi},
    contract_base::BlockchainWrapper,
    types::{
        AnnotatedValue, ManagedAddress, ManagedBuffer, ManagedType, TxScEnv, TxTo, TxToSpecified,
    },
};

/// Indicates that transaction should be sent to itself.
pub struct ToSelf;

impl<Api> AnnotatedValue<TxScEnv<Api>, ManagedAddress<Api>> for ToSelf
where
    Api: CallTypeApi + BlockchainApi,
{
    fn annotation(&self, env: &TxScEnv<Api>) -> ManagedBuffer<Api> {
        self.with_address_ref(env, |addr_ref| addr_ref.hex_expr())
    }

    #[inline]
    fn to_value(&self, _env: &TxScEnv<Api>) -> ManagedAddress<Api> {
        BlockchainWrapper::<Api>::new().get_sc_address()
    }

    fn with_value_ref<F, R>(&self, _env: &TxScEnv<Api>, f: F) -> R
    where
        F: FnOnce(&ManagedAddress<Api>) -> R,
    {
        unsafe {
            let temp = ManagedAddress::temp_const_ref(const_handles::ADDRESS_CALLER);
            Api::blockchain_api_impl().load_sc_address_managed(temp.get_handle());
            f(&temp)
        }
    }
}

impl<Api> TxTo<TxScEnv<Api>> for ToSelf where Api: CallTypeApi + BlockchainApi {}
impl<Api> TxToSpecified<TxScEnv<Api>> for ToSelf where Api: CallTypeApi + BlockchainApi {}
