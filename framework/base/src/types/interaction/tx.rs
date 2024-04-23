use crate::{
    api::CallTypeApi,
    types::{
        BigUint, CodeMetadata, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment,
        EgldOrEsdtTokenPaymentRefs, EgldOrMultiEsdtPayment, EsdtTokenPayment, EsdtTokenPaymentRefs,
        ManagedAddress, ManagedBuffer, ManagedOption, ManagedVec, MultiEsdtPayment,
        TokenIdentifier,
    },
};

use multiversx_sc_codec::TopEncodeMulti;

use super::{
    contract_deploy::UNSPECIFIED_GAS_LIMIT, AnnotatedValue, Code, ContractCallBase,
    ContractCallNoPayment, ContractCallWithEgld, ContractDeploy, DeployCall, Egld, EgldPayment,
    ExplicitGas, FromSource, FunctionCall, ManagedArgBuffer, OriginalResultMarker, RHList,
    RHListAppendNoRet, RHListAppendRet, RHListItem, TxCodeSource, TxCodeValue, TxData,
    TxDataFunctionCall, TxEgldValue, TxEnv, TxEnvMockDeployAddress, TxEnvWithTxHash, TxFrom,
    TxFromSourceValue, TxFromSpecified, TxGas, TxGasValue, TxPayment, TxPaymentEgldOnly,
    TxProxyTrait, TxResultHandler, TxScEnv, TxTo, TxToSpecified, UpgradeCall,
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

impl<Env, From, To, Payment, Gas, Data, RH> Tx<Env, From, To, Payment, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxDataFunctionCall<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn to_call_data_string(&self) -> ManagedBuffer<Env::Api> {
        self.data.to_call_data_string()
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
    /// Adds any payment to a transaction, if no payment has been added before.
    pub fn payment<Payment>(self, payment: Payment) -> Tx<Env, From, To, Payment, Gas, Data, RH>
    where
        Payment: TxPayment<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler,
        }
    }

    /// Adds EGLD value to a transaction.
    ///
    /// Accepts any type that can represent and EGLD amount: BigUint, &BigUint, etc.
    pub fn egld<EgldValue>(
        self,
        egld_value: EgldValue,
    ) -> Tx<Env, From, To, Egld<EgldValue>, Gas, Data, RH>
    where
        EgldValue: TxEgldValue<Env>,
    {
        self.payment(Egld(egld_value))
    }

    /// Backwards compatibility. Use method `egld` instead.
    pub fn with_egld_transfer(
        self,
        egld_amount: BigUint<Env::Api>,
    ) -> Tx<Env, From, To, EgldPayment<Env::Api>, Gas, Data, RH> {
        self.egld(egld_amount)
    }

    /// Adds the first single, owned ESDT token payment to a transaction.
    ///
    /// Since this is the first ESDT payment, a single payment tx is produced.
    ///
    /// Can subsequently be called again for multiple payments.
    pub fn esdt<P: Into<EsdtTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, EsdtTokenPayment<Env::Api>, Gas, Data, RH> {
        self.payment(payment.into())
    }

    /// Sets a single token payment, with the token identifier and amount kept as references.
    ///
    /// This is handy whem we only want one ESDT transfer and we want to avoid unnecessary object clones.
    pub fn single_esdt<'a>(
        self,
        token_identifier: &'a TokenIdentifier<Env::Api>,
        token_nonce: u64,
        amount: &'a BigUint<Env::Api>,
    ) -> Tx<Env, From, To, EsdtTokenPaymentRefs<'a, Env::Api>, Gas, Data, RH> {
        self.payment(EsdtTokenPaymentRefs {
            token_identifier,
            token_nonce,
            amount,
        })
    }

    /// Syntactic sugar for `self.payment(EgldOrEsdtTokenPaymentRefs::new(...)`. Takes references.
    pub fn egld_or_single_esdt<'a>(
        self,
        token_identifier: &'a EgldOrEsdtTokenIdentifier<Env::Api>,
        token_nonce: u64,
        amount: &'a BigUint<Env::Api>,
    ) -> Tx<Env, From, To, EgldOrEsdtTokenPaymentRefs<'a, Env::Api>, Gas, Data, RH> {
        self.payment(EgldOrEsdtTokenPaymentRefs::new(
            token_identifier,
            token_nonce,
            amount,
        ))
    }

    /// Sets a collection of ESDT transfers as the payment of the transaction.
    ///
    /// Can be formed from single ESDT payments, but the result will always be a collection.
    ///
    /// Always converts the argument into an owned collection of ESDT payments. For work with references, use `.payment(&p)` instead.
    pub fn multi_esdt<IntoMulti>(
        self,
        payments: IntoMulti,
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, RH>
    where
        IntoMulti: Into<MultiEsdtPayment<Env::Api>>,
    {
        self.payment(payments.into())
    }

    /// Backwards compatibility.
    pub fn with_esdt_transfer<P: Into<EsdtTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, RH> {
        self.payment(MultiEsdtPayment::new())
            .with_esdt_transfer(payment)
    }

    /// Backwards compatibility.
    pub fn with_multi_token_transfer(
        self,
        payments: MultiEsdtPayment<Env::Api>,
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, RH> {
        self.multi_esdt(payments)
    }

    /// Backwards compatibility.
    pub fn with_egld_or_single_esdt_transfer<P: Into<EgldOrEsdtTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, EgldOrEsdtTokenPayment<Env::Api>, Gas, Data, RH> {
        self.payment(payment.into())
    }

    pub fn egld_or_multi_esdt<P: Into<EgldOrMultiEsdtPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, EgldOrMultiEsdtPayment<Env::Api>, Gas, Data, RH> {
        self.payment(payment.into())
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
    /// Adds the second ESDT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    ///
    /// When the Tx already contains a single (owned) ESDT payment,
    /// adding the second one will convert it to a list.
    pub fn esdt<P: Into<EsdtTokenPayment<Env::Api>>>(
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
    pub fn esdt<P: Into<EsdtTokenPayment<Env::Api>>>(
        mut self,
        payment: P,
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, RH> {
        self.payment.push(payment.into());
        self
    }

    /// When the Tx already contains an owned collection of ESDT payments,
    /// calling `multi_esdt` is equivalent to `esdt`, it just adds another payment to the list.
    ///
    /// Can be called multiple times.
    pub fn multi_esdt<P: Into<EsdtTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, RH> {
        self.esdt(payment)
    }

    /// Backwards compatibility.
    pub fn with_esdt_transfer<P: Into<EsdtTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, MultiEsdtPayment<Env::Api>, Gas, Data, RH> {
        self.multi_esdt(payment)
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
    pub fn gas<GasValue>(
        self,
        gas_value: GasValue,
    ) -> Tx<Env, From, To, Payment, ExplicitGas<GasValue>, Data, RH>
    where
        GasValue: TxGasValue<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: ExplicitGas(gas_value),
            data: self.data,
            result_handler: self.result_handler,
        }
    }

    /// Backwards compatibility.
    #[inline]
    pub fn with_gas_limit(
        self,
        gas_limit: u64,
    ) -> Tx<Env, From, To, Payment, ExplicitGas<u64>, Data, RH> {
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
    pub fn raw_call<N: Into<ManagedBuffer<Env::Api>>>(
        self,
        function_name: N,
    ) -> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: FunctionCall::new(function_name),
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

impl<Env, From, To, Gas> Tx<Env, From, To, (), Gas, (), ()>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn typed<Proxy>(self, proxy: Proxy) -> Proxy::TxProxyMethods
    where
        Proxy: TxProxyTrait<Env, From, To, Gas>,
    {
        proxy.proxy_methods(self)
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
        self.payment.with_normalized(
            &self.env,
            &self.from,
            self.to,
            self.data,
            |norm_to, norm_egld, norm_fc| ContractCallWithEgld {
                basic: ContractCallNoPayment {
                    _phantom: core::marker::PhantomData,
                    to: norm_to.clone(),
                    function_call: norm_fc.clone(),
                    explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
                    _return_type: core::marker::PhantomData,
                },
                egld_payment: norm_egld.clone(),
            },
        )
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, (), RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn raw_deploy(self) -> Tx<Env, From, To, Payment, Gas, DeployCall<Env, ()>, RH> {
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

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, (), RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn raw_upgrade(self) -> Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, ()>, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: UpgradeCall::default(),
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, ()>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn code<CodeValue>(
        self,
        code: CodeValue,
    ) -> Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, Code<CodeValue>>, RH>
    where
        CodeValue: TxCodeValue<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data.code_source(Code(code)),
            result_handler: self.result_handler,
        }
    }

    pub fn from_source<FromSourceValue>(
        self,
        source_address: FromSourceValue,
    ) -> Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, FromSource<FromSourceValue>>, RH>
    where
        FromSourceValue: TxFromSourceValue<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data.code_source(FromSource(source_address)),
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, DeployCall<Env, ()>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn code<CodeValue>(
        self,
        code: CodeValue,
    ) -> Tx<Env, From, To, Payment, Gas, DeployCall<Env, Code<CodeValue>>, RH>
    where
        CodeValue: TxCodeValue<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data.code_source(Code(code)),
            result_handler: self.result_handler,
        }
    }

    pub fn from_source<FromSourceValue>(
        self,
        source_address: FromSourceValue,
    ) -> Tx<Env, From, To, Payment, Gas, DeployCall<Env, FromSource<FromSourceValue>>, RH>
    where
        FromSourceValue: TxFromSourceValue<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data.code_source(FromSource(source_address)),
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Payment, Gas, CodeSource, RH>
    Tx<Env, From, To, Payment, Gas, DeployCall<Env, CodeSource>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
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

impl<Env, From, To, Payment, Gas, CodeSource, RH>
    Tx<Env, From, To, Payment, Gas, DeployCall<Env, CodeSource>, RH>
where
    Env: TxEnvMockDeployAddress,
    From: TxFromSpecified<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    CodeSource: TxCodeSource<Env>,
    RH: TxResultHandler<Env>,
{
    /// Sets the new mock address to be used for the newly deployed contract.
    ///
    /// Only allowed in tests.
    pub fn new_address<NA>(mut self, new_address: NA) -> Self
    where
        NA: AnnotatedValue<Env, ManagedAddress<Env::Api>>,
    {
        self.env.mock_deploy_new_address(&self.from, new_address);
        self
    }
}

impl<Env, From, To, Payment, Gas, Data, RH> Tx<Env, From, To, Payment, Gas, Data, RH>
where
    Env: TxEnvWithTxHash,
    From: TxFromSpecified<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    Data: TxDataFunctionCall<Env>,
    RH: TxResultHandler<Env>,
{
    /// Sets the new mock address to be used for the newly deployed contract.
    ///
    /// Only allowed in tests.
    pub fn tx_hash<TH>(mut self, tx_hash: TH) -> Self
    where
        TH: AnnotatedValue<Env, ManagedBuffer<Env::Api>>,
    {
        self.env.set_tx_hash(tx_hash);
        self
    }
}

impl<Env, From, To, Payment, Gas, CodeSource, RH>
    Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, CodeSource>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
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
            egld_payment: value.payment.into_egld_payment(&value.env),
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            arg_buffer: value.data.arg_buffer,
            _return_type: core::marker::PhantomData,
        }
    }
}
