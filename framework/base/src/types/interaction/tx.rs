use crate::types::{
    BigUint, CodeMetadata, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment,
    EgldOrEsdtTokenPaymentRefs, EgldOrMultiEsdtPayment, EsdtTokenIdentifier, EsdtTokenPayment,
    EsdtTokenPaymentRefs, ManagedAddress, ManagedBuffer, ManagedVec, MultiEsdtPayment,
    TxPaymentCompose, heap::H256,
};

use multiversx_sc_codec::TopEncodeMulti;

use super::{
    AnnotatedValue, Code, DeployCall, Egld, EgldPayment, ExplicitGas, FromSource, FunctionCall,
    ManagedArgBuffer, OriginalResultMarker, RHList, RHListAppendNoRet, RHListAppendRet, RHListItem,
    TxCodeSource, TxCodeValue, TxData, TxDataFunctionCall, TxEgldValue, TxEnv,
    TxEnvMockDeployAddress, TxEnvWithTxHash, TxFrom, TxFromSourceValue, TxFromSpecified, TxGas,
    TxGasValue, TxPayment, TxPaymentEgldOnly, TxProxyTrait, TxResultHandler, TxTo, TxToSpecified,
    UpgradeCall,
};

/// Universal representation of a blockchain transaction.
///
/// Uses 7 generic type arguments to encode all aspects of the transaction.
///
/// It is future-like, does nothing by itself, it needs a specialized method call to actually run or send it.
///
/// Rationale: https://twitter.com/andreimmarinica/status/1777157322155966601
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
    Data: TxDataFunctionCall<Env>,
    RH: TxResultHandler<Env>,
{
    /// Converts object to a MultiversX transaction data field string.
    pub fn to_call_data_string(&self) -> ManagedBuffer<Env::Api> {
        self.data.to_call_data_string()
    }
}

pub type TxBaseWithEnv<Env> = Tx<Env, (), (), (), (), (), ()>;

