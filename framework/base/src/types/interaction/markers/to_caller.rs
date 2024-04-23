use crate::{
    api::CallTypeApi,
    contract_base::BlockchainWrapper,
    types::{ManagedAddress, ManagedRef, TxScEnv, TxToInto},
};

/// Indicates that transaction should be sent to the caller (the sender of the current transaction).
pub struct ToCaller;

impl<Api> TxToInto<TxScEnv<Api>> for ToCaller
where
    Api: CallTypeApi,
{
    type Into = ManagedRef<'static, Api, ManagedAddress<Api>>;

    fn into_recipient(self) -> Self::Into {
        BlockchainWrapper::<Api>::new().get_caller_ref()
    }
}
