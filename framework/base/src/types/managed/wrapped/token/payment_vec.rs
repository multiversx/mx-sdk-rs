use crate::{
    api::ManagedTypeApi,
    types::{ManagedType, ManagedVec, MultiEgldOrEsdtPayment, Payment},
};

/// Alias for a list of payments.
pub type PaymentVec<Api> = ManagedVec<Api, Payment<Api>>;

impl<M: ManagedTypeApi> PaymentVec<M> {
    /// Converts to the legacy `MultiEgldOrEsdtPayment`.
    ///
    /// It is always safe to do, since the 2 types are guaranteed to have the same layout.
    pub fn as_multi_egld_or_esdt_payment(&self) -> &MultiEgldOrEsdtPayment<M> {
        unsafe { core::mem::transmute(self) }
    }

    /// Converts to the legacy `MultiEgldOrEsdtPayment`.
    ///
    /// It is always safe to do, since the 2 types are guaranteed to have the same layout.
    pub fn into_multi_egld_or_esdt_payment(self) -> MultiEgldOrEsdtPayment<M> {
        unsafe { MultiEgldOrEsdtPayment::from_handle(self.forget_into_handle()) }
    }
}

impl<M: ManagedTypeApi> From<()> for PaymentVec<M> {
    #[inline]
    fn from(_value: ()) -> Self {
        PaymentVec::new()
    }
}

impl<M: ManagedTypeApi> From<Payment<M>> for PaymentVec<M> {
    #[inline]
    fn from(value: Payment<M>) -> Self {
        PaymentVec::from_single_item(value)
    }
}
