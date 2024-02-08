use multiversx_sc_codec::TopEncodeMulti;

use crate::{
    contract_base::BlockchainWrapper,
    types::{
        BigUint, EgldPayment, EsdtTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec,
        MultiEsdtPayment,
    },
};

use super::{
    ExplicitGas, FunctionCall, ManagedArgBuffer, OriginalResultMarker, RHList, RHListAppendNoRet,
    RHListAppendRet, RHListItem, TxData, TxDataFunctionCall, TxEnv, TxFrom, TxFromSpecified, TxGas,
    TxPayment, TxResultHandler, TxTo, TxToSpecified,
};

#[must_use]
pub struct Tx<Env, From, To, Payment, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    pub env: Env,
    pub from: From,
    pub to: To,
    pub payment: Payment,
    pub gas: Gas,
    pub data: Data,
    pub result_handler: RH,
}

impl<Env, From, To, Payment, Gas, Data, RH> Tx<Env, From, To, Payment, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// TODO: does nothing, delete, added for easier copy-paste.
    #[inline]
    pub fn nothing(self) -> Tx<Env, From, To, Payment, Gas, Data, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler,
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
            result_handler: (),
        }
    }
}

impl<Env, To, Payment, Gas, Data, RH> Tx<Env, (), To, Payment, Gas, Data, RH>
where
    Env: TxEnv,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn from<From>(self, from: From) -> Tx<Env, From, To, Payment, Gas, Data, RH>
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
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, Payment, Gas, Data, RH> Tx<Env, From, (), Payment, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn to<To>(self, to: To) -> Tx<Env, From, To, Payment, Gas, Data, RH>
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
            result_handler: self.result_handler,
        }
    }

    pub fn to_caller(self) -> Tx<Env, From, ManagedAddress<Env::Api>, Payment, Gas, Data, RH> {
        let caller = BlockchainWrapper::<Env::Api>::new().get_caller();
        self.to(caller)
    }
}

impl<Env, From, To, Gas, Data, RH> Tx<Env, From, To, (), Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn egld(
        self,
        egld_amount: BigUint<Env::Api>,
    ) -> Tx<Env, From, To, EgldPayment<Env::Api>, Gas, Data, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: EgldPayment { value: egld_amount },
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Gas, Data, RH> Tx<Env, From, To, (), Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// Adds a single ESDT token transfer to a transaction.
    ///
    /// Since this is the first ESDT payment, a single payment tx is produced. Can be called again for multiple payments.
    pub fn esdt<P: Into<EsdtTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, EsdtTokenPayment<Env::Api>, Gas, Data, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: payment.into(),
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler,
        }
    }

    /// Adds a collection of ESDT payments to a transaction.
    pub fn multi_esdt(
        self,
        payments: MultiEsdtPayment<Env::Api>, // TODO: references
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: payments,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Gas, Data, RH> Tx<Env, From, To, EsdtTokenPayment<Env::Api>, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// Adds a single ESDT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_esdt_transfer<P: Into<EsdtTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, RH> {
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
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Gas, Data, RH> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// Adds a single ESDT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_esdt_transfer<P: Into<EsdtTokenPayment<Env::Api>>>(
        mut self,
        payment: P,
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, RH> {
        self.payment.push(payment.into());
        self
    }
}

impl<Env, From, To, Payment, Data, RH> Tx<Env, From, To, Payment, (), Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// Sets an explicit gas limit to the call.
    #[inline]
    pub fn with_gas_limit(
        self,
        gas_limit: u64,
    ) -> Tx<Env, From, To, Payment, ExplicitGas, Data, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: ExplicitGas(gas_limit),
            data: self.data,
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Payment, Gas> Tx<Env, From, To, Payment, Gas, (), ()>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
{
    #[inline]
    pub fn call<FC: Into<FunctionCall<Env::Api>>>(
        self,
        call: FC,
    ) -> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, ()> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: call.into(),
            result_handler: (),
        }
    }

    #[inline]
    pub fn function_name<N: Into<ManagedBuffer<Env::Api>>>(
        self,
        function_name: N,
    ) -> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, ()> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: FunctionCall::new(function_name),
            result_handler: (),
        }
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    #[inline]
    pub fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.data = self.data.argument(arg);
        self
    }

    #[inline]
    pub fn arguments_raw(mut self, raw: ManagedArgBuffer<Env::Api>) -> Self {
        self.data.arg_buffer = raw;
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
    pub fn original_result<OriginalResult>(
        self,
    ) -> Tx<Env, From, To, Payment, Gas, Data, OriginalResultMarker<OriginalResult>> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: OriginalResultMarker::new(),
        }
    }
}

impl<Env, From, To, Payment, Gas, Data, ResultList>
    Tx<Env, From, To, Payment, Gas, Data, ResultList>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    ResultList: RHList<Env>,
{
    #[inline]
    pub fn with_result<ResultHandler>(
        self,
        result_handler: ResultHandler,
    ) -> Tx<Env, From, To, Payment, Gas, Data, ResultList::NoRetOutput>
    where
        ResultHandler: RHListItem<Env, ResultList::OriginalResult, Returns = ()>,
        ResultList: RHListAppendNoRet<Env, ResultHandler>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler.append_no_ret(result_handler),
        }
    }

    #[inline]
    pub fn returns<RH>(
        self,
        item: RH,
    ) -> Tx<Env, From, To, Payment, Gas, Data, ResultList::RetOutput>
    where
        RH: RHListItem<Env, ResultList::OriginalResult>,
        ResultList: RHListAppendRet<Env, RH>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler.append_ret(item),
        }
    }
}

impl<Env, From, To, Payment, Gas, FC, RH> Tx<Env, From, To, Payment, Gas, FC, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    FC: TxDataFunctionCall<Env>,
    RH: TxResultHandler<Env>,
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
        RH,
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
            result_handler: self.result_handler,
        }
    }
}
