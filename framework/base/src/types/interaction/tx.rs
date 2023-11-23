use alloc::boxed::Box;
use multiversx_sc_codec::TopEncodeMulti;

use crate::{
    api::{self, CallTypeApi, ManagedTypeApi},
    contract_base::{BlockchainWrapper, SendRawWrapper},
    types::{
        BigUint, CodeMetadata, EgldPayment, EsdtTokenPayment, ManagedAddress, ManagedBuffer,
        ManagedVec, MultiEsdtPayment,
    },
};

use super::{
    AsyncCall, ExplicitGas, FunctionCall, ManagedArgBuffer, TxCallback, TxData, TxDataDeploy,
    TxDataFunctionCall, TxEnv, TxFrom, TxFromSpecified, TxGas, TxPayment, TxScEnv, TxTo,
    TxToSpecified,
};

#[must_use]
pub struct Tx<Env, From, To, Payment, Gas, Data, Callback>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    Callback: TxCallback<Env>,
{
    pub env: Env,
    pub from: From,
    pub to: To,
    pub payment: Payment,
    pub gas: Gas,
    pub data: Data,
    pub callback: Callback,
}

impl<Env, From, To, Payment, Gas, Data, Callback> Tx<Env, From, To, Payment, Gas, Data, Callback>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    Callback: TxCallback<Env>,
{
    /// TODO: does nothing, delete, added for easier copy-paste.
    #[inline]
    pub fn nothing(self) -> Tx<Env, From, To, Payment, Gas, Data, Callback> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            callback: self.callback,
        }
    }
}

pub type TxBaseWithEnv<Env> = Tx<Env, (), (), (), (), (), ()>;

impl<Env> TxBaseWithEnv<Env>
where
    Env: TxEnv,
{
    #[inline]
    pub fn new_with_env(env: Env) -> Self {
        Tx {
            env,
            from: (),
            to: (),
            payment: (),
            gas: (),
            data: (),
            callback: (),
        }
    }
}

impl<Env, To, Payment, Gas, Data, Callback> Tx<Env, (), To, Payment, Gas, Data, Callback>
where
    Env: TxEnv,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    Callback: TxCallback<Env>,
{
    pub fn from<From>(self, from: From) -> Tx<Env, From, To, Payment, Gas, Data, Callback>
    where
        From: TxFromSpecified<Env>,
    {
        let mut env = self.env;
        env.annotate_from(&from);
        Tx {
            env,
            from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            callback: self.callback,
        }
    }
}

impl<Env, From, Payment, Gas, Data, Callback> Tx<Env, From, (), Payment, Gas, Data, Callback>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    Callback: TxCallback<Env>,
{
    pub fn to<To>(self, to: To) -> Tx<Env, From, To, Payment, Gas, Data, Callback>
    where
        To: TxToSpecified<Env>,
    {
        let mut env = self.env;
        env.annotate_to(&to);
        Tx {
            env,
            from: self.from,
            to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            callback: self.callback,
        }
    }

    pub fn to_caller(
        self,
    ) -> Tx<Env, From, ManagedAddress<Env::Api>, Payment, Gas, Data, Callback> {
        let caller = BlockchainWrapper::<Env::Api>::new().get_caller();
        self.to(caller)
    }
}

impl<Env, From, To, Gas, Data, Callback> Tx<Env, From, To, (), Gas, Data, Callback>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    Callback: TxCallback<Env>,
{
    pub fn egld(
        self,
        egld_amount: BigUint<Env::Api>,
    ) -> Tx<Env, From, To, EgldPayment<Env::Api>, Gas, Data, Callback> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: EgldPayment { value: egld_amount },
            gas: self.gas,
            data: self.data,
            callback: self.callback,
        }
    }
}

impl<Env, From, To, Gas, Data, Callback> Tx<Env, From, To, (), Gas, Data, Callback>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    Callback: TxCallback<Env>,
{
    /// Adds a single ESDT token transfer to a transaction.
    ///
    /// Since this is the first ESDT payment, a single payment tx is produced. Can be called again for multiple payments.
    pub fn esdt<P: Into<EsdtTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, EsdtTokenPayment<Env::Api>, Gas, Data, Callback> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: payment.into(),
            gas: self.gas,
            data: self.data,
            callback: self.callback,
        }
    }

    /// Adds a collection of ESDT payments to a transaction.
    pub fn multi_esdt(
        self,
        payments: MultiEsdtPayment<Env::Api>, // TODO: references
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, Callback> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: payments,
            gas: self.gas,
            data: self.data,
            callback: self.callback,
        }
    }
}

impl<Env, From, To, Gas, Data, Callback>
    Tx<Env, From, To, EsdtTokenPayment<Env::Api>, Gas, Data, Callback>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    Callback: TxCallback<Env>,
{
    /// Adds a single ESDT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_esdt_transfer<P: Into<EsdtTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, Callback> {
        let mut payments = ManagedVec::new();
        payments.push(self.payment);
        payments.push(payment.into());
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: payments,
            gas: self.gas,
            data: self.data,
            callback: self.callback,
        }
    }
}

impl<Env, From, To, Gas, Data, Callback>
    Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, Callback>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    Callback: TxCallback<Env>,
{
    /// Adds a single ESDT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_esdt_transfer<P: Into<EsdtTokenPayment<Env::Api>>>(
        mut self,
        payment: P,
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, Callback> {
        self.payment.push(payment.into());
        self
    }
}

