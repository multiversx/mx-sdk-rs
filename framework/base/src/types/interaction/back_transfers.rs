use crate::{
    api::ManagedTypeApi,
    types::{
        BigUint, EgldOrEsdtTokenPaymentMultiValue, EsdtTokenPayment, EsdtTokenPaymentVec,
        MultiEgldOrEsdtPayment, MultiValueEncoded, PaymentVec,
    },
};

/// Holding back-transfer data, as retrieved from the VM.
#[deprecated(
    since = "0.59.0",
    note = "BackTransfers is now used instead, the legacy mechanism doesn't handle multi-transfers well"
)]
#[derive(Clone)]
pub struct BackTransfersLegacy<A>
where
    A: ManagedTypeApi,
{
    pub total_egld_amount: BigUint<A>,
    pub esdt_payments: EsdtTokenPaymentVec<A>,
}

/// Holding back-transfer data, as retrieved from the VM.
///
/// It supports all transfer scenarios (EGLD, ESDT, mixed).
#[derive(Clone)]
pub struct BackTransfers<A>
where
    A: ManagedTypeApi,
{
    pub payments: MultiEgldOrEsdtPayment<A>,
}

impl<A> From<MultiEgldOrEsdtPayment<A>> for BackTransfers<A>
where
    A: ManagedTypeApi,
{
    fn from(value: MultiEgldOrEsdtPayment<A>) -> Self {
        BackTransfers::new(value)
    }
}

impl<A> BackTransfers<A>
where
    A: ManagedTypeApi,
{
    pub fn new(payments: MultiEgldOrEsdtPayment<A>) -> Self {
        BackTransfers { payments }
    }

    /// The sum of all EGLD-000000 back-transfers.
    pub fn egld_sum(&self) -> BigUint<A> {
        self.payments.egld_sum()
    }

    /// Requires that back-transfer is a single ESDT payment, and returns it, crashes otherwise.
    pub fn to_single_esdt(self) -> EsdtTokenPayment<A> {
        self.payments.to_single_esdt()
    }

    /// Converts back-transfer to a multi-value object, in this case a multi-value list of triples:
    /// `[(token identifier, payment, nonce)]`
    pub fn into_multi_value(self) -> MultiValueEncoded<A, EgldOrEsdtTokenPaymentMultiValue<A>> {
        self.payments.into_multi_value()
    }

    /// Converts data to the newer PaymentVec (ManagedVec<Payment>).
    pub fn into_payment_vec(self) -> PaymentVec<A> {
        self.payments.into_payment_vec()
    }
}
