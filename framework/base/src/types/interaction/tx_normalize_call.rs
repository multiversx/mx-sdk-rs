use core::marker::PhantomData;

use crate::{
    api::CallTypeApi,
    types::{BigUint, EgldPayment, ManagedAddress},
};

use super::{AsyncCall, FunctionCall, Tx, TxData, TxFrom, TxGas};

/// Encodes
pub type NormalizedTx<Api, From, Gas> =
    Tx<Api, From, ManagedAddress<Api>, EgldPayment<Api>, Gas, FunctionCall<Api>>;

pub trait TxNormalizeCall<Api, From, Gas>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    Gas: TxGas,
{
    fn normalize_call(self) -> NormalizedTx<Api, From, Gas>;
}

impl<Api, From, Gas, Call> TxNormalizeCall<Api, From, Gas>
    for Tx<Api, From, ManagedAddress<Api>, (), Gas, Call>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    Gas: TxGas,
    Call: TxData<Api> + Into<FunctionCall<Api>>,
{
    fn normalize_call(self) -> NormalizedTx<Api, From, Gas> {
        Tx {
            _phantom: PhantomData,
            from: self.from,
            to: self.to,
            payment: EgldPayment {
                value: BigUint::zero(),
            },
            gas: self.gas,
            data: self.data.into(),
        }
    }
}

impl<Api, From, Gas, Call> TxNormalizeCall<Api, From, Gas>
    for Tx<Api, From, ManagedAddress<Api>, EgldPayment<Api>, Gas, Call>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    Gas: TxGas,
    Call: TxData<Api> + Into<FunctionCall<Api>>,
{
    fn normalize_call(self) -> NormalizedTx<Api, From, Gas> {
        Tx {
            _phantom: PhantomData,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data.into(),
        }
    }
}

impl<Api, From, Gas> NormalizedTx<Api, From, Gas>
where
    Api: CallTypeApi + 'static,
    From: TxFrom<Api>,
    Gas: TxGas,
{
    fn async_call(self) -> AsyncCall<Api> {
        AsyncCall {
            to: self.to,
            egld_payment: self.payment.value,
            function_call: self.data,
            callback_call: None,
        }
    }

    // #[cfg(feature = "promises")]
    // fn async_call_promise(self) -> super::AsyncCallPromises<SA> {
    //     super::AsyncCallPromises {
    //         to: self.basic.to,
    //         egld_payment: self.egld_payment,
    //         function_call: self.basic.function_call,
    //         explicit_gas_limit: self.basic.explicit_gas_limit,
    //         extra_gas_for_callback: 0,
    //         callback_call: None,
    //     }
    // }
}
