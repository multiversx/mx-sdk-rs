use multiversx_sc::api::ManagedTypeApi;
use multiversx_sc::types::EsdtTokenPayment;
use crate::data::types::native::NativeConvertible;

#[derive(Clone, PartialEq, Debug)]
pub struct Payment {
    pub token_identifier: String,
    pub token_nonce: u64,
    pub amount: num_bigint::BigUint
}

impl<M: ManagedTypeApi> NativeConvertible for EsdtTokenPayment<M> {
    type Native = Payment;

    fn to_native(&self) -> Self::Native {
        Payment {
            token_identifier: self.token_identifier.to_native(),
            token_nonce: self.token_nonce.to_native(),
            amount: self.amount.to_native()
        }
    }
}