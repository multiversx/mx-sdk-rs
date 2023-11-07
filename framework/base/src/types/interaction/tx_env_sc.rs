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

impl<Api, To, Payment> Tx<TxScEnv<Api>, (), To, Payment, (), FunctionCall<Api>>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
{
    pub fn async_call(self) -> AsyncCall<Api> {
        let normalized = self.normalize_tx();
        AsyncCall {
            to: normalized.to,
            egld_payment: normalized.payment.value,
            function_call: normalized.data,
            callback_call: None,
        }
    }
}

impl<Api, To, Payment> Tx<TxScEnv<Api>, (), To, Payment, ExplicitGas, FunctionCall<Api>>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
{
    #[cfg(feature = "promises")]
    pub fn async_call_promise(self) -> super::AsyncCallPromises<Api> {
        let explicit_gas_limit = self.gas.0;
        let normalized = self.normalize_tx();
        super::AsyncCallPromises {
            to: normalized.to,
            egld_payment: normalized.payment.value,
            function_call: normalized.data,
            explicit_gas_limit,
            extra_gas_for_callback: 0,
            callback_call: None,
        }
    }
}

impl<Api, From, To, Payment, Gas, FC> Tx<TxScEnv<Api>, From, To, Payment, Gas, FC>
where
    Api: CallTypeApi,
    From: TxFrom<TxScEnv<Api>>,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxData<TxScEnv<Api>> + Into<FunctionCall<Api>>,
{
    pub fn transfer_execute(self) {
        if self.payment.is_no_payment() && self.data.is_no_call() {
            return;
        }

        let gas_limit = self.gas.resolve_gas(&self.env);

        self.to.with_value_ref(|to| {
            self.payment
                .perform_transfer_execute(&self.env, to, gas_limit, self.data.into());
        });
    }
}

impl<Api, From, To, Payment, Gas> Tx<TxScEnv<Api>, From, To, Payment, Gas, ()>
where
    Api: CallTypeApi,
    From: TxFrom<TxScEnv<Api>>,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
{
    /// Syntactic sugar, only allowed for simple transfers.
    pub fn transfer(self) {
        self.transfer_execute()
    }
}