impl<Env> TxBaseWithEnv<Env>
where
    Env: TxEnv,
{
    /// Constructor, needs to take an environment object.
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
    /// Specifies transaction sender.
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

impl<Env, From, To, OldPayment, Gas, Data, RH> Tx<Env, From, To, OldPayment, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    OldPayment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// Adds any payment to a transaction, using the payment compose mechanism.
    ///
    /// # Compose Mechanism
    ///
    /// The `payment` method is generic and flexible, allowing you to add any type of payment to a transaction.
    /// It uses the [`TxPaymentCompose`] trait to determine how to combine the existing payment (if any)
    /// with the new payment being added. This enables a fluent and type-safe way to build up complex payment scenarios.
    ///
    /// - If no payment has been added yet (i.e., the payment type is `()`), the new payment is simply set as the transaction's payment.
    /// - If a payment already exists, the `compose` method is called, which merges the new payment with the existing one according to the rules defined by their types.
    /// - This allows for single or multiple ESDT payments, EGLD payments, or combinations thereof, all handled transparently.
    ///
    /// ## Example
    ///
    /// ```ignore
    /// let tx = Tx::new_with_env(env)
    ///     .to(address)
    ///     .payment(esdt_payment)
    ///     .payment(another_esdt_payment)
    ///     .payment(egld_payment);
    /// ```
    ///
    /// Each call to `.payment(...)` will combine the new payment with the previous one, producing the correct payment type for the transaction.
    ///
    /// This mechanism is extensible and supports custom payment types as long as they implement the required traits.
    pub fn payment<NewPayment>(
        self,
        payment: NewPayment,
    ) -> Tx<Env, From, To, <OldPayment as TxPaymentCompose<Env, NewPayment>>::Output, Gas, Data, RH>
    where
        NewPayment: TxPayment<Env>,
        OldPayment: TxPaymentCompose<Env, NewPayment>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment.compose(payment),
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
    #[deprecated(
        since = "0.65.0",
        note = "Use .payment(...) instead, it should support all the same, plus composition of payments"
    )]
    pub fn esdt<P: Into<EsdtTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, EsdtTokenPayment<Env::Api>, Gas, Data, RH> {
        self.payment(payment.into())
    }

    /// Sets a single token payment, with the token identifier and amount kept as references.
    ///
    /// This is handy when we only want one ESDT transfer and we want to avoid unnecessary object clones.
    pub fn single_esdt<'a>(
        self,
        token_identifier: &'a EsdtTokenIdentifier<Env::Api>,
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
    #[deprecated(
        since = "0.65.0",
        note = "Use .payment(...) instead, it should support all the same, plus composition of payments"
    )]
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

    /// Converts argument to `EgldOrMultiEsdtPayment`, then sets it as payment.
    ///
    /// In most cases, `payment` should be used instead.
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
    #[deprecated(
        since = "0.65.0",
        note = "Use .payment(...) instead, it should support all the same, plus composition of payments"
    )]
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
    #[deprecated(
        since = "0.65.0",
        note = "Use .payment(...) instead, it should support all the same, plus composition of payments"
    )]
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
    #[deprecated(
        since = "0.65.0",
        note = "Use .payment(...) instead, it should support all the same, plus composition of payments"
    )]
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
    /// Sets the data field. Do not use directly.
    #[inline]
    #[doc(hidden)]
    pub fn raw_data<Data>(self, data: Data) -> Tx<Env, From, To, Payment, Gas, Data, RH>
    where
        Data: TxData<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data,
            result_handler: self.result_handler,
        }
    }

    /// Starts a contract call, serialized by hand.
    ///
    /// Whenever possible, should use proxies instead, since manual serialization is not type-safe.
    #[inline]
    pub fn raw_call<N: Into<ManagedBuffer<Env::Api>>>(
        self,
        function_name: N,
    ) -> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH> {
        self.raw_data(FunctionCall::new(function_name))
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
    /// Converts tx to a simple FunctionCall, to be used as argument or data in contracts.
    pub fn into_function_call(self) -> FunctionCall<Env::Api> {
        self.data
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    /// Produces the normalized function call, i.e. with builtin function calls for ESDT transfers.
    ///
    /// The resulting transaction can differ from the input in several ways:
    /// - the recipient is changed (some builtin functions are called with recipient = sender),
    /// - the function call becomes a builtin function call.
    ///
    /// ## Important
    ///
    /// Do not call this before sending transactions! Normalization is don automatically whenever necessary.
    /// Only use when you need the normalized data, e.g. for a multisig.
    ///
    /// ## Warning
    ///
    /// To produce owned values, some clones are performed.
    /// It is not optimized for contracts, but can be used nonetheless.
    #[allow(clippy::type_complexity)]
    pub fn normalize(
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
        let (norm_to, norm_egld, norm_fc) = self.payment.with_normalized(
            &self.env,
            &self.from,
            self.to,
            self.data,
            |norm_to, norm_egld, norm_fc| (norm_to.clone(), norm_egld.clone(), norm_fc),
        );

        Tx {
            env: self.env,
            from: self.from,
            to: norm_to,
            payment: Egld(norm_egld),
            gas: self.gas,
            data: norm_fc,
            result_handler: self.result_handler,
        }
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
    /// Used for function calls originating in legacy proxies.
    ///
    /// Different environment in the argument allowed because of compatibility with old proxies.
    ///
    /// Method still subject to considerable change.
    pub fn legacy_proxy_call<Env2, To, O>(
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
    /// Adds argument to function call.
    ///
    /// Whenever possible, use proxies instead.
    ///
    /// It serializes the value, but does not enforce type safety.
    #[inline]
    pub fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.data = self.data.argument(arg);
        self
    }

    /// Adds serialized argument to function call.
    ///
    /// Whenever possible, use proxies instead.
    ///
    /// Does not serialize, does not enforce type safety.
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
    /// Type marker to set the original contract or VM function return type.
    ///
    /// Only the compile-time type annotation is given.
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
    /// Starts a proxy call, deploy, or upgrade.
    ///
    /// The proxy object will be given, the subsequent call will be from a proxy context, containing all the contract endpoint names.
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
    /// Adds a result handler that doesn't return anything.
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

    /// Adds a result handler that can also return processed data.
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

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, (), RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    /// Starts a contract deploy call, serialized by hand.
    ///
    /// Whenever possible, should use proxies instead, since manual serialization is not type-safe.
    pub fn raw_deploy(self) -> Tx<Env, From, To, Payment, Gas, DeployCall<Env, ()>, RH> {
        self.raw_data(DeployCall::default())
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
    /// Sets upgrade code source as explicit code bytes.
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

    /// Sets upgrade code source as another deployed contract code.
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
    /// Sets deploy code source as explicit code bytes.
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

    /// Sets deploy code source as another deployed contract code.
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
    /// Sets code metadata to deploy.
    pub fn code_metadata(mut self, code_metadata: CodeMetadata) -> Self {
        self.data = self.data.code_metadata(code_metadata);
        self
    }

    /// Adds argument to a contract deploy.
    ///
    /// Whenever possible, use proxies instead.
    ///
    /// It serializes the value, but does not enforce type safety.
    #[inline]
    pub fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.data = self.data.argument(arg);
        self
    }

    /// Adds serialized argument to a contract deploy.
    ///
    /// Whenever possible, use proxies instead.
    ///
    /// Does not serialize, does not enforce type safety.
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

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, (), RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    /// Starts a contract deploy upgrade, serialized by hand.
    ///
    /// Whenever possible, should use proxies instead, since manual serialization is not type-safe.
    pub fn raw_upgrade(self) -> Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, ()>, RH> {
        self.raw_data(UpgradeCall::default())
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

    /// Adds argument to upgrade call.
    ///
    /// Whenever possible, use proxies instead.
    ///
    /// It serializes the value, but does not enforce type safety.
    #[inline]
    pub fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.data = self.data.argument(arg);
        self
    }

    /// Adds serialized argument to an upgrade call.
    ///
    /// Whenever possible, use proxies instead.
    ///
    /// Doesa not serialize, does not enforce type safety.
    #[inline]
    pub fn arguments_raw(mut self, raw: ManagedArgBuffer<Env::Api>) -> Self {
        self.data.arg_buffer = raw;
        self
    }
}

impl<Env, From, To, Payment, Gas, Data, RH> Tx<Env, From, To, Payment, Gas, Data, RH>
where
    Env: TxEnvWithTxHash,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn id(self, _id: &str) -> Self {
        // TODO: implement
        self
    }

    /// Sets the mock transaction hash to be used in a test.
    ///
    /// Only allowed in tests.
    pub fn tx_hash<H>(mut self, tx_hash: H) -> Self
    where
        H256: core::convert::From<H>,
    {
        self.env.set_tx_hash(H256::from(tx_hash));
        self
    }
}
