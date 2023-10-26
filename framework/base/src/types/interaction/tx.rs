use core::marker::PhantomData;

use multiversx_sc_codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, EgldPayment, EsdtTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec,
        MultiEsdtPayment,
    },
};

use super::{AsyncCall, ExplicitGas, FunctionCall, TxData, TxFrom, TxGas, TxPayment, TxTo};

pub struct Tx<Api, From, To, Payment, Gas, Data>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    To: TxTo<Api>,
    Payment: TxPayment<Api>,
    Gas: TxGas,
    Data: TxData<Api>,
{
    pub(super) _phantom: PhantomData<Api>,
    pub from: From,
    pub to: To,
    pub payment: Payment,
    pub gas: Gas,
    pub data: Data,
}

impl<Api, From, To, Payment, Gas, Data> Tx<Api, From, To, Payment, Gas, Data>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    To: TxTo<Api>,
    Payment: TxPayment<Api>,
    Gas: TxGas,
    Data: TxData<Api>,
{
    #[inline]
    pub fn nothing(self) -> Tx<Api, From, To, Payment, Gas, Data> {
        Tx {
            _phantom: PhantomData,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
        }
    }
}

pub type TxBase<Api> = Tx<Api, (), (), (), (), ()>;

impl<Api> TxBase<Api>
where
    Api: CallTypeApi + 'static,
{
    #[inline]
    pub fn new() -> Self {
        Tx {
            _phantom: PhantomData,
            from: (),
            to: (),
            payment: (),
            gas: (),
            data: (),
        }
    }
}

impl<Api, To, Payment, Gas, Data> Tx<Api, (), To, Payment, Gas, Data>
where
    Api: CallTypeApi + 'static,
    To: TxTo<Api>,
    Payment: TxPayment<Api>,
    Gas: TxGas,
    Data: TxData<Api>,
{
    #[inline]
    pub fn from<From>(self, from: From) -> Tx<Api, From, To, Payment, Gas, Data>
    where
        From: TxFrom<Api>,
    {
        Tx {
            _phantom: PhantomData,
            from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
        }
    }
}

impl<Api, From, Payment, Gas, Data> Tx<Api, From, (), Payment, Gas, Data>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    Payment: TxPayment<Api>,
    Gas: TxGas,
    Data: TxData<Api>,
{
    #[inline]
    pub fn to<To>(self, to: To) -> Tx<Api, From, To, Payment, Gas, Data>
    where
        To: TxTo<Api>,
    {
        Tx {
            _phantom: PhantomData,
            from: self.from,
            to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
        }
    }
}

impl<Api, From, To, Gas, Data> Tx<Api, From, To, (), Gas, Data>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    To: TxTo<Api>,
    Gas: TxGas,
    Data: TxData<Api>,
{
    #[inline]
    pub fn with_egld_transfer(
        self,
        egld_amount: BigUint<Api>,
    ) -> Tx<Api, From, To, EgldPayment<Api>, Gas, Data> {
        Tx {
            _phantom: PhantomData,
            from: self.from,
            to: self.to,
            payment: EgldPayment { value: egld_amount },
            gas: self.gas,
            data: self.data,
        }
    }
}

impl<Api, From, To, Gas, Data> Tx<Api, From, To, (), Gas, Data>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    To: TxTo<Api>,
    Gas: TxGas,
    Data: TxData<Api>,
{
    /// Adds a single ESDT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    #[inline]
    pub fn with_esdt_transfer<P: Into<EsdtTokenPayment<Api>>>(
        self,
        payment: P,
    ) -> Tx<Api, From, To, MultiEsdtPayment<Api>, Gas, Data> {
        let mut payments = ManagedVec::new();
        payments.push(payment.into());
        Tx {
            _phantom: PhantomData,
            from: self.from,
            to: self.to,
            payment: payments,
            gas: self.gas,
            data: self.data,
        }
    }
}

impl<Api, From, To, Gas, Data> Tx<Api, From, To, MultiEsdtPayment<Api>, Gas, Data>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    To: TxTo<Api>,
    Gas: TxGas,
    Data: TxData<Api>,
{
    /// Adds a single ESDT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    #[inline]
    pub fn with_esdt_transfer<P: Into<EsdtTokenPayment<Api>>>(
        mut self,
        payment: P,
    ) -> Tx<Api, From, To, MultiEsdtPayment<Api>, Gas, Data> {
        self.payment.push(payment.into());
        self
    }
}

impl<Api, From, To, Payment, Data> Tx<Api, From, To, Payment, (), Data>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    To: TxTo<Api>,
    Payment: TxPayment<Api>,
    Data: TxData<Api>,
{
    /// Sets an explicit gas limit to the call.
    #[inline]
    pub fn with_gas_limit(self, gas_limit: u64) -> Tx<Api, From, To, Payment, ExplicitGas, Data> {
        Tx {
            _phantom: PhantomData,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: ExplicitGas(gas_limit),
            data: self.data,
        }
    }
}

impl<Api, From, To, Payment, Gas> Tx<Api, From, To, Payment, Gas, ()>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    To: TxTo<Api>,
    Payment: TxPayment<Api>,
    Gas: TxGas,
{
    #[inline]
    pub fn call<F: Into<FunctionCall<Api>>>(
        self,
        call: F,
    ) -> Tx<Api, From, To, Payment, Gas, FunctionCall<Api>> {
        Tx {
            _phantom: PhantomData,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: call.into(),
        }
    }

    #[inline]
    pub fn function_name<N: Into<ManagedBuffer<Api>>>(
        self,
        function_name: N,
    ) -> Tx<Api, From, To, Payment, Gas, FunctionCall<Api>> {
        Tx {
            _phantom: PhantomData,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: FunctionCall::new(function_name),
        }
    }
}

impl<Api, From, To, Payment, Gas> Tx<Api, From, To, Payment, Gas, FunctionCall<Api>>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    To: TxTo<Api>,
    Payment: TxPayment<Api>,
    Gas: TxGas,
{
    #[inline]
    pub fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.data = self.data.argument(arg);
        self
    }
}

impl<Api, From, Payment, Gas> Tx<Api, From, ManagedAddress<Api>, Payment, Gas, FunctionCall<Api>>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    Payment: TxPayment<Api>,
    Gas: TxGas,
{
    pub fn normalize_tx(
        self,
    ) -> Tx<Api, From, ManagedAddress<Api>, EgldPayment<Api>, Gas, FunctionCall<Api>> {
        let result = self.payment.convert_tx_data(&self.from, self.to, self.data);
        Tx {
            _phantom: PhantomData,
            from: self.from,
            to: result.to,
            payment: result.egld_payment,
            gas: self.gas,
            data: result.fc,
        }
    }
}

impl<Api, Payment> Tx<Api, (), ManagedAddress<Api>, Payment, (), FunctionCall<Api>>
where
    Api: CallTypeApi + 'static,
    Payment: TxPayment<Api>,
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

impl<Api, Payment> Tx<Api, (), ManagedAddress<Api>, Payment, ExplicitGas, FunctionCall<Api>>
where
    Api: CallTypeApi + 'static,
    Payment: TxPayment<Api>,
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
