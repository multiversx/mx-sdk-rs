use crate::{
    api::CallTypeApi,
    contract_base::BlockchainWrapper,
    types::{ManagedAddress, ManagedRef, TxScEnv, TxToInto},
};

/// Indicates that transaction should be sent to itself.
pub struct ToSelf;

impl<Api> TxToInto<TxScEnv<Api>> for ToSelf
where
    Api: CallTypeApi,
{
    type Into = ManagedRef<'static, Api, ManagedAddress<Api>>;

    fn into_recipient(self) -> Self::Into {
        BlockchainWrapper::<Api>::new().get_sc_address_ref()
    }
}