impl<Env, From, To, Payment, Data, Callback> Tx<Env, From, To, Payment, (), Data, Callback>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Data: TxData<Env>,
    Callback: TxCallback<Env>,
{
    /// Sets an explicit gas limit to the call.
    #[inline]
    pub fn with_gas_limit(
        self,
        gas_limit: u64,
    ) -> Tx<Env, From, To, Payment, ExplicitGas, Data, Callback> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: ExplicitGas(gas_limit),
            data: self.data,
            callback: self.callback,
        }
    }
}

impl<Env, From, To, Payment, Gas, Callback> Tx<Env, From, To, Payment, Gas, (), Callback>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Callback: TxCallback<Env>,
{
    #[inline]
    pub fn call<FC: Into<FunctionCall<Env::Api>>>(
        self,
        call: FC,
    ) -> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, Callback> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: call.into(),
            callback: self.callback,
        }
    }

    #[inline]
    pub fn function_name<N: Into<ManagedBuffer<Env::Api>>>(
        self,
        function_name: N,
    ) -> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, Callback> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: FunctionCall::new(function_name),
            callback: self.callback,
        }
    }
}

impl<Env, From, To, Payment, Gas, Callback>
    Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, Callback>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Callback: TxCallback<Env>,
{
    #[inline]
    pub fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.data = self.data.argument(arg);
        self
    }
}

impl<Env, From, To, Payment, Gas, Data> Tx<Env, From, To, Payment, Gas, Data, ()>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
{
    #[inline]
    pub fn callback<Callback>(
        self,
        callback: Callback,
    ) -> Tx<Env, From, To, Payment, Gas, Data, Callback>
    where
        Callback: TxCallback<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            callback,
        }
    }
}

impl<Env, From, To, Payment, Gas, FC, Callback> Tx<Env, From, To, Payment, Gas, FC, Callback>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    FC: TxDataFunctionCall<Env>,
    Callback: TxCallback<Env>,
{
    pub fn normalize_tx(
        self,
    ) -> Tx<
        Env,
        From,
        ManagedAddress<Env::Api>,
        EgldPayment<Env::Api>,
        Gas,
        FunctionCall<Env::Api>,
        Callback,
    > {
        let result = self.payment.convert_tx_data(
            &self.env,
            &self.from,
            self.to.into_value(),
            self.data.into(),
        );
        Tx {
            env: self.env,
            from: self.from,
            to: result.to,
            payment: result.egld_payment,
            gas: self.gas,
            data: result.fc,
            callback: self.callback,
        }
    }
}

impl<Env, Payment, Gas, Data> Tx<Env, (), (), Payment, Gas, Data, ()>
where
    Env: TxEnv,
    Payment: TxPayment<Env> + TxEgldOnlyPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
{
    pub fn deploy(self) -> Tx<Env, (), (), Payment, Gas, TxDataDeploy<Env>, ()> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: TxDataDeploy::default(),
            callback: self.callback,
        }
    }
}

impl<Env, Payment, Gas> Tx<Env, (), (), Payment, Gas, TxDataDeploy<Env>, ()>
where
    Env: TxEnv,
    Payment: TxPayment<Env> + TxEgldOnlyPayment<Env>,
    Gas: TxGas<Env>,
{
    pub fn code(
        self,
        code: ManagedBuffer<Env::Api>,
    ) -> Tx<Env, (), (), Payment, Gas, TxDataDeploy<Env>, ()> {
        let mut data_deploy = self.data;
        data_deploy.code = code;
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: data_deploy,
            callback: self.callback,
        }
    }

    pub fn code_metadata(
        self,
        code_metadata: CodeMetadata,
    ) -> Tx<Env, (), (), Payment, Gas, TxDataDeploy<Env>, ()> {
        let mut data_deploy = self.data;
        data_deploy.metadata = code_metadata;
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: data_deploy,
            callback: self.callback,
        }
    }

    pub fn arguments(
        self,
        arg_buffer: ManagedArgBuffer<Env::Api>,
    ) -> Tx<Env, (), (), Payment, Gas, TxDataDeploy<Env>, ()> {
        let mut data_deploy = self.data;
        data_deploy.arg_buffer = arg_buffer;
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: data_deploy,
            callback: self.callback,
        }
    }
}

pub trait TxEgldOnlyPayment<Env>
where
    Env: TxEnv,
    Self: Clone,
{
}
impl<Env> TxEgldOnlyPayment<Env> for EgldPayment<Env::Api> where Env: TxEnv {}

impl<Env> TxEgldOnlyPayment<Env> for () where Env: TxEnv {}

impl<Env, Payment, Gas> Tx<Env, (), (), Payment, Gas, TxDataDeploy<Env>, ()>
where
    Env: TxEnv,
    Payment: TxPayment<Env> + TxEgldOnlyPayment<Env>,
    Gas: TxGas<Env>,
{
    pub fn execute_deploy(
        &self,
    ) -> (
        ManagedAddress<Env::Api>,
        ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) {
        let result = self
            .payment
            .clone()
            .convert_tx_data(
                &self.env,
                &self.from,
                ManagedAddress::<Env::Api>::default(),
                FunctionCall {
                    function_name: "".into(),
                    arg_buffer: self.data.arg_buffer.clone(),
                },
            )
            .egld_payment
            .value;
        let wrap = SendRawWrapper::<Env::Api>::new();
        wrap.deploy_contract(
            self.gas.resolve_gas(&self.env),
            &result,
            &self.data.code,
            self.data.metadata,
            &self.data.arg_buffer,
        )
    }
}
