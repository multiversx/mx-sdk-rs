use crate::{
    api::{quick_signal_error, ManagedTypeApi},
    err_msg,
    types::{
        BigUint, EgldOrEsdtTokenPayment, EgldOrEsdtTokenPaymentMultiValue, EsdtTokenPayment,
        ManagedVec, MultiValueEncoded,
    },
};

/// Alias for a list of payments of EGLD or ESDT tokens.
pub type MultiEgldOrEsdtPayment<Api> = ManagedVec<Api, EgldOrEsdtTokenPayment<Api>>;

impl<M> MultiEgldOrEsdtPayment<M>
where
    M: ManagedTypeApi,
{
    /// The sum of all EGLD-000000 transfers.
    pub fn egld_sum(&self) -> BigUint<M> {
        let mut sum = BigUint::zero();
        for payment in self {
            if payment.token_identifier.is_egld() {
                sum += &payment.amount;
            }
        }
        sum
    }

    /// Requires that this is a single ESDT payment, and returns it, crashes otherwise.
    pub fn to_single_esdt(self) -> EsdtTokenPayment<M> {
        if self.len() != 1 {
            quick_signal_error::<M>(err_msg::SINGLE_ESDT_EXPECTED)
        }

        let payment = self.get(0).clone();
        payment.unwrap_esdt()
    }

    /// Converts to a multi-value object, in this case a multi-value list of triples:
    /// `[(token identifier, payment, nonce)]`
    pub fn into_multi_value(self) -> MultiValueEncoded<M, EgldOrEsdtTokenPaymentMultiValue<M>> {
        let mut encoded = MultiValueEncoded::new();

        for payment in self {
            encoded.push(EgldOrEsdtTokenPaymentMultiValue::from(payment));
        }

        encoded
    }
}
