use crate::{
    api::ManagedTypeApi,
    types::{BigUint, EgldOrEsdtTokenIdentifier},
};

use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    CodecFrom, CodecFromSelf,
};

use crate as multiversx_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

use super::EsdtTokenPayment;

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub struct EgldOrEsdtTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: EgldOrEsdtTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> EgldOrEsdtTokenPayment<M> {
    pub fn no_payment() -> Self {
        EgldOrEsdtTokenPayment {
            token_identifier: EgldOrEsdtTokenIdentifier::egld(),
            token_nonce: 0,
            amount: BigUint::zero(),
        }
    }

    pub fn new(
        token_identifier: EgldOrEsdtTokenIdentifier<M>,
        token_nonce: u64,
        amount: BigUint<M>,
    ) -> Self {
        EgldOrEsdtTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    /// Will convert to just ESDT or terminate execution if the token is EGLD.
    pub fn unwrap_esdt(self) -> EsdtTokenPayment<M> {
        EsdtTokenPayment::new(
            self.token_identifier.unwrap_esdt(),
            self.token_nonce,
            self.amount,
        )
    }

    pub fn into_tuple(self) -> (EgldOrEsdtTokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }
}

impl<M: ManagedTypeApi> From<(EgldOrEsdtTokenIdentifier<M>, u64, BigUint<M>)>
    for EgldOrEsdtTokenPayment<M>
{
    #[inline]
    fn from(value: (EgldOrEsdtTokenIdentifier<M>, u64, BigUint<M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> From<EsdtTokenPayment<M>> for EgldOrEsdtTokenPayment<M> {
    fn from(esdt_payment: EsdtTokenPayment<M>) -> Self {
        EgldOrEsdtTokenPayment {
            token_identifier: EgldOrEsdtTokenIdentifier::esdt(esdt_payment.token_identifier),
            token_nonce: esdt_payment.token_nonce,
            amount: esdt_payment.amount,
        }
    }
}

impl<M> CodecFromSelf for EgldOrEsdtTokenPayment<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<&[u8]> for EgldOrEsdtTokenPayment<M> where M: ManagedTypeApi {}
