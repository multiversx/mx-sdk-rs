use super::{EsdtTokenPayment, ManagedVec};
use crate::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
        CodecFromSelf,
    },
    types::BigUint,
};

use crate as multiversx_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

/// Encodes any type of payment, which either:
/// - EGLD (can be zero in case of no payment whatsoever);
/// - Multi-ESDT (one or more ESDT transfers).
#[derive(
    TopDecode, TopEncode, TypeAbi, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug,
)]
pub enum EgldOrMultiEsdtPayment<M: ManagedTypeApi> {
    Egld(BigUint<M>),
    MultiEsdt(ManagedVec<M, EsdtTokenPayment<M>>),
}

impl<M> CodecFromSelf for EgldOrMultiEsdtPayment<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> EgldOrMultiEsdtPayment<M> {
    pub fn is_empty(&self) -> bool {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_value) => egld_value == &0u32,
            EgldOrMultiEsdtPayment::MultiEsdt(esdt_payments) => esdt_payments.is_empty(),
        }
    }
}

pub enum EgldOrMultiEsdtPaymentRefs<'a, M: ManagedTypeApi> {
    Egld(&'a BigUint<M>),
    MultiEsdt(&'a ManagedVec<M, EsdtTokenPayment<M>>),
}

impl<M: ManagedTypeApi> EgldOrMultiEsdtPayment<M> {
    pub fn as_refs(&self) -> EgldOrMultiEsdtPaymentRefs<'_, M> {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_value) => {
                EgldOrMultiEsdtPaymentRefs::Egld(egld_value)
            },
            EgldOrMultiEsdtPayment::MultiEsdt(esdt_payments) => {
                EgldOrMultiEsdtPaymentRefs::MultiEsdt(esdt_payments)
            },
        }
    }
}
