use crate::{
    api::ManagedTypeApi,
    contract_base::SendRawWrapper,
    formatter::FormatBuffer,
    imports::{BigUint, ManagedBuffer, ManagedBufferCachedBuilder, ManagedVec},
    types::{
        EgldOrEsdtTokenPayment, EgldOrMultiEsdtPayment, EgldPayment, EsdtTokenPayment,
        ManagedAddress, MultiEsdtPayment,
    },
};

use super::{FunctionCall, TxEnv, TxFrom, TxToSpecified};

#[derive(Clone)]
pub struct AnnotatedEgldPayment<Api>
where
    Api: ManagedTypeApi,
{
    pub value: BigUint<Api>,
    pub annotation: ManagedBuffer<Api>,
}

impl<Api> AnnotatedEgldPayment<Api>
where
    Api: ManagedTypeApi,
{
    pub fn new_egld(value: BigUint<Api>) -> Self {
        let mut annotation = ManagedBufferCachedBuilder::default();
        annotation.append_display(&value);
        AnnotatedEgldPayment {
            value,
            annotation: annotation.into_managed_buffer(),
        }
    }
}

#[derive(Clone)]
pub struct FullPaymentData<Api>
where
    Api: ManagedTypeApi,
{
    pub egld: Option<AnnotatedEgldPayment<Api>>,
    pub multi_esdt: MultiEsdtPayment<Api>,
}

impl<Api> Default for FullPaymentData<Api>
where
    Api: ManagedTypeApi,
{
    fn default() -> Self {
        Self {
            egld: None,
            multi_esdt: Default::default(),
        }
    }
}

/// Describes a payment that is part of a transaction.
pub trait TxPayment<Env>
where
    Env: TxEnv,
    Self: Clone,
{
    fn is_no_payment(&self) -> bool;

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    );

    fn into_full_payment_data(self) -> FullPaymentData<Env::Api>;
}

/// Marks a payment object that only contains EGLD or nothing at all.
pub trait TxPaymentEgldOnly<Env>: TxPayment<Env>
where
    Env: TxEnv,
{
    fn to_egld_payment(self) -> EgldPayment<Env::Api>;
}

impl<Env> TxPayment<Env> for ()
where
    Env: TxEnv,
{
    fn is_no_payment(&self) -> bool {
        true
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        EgldPayment::no_payment().perform_transfer_execute(env, to, gas_limit, fc);
    }

    fn into_full_payment_data(self) -> FullPaymentData<Env::Api> {
        FullPaymentData::default()
    }
}

impl<Env> TxPaymentEgldOnly<Env> for ()
where
    Env: TxEnv,
{
    fn to_egld_payment(self) -> EgldPayment<Env::Api> {
        EgldPayment::no_payment()
    }
}

impl<Env> TxPayment<Env> for EgldPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self) -> bool {
        self.value == 0u32
    }

    fn perform_transfer_execute(
        self,
        _env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        let _ = SendRawWrapper::<Env::Api>::new().direct_egld_execute(
            to,
            &self.value,
            gas_limit,
            &fc.function_name,
            &fc.arg_buffer,
        );
    }

    fn into_full_payment_data(self) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: Some(AnnotatedEgldPayment::new_egld(self.value)),
            multi_esdt: ManagedVec::new(),
        }
    }
}

impl<Env> TxPaymentEgldOnly<Env> for EgldPayment<Env::Api>
where
    Env: TxEnv,
{
    fn to_egld_payment(self) -> EgldPayment<Env::Api> {
        self
    }
}

impl<Env> TxPayment<Env> for EsdtTokenPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self) -> bool {
        self.amount == 0u32
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        MultiEsdtPayment::from_single_item(self).perform_transfer_execute(env, to, gas_limit, fc);
    }

    fn into_full_payment_data(self) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: None,
            multi_esdt: MultiEsdtPayment::from_single_item(self),
        }
    }
}

impl<Env> TxPayment<Env> for MultiEsdtPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self) -> bool {
        self.is_empty()
    }

    fn perform_transfer_execute(
        self,
        _env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        let _ = SendRawWrapper::<Env::Api>::new().multi_esdt_transfer_execute(
            to,
            &self,
            gas_limit,
            &fc.function_name,
            &fc.arg_buffer,
        );
    }

    fn into_full_payment_data(self) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: None,
            multi_esdt: self,
        }
    }
}

impl<Env> TxPayment<Env> for EgldOrEsdtTokenPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self) -> bool {
        self.amount == 0u32
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.map_egld_or_esdt(
            (to, fc),
            |(to, fc), amount| {
                EgldPayment::from(amount).perform_transfer_execute(env, to, gas_limit, fc)
            },
            |(to, fc), esdt_payment| esdt_payment.perform_transfer_execute(env, to, gas_limit, fc),
        )
    }

    fn into_full_payment_data(self) -> FullPaymentData<Env::Api> {
        self.map_egld_or_esdt(
            (),
            |(), amount| TxPayment::<Env>::into_full_payment_data(EgldPayment::from(amount)),
            |(), esdt_payment| TxPayment::<Env>::into_full_payment_data(esdt_payment),
        )
    }
}

impl<Env> TxPayment<Env> for EgldOrMultiEsdtPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self) -> bool {
        self.is_empty()
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => {
                EgldPayment::from(egld_amount).perform_transfer_execute(env, to, gas_limit, fc)
            },
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => {
                multi_esdt_payment.perform_transfer_execute(env, to, gas_limit, fc)
            },
        }
    }

    fn into_full_payment_data(self) -> FullPaymentData<Env::Api> {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => {
                TxPayment::<Env>::into_full_payment_data(EgldPayment::from(egld_amount))
            },
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => {
                TxPayment::<Env>::into_full_payment_data(multi_esdt_payment)
            },
        }
    }
}
