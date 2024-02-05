use core::marker::PhantomData;

use crate::{
    api::{BlockchainApiImpl, CallTypeApi},
    contract_base::BlockchainWrapper,
    types::{ManagedAddress, ManagedBuffer},
};

use super::{
    contract_call_exec::TRANSFER_EXECUTE_DEFAULT_LEFTOVER, AnnotatedValue, AsyncCall, ExplicitGas,
    FunctionCall, Tx, TxBaseWithEnv, TxData, TxEnv, TxFrom, TxGas, TxPayment, TxToSpecified,
};

pub struct TxScEnv<Api>
where
    Api: CallTypeApi,
{
    _phantom: PhantomData<Api>,
}

impl<Api> Default for TxScEnv<Api>
where
    Api: CallTypeApi,
{
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Api> TxBaseWithEnv<TxScEnv<Api>>
where
    Api: CallTypeApi,
{
    pub fn new_tx_from_sc() -> Self {
        Tx::new_with_env(TxScEnv::default())
    }
}

impl<Api> TxEnv for TxScEnv<Api>
where
    Api: CallTypeApi,
{
    type Api = Api;

    fn annotate_from<From>(&mut self, _from: &From)
    where
        From: AnnotatedValue<Self, ManagedAddress<Api>>,
    {
    }

    fn annotate_to<To>(&mut self, _to: &To)
    where
        To: AnnotatedValue<Self, ManagedAddress<Api>>,
    {
    }

    fn resolve_sender_address(&self) -> ManagedAddress<Api> {
        BlockchainWrapper::<Api>::new().get_sc_address()
    }

    fn default_gas(&self) -> u64 {
        let mut gas_left = Api::blockchain_api_impl().get_gas_left();
        if gas_left > TRANSFER_EXECUTE_DEFAULT_LEFTOVER {
            gas_left -= TRANSFER_EXECUTE_DEFAULT_LEFTOVER;
        }
        gas_left
    }
}
