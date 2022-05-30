use crate::{
    api::ManagedTypeApi,
    types::{BigUint, EgldOrEsdtTokenIdentifier},
};

use elrond_codec::{
    elrond_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    CodecFrom, CodecFromSelf,
};

use crate as elrond_wasm; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

use super::EsdtTokenPayment;

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Debug)]
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

    pub fn into_tuple(self) -> (EgldOrEsdtTokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
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
