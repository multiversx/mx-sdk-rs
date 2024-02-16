use crate::{
    api::{self, CallTypeApi, ManagedTypeApi},
    contract_base::{BlockchainWrapper, SendRawWrapper},
    types::{
        BigUint, CodeMetadata, EgldOrEsdtTokenPayment, EgldPayment, EsdtTokenPayment,
        ManagedAddress, ManagedBuffer, ManagedOption, ManagedVec, MultiEsdtPayment,
    },
};
use alloc::boxed::Box;
use multiversx_sc_codec::TopEncodeMulti;

use super::{
    contract_call_exec::UNSPECIFIED_GAS_LIMIT, contract_call_trait::ContractCallBase, AsyncCall,
    Code, ContractCallNoPayment, ContractCallWithEgld, ContractDeploy, DeployCall, ExplicitGas,
    FromSource, FunctionCall, ManagedArgBuffer, OriginalResultMarker, RHList, RHListAppendNoRet,
    RHListAppendRet, RHListItem, TxCodeSource, TxData, TxDataFunctionCall, TxEnv, TxFrom,
    TxFromSpecified, TxGas, TxPayment, TxPaymentEgldOnly, TxResultHandler, TxScEnv, TxTo,
    TxToSpecified,
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
        From: TxFrom<Env>,
    {
        Tx {
            env: self.env,
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
    /// Specifies the recipient of the transaction.
    ///
    /// Allows argument to also be `()`.
    pub fn to<To>(self, to: To) -> Tx<Env, From, To, Payment, Gas, Data, RH>
    where
        To: TxTo<Env>,
    {
        Tx {
            env: self.env,
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

    pub fn with_egld_transfer(
        self,
        egld_amount: BigUint<Env::Api>,
    ) -> Tx<Env, From, To, EgldPayment<Env::Api>, Gas, Data, RH> {
        self.egld(egld_amount)
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

    /// Backwards compatibility.
    pub fn with_multi_token_transfer(
        self,
        payments: MultiEsdtPayment<Env::Api>, // TODO: references
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, RH> {
        self.multi_esdt(payments)
    }

    pub fn egld_or_single_esdt<P: Into<EgldOrEsdtTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, EgldOrEsdtTokenPayment<Env::Api>, Gas, Data, RH> {
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

    /// Backwards compatibility.
    pub fn with_egld_or_single_esdt_transfer<P: Into<EgldOrEsdtTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, EgldOrEsdtTokenPayment<Env::Api>, Gas, Data, RH> {
        self.egld_or_single_esdt(payment)
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

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, (), RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    #[inline]
    pub fn raw_call(self) -> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: FunctionCall::empty(),
            result_handler: self.result_handler,
        }
    }

    #[inline]
    pub fn function_call(
        self,
        call: FunctionCall<Env::Api>,
    ) -> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: call,
            result_handler: self.result_handler,
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
    pub fn into_function_call(self) -> FunctionCall<Env::Api> {
        self.data
    }
}

impl<Env, From, Payment, Gas> Tx<Env, From, (), Payment, Gas, (), ()>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
{
    /// Merges the argument data into the current tx.
    /// Used for function calls originating in proxies.
    ///
    /// Different environment in the argument allowed because of compatibility with old proxies.
    ///
    /// Method still subject to considerable change.
    pub fn call<Env2, To, O>(
        self,
        call: Tx<Env2, (), To, (), (), FunctionCall<Env::Api>, OriginalResultMarker<O>>,
    ) -> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, OriginalResultMarker<O>>
    where
        Env2: TxEnv<Api = Env::Api>,
        To: TxTo<Env> + TxTo<Env2>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: call.to,
            payment: self.payment,
            gas: self.gas,
            data: call.data,
            result_handler: call.result_handler,
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
    pub fn function_name<N: Into<ManagedBuffer<Env::Api>>>(mut self, function_name: N) -> Self {
        self.data.function_name = function_name.into();
        self
    }

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

impl<Api, To, Payment, OriginalResult> ContractCallBase<Api>
    for Tx<
        TxScEnv<Api>,
        (),
        To,
        Payment,
        (),
        FunctionCall<Api>,
        OriginalResultMarker<OriginalResult>,
    >
where
    Api: CallTypeApi + 'static,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithEgld<Api, OriginalResult> {
        let normalized = self.normalize_tx();
        ContractCallWithEgld {
            basic: ContractCallNoPayment {
                _phantom: core::marker::PhantomData,
                to: normalized.to,
                function_call: normalized.data,
                explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
                _return_type: core::marker::PhantomData,
            },
            egld_payment: normalized.payment.value,
        }
    }
}

impl<Env, To, Payment, Gas, RH> Tx<Env, (), To, Payment, Gas, (), RH>
where
    Env: TxEnv,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn raw_deploy(self) -> Tx<Env, (), To, Payment, Gas, DeployCall<Env, ()>, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: DeployCall::default(),
            result_handler: self.result_handler,
        }
    }
}

impl<Env, To, Payment, Gas, RH> Tx<Env, (), To, Payment, Gas, DeployCall<Env, ()>, RH>
where
    Env: TxEnv,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn code(
        mut self,
        code: ManagedBuffer<Env::Api>,
    ) -> Tx<Env, (), To, Payment, Gas, DeployCall<Env, Code<Env>>, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data.code_source(Code::new(code)),
            result_handler: self.result_handler,
        }
    }

    pub fn from_source(
        mut self,
        source_address: ManagedAddress<Env::Api>,
    ) -> Tx<Env, (), To, Payment, Gas, DeployCall<Env, FromSource<Env>>, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data.code_source(FromSource::new(source_address)),
            result_handler: self.result_handler,
        }
    }
}

impl<Env, To, Payment, Gas, CodeSource, RH>
    Tx<Env, (), To, Payment, Gas, DeployCall<Env, CodeSource>, RH>
where
    Env: TxEnv,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    CodeSource: TxCodeSource<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn code_metadata(mut self, code_metadata: CodeMetadata) -> Self {
        self.data = self.data.code_metadata(code_metadata);
        self
    }

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

impl<Api, To, Payment, OriginalResult>
    From<
        Tx<
            TxScEnv<Api>,
            (),
            To,
            Payment,
            (),
            DeployCall<TxScEnv<Api>, ()>,
            OriginalResultMarker<OriginalResult>,
        >,
    > for ContractDeploy<Api, OriginalResult>
where
    Api: CallTypeApi + 'static,
    To: TxTo<TxScEnv<Api>>,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    OriginalResult: TopEncodeMulti,
{
    fn from(
        value: Tx<
            TxScEnv<Api>,
            (),
            To,
            Payment,
            (),
            DeployCall<TxScEnv<Api>, ()>,
            OriginalResultMarker<OriginalResult>,
        >,
    ) -> Self {
        ContractDeploy {
            _phantom: core::marker::PhantomData,
            to: ManagedOption::none(),
            egld_payment: value.payment.to_egld_payment().value,
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            arg_buffer: value.data.arg_buffer,
            _return_type: core::marker::PhantomData,
        }
    }
}