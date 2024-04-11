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
pub struct EgldOrEsdtTokenPayment<'a, M: ManagedTypeApi<'a>> {
    pub token_identifier: EgldOrEsdtTokenIdentifier<'a, M>,
    pub token_nonce: u64,
    pub amount: BigUint<'a, M>,
}

impl<'a, M: ManagedTypeApi<'a>> EgldOrEsdtTokenPayment<'a, M> {
    pub fn no_payment() -> Self {
        EgldOrEsdtTokenPayment {
            token_identifier: EgldOrEsdtTokenIdentifier::egld(),
            token_nonce: 0,
            amount: BigUint::zero(),
        }
    }

    pub fn new(
        token_identifier: EgldOrEsdtTokenIdentifier<'a, M>,
        token_nonce: u64,
        amount: BigUint<'a, M>,
    ) -> Self {
        EgldOrEsdtTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    /// Will convert to just ESDT or terminate execution if the token is EGLD.
    pub fn unwrap_esdt(self) -> EsdtTokenPayment<'a, M> {
        EsdtTokenPayment::new(
            self.token_identifier.unwrap_esdt(),
            self.token_nonce,
            self.amount,
        )
    }

    pub fn into_tuple(self) -> (EgldOrEsdtTokenIdentifier<'a, M>, u64, BigUint<'a, M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<(EgldOrEsdtTokenIdentifier<'a, M>, u64, BigUint<'a, M>)>
    for EgldOrEsdtTokenPayment<'a, M>
{
    #[inline]
    fn from(value: (EgldOrEsdtTokenIdentifier<'a, M>, u64, BigUint<'a, M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<EsdtTokenPayment<'a, M>> for EgldOrEsdtTokenPayment<'a, M> {
    fn from(esdt_payment: EsdtTokenPayment<'a, M>) -> Self {
        EgldOrEsdtTokenPayment {
            token_identifier: EgldOrEsdtTokenIdentifier::esdt(esdt_payment.token_identifier),
            token_nonce: esdt_payment.token_nonce,
            amount: esdt_payment.amount,
        }
    }
}

impl<'a, M> CodecFromSelf for EgldOrEsdtTokenPayment<'a, M> where M: ManagedTypeApi<'a> {}

impl<'a, M> CodecFrom<&[u8]> for EgldOrEsdtTokenPayment<'a, M> where M: ManagedTypeApi<'a> {}
