use super::{EsdtTokenPayment, ManagedVec};
use crate::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    },
    types::BigUint,
};

use crate as multiversx_sc; // needed by the TypeAbi generated code
use crate::derive::type_abi;

/// Encodes any type of payment, which either:
/// - EGLD (can be zero in case of no payment whatsoever);
/// - Multi-ESDT (one or more ESDT transfers).
#[type_abi]
#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug)]
pub enum EgldOrMultiEsdtPayment<M: ManagedTypeApi> {
    Egld(BigUint<M>),
    MultiEsdt(ManagedVec<M, EsdtTokenPayment<M>>),
}

impl<M: ManagedTypeApi> EgldOrMultiEsdtPayment<M> {
    pub fn is_empty(&self) -> bool {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_value) => egld_value == &0u32,
            EgldOrMultiEsdtPayment::MultiEsdt(esdt_payments) => esdt_payments.is_empty(),
        }
    }
}

/// The version of `EgldOrMultiEsdtPayment` that contains referrences instead of owned fields.
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

impl<'a, M: ManagedTypeApi> EgldOrMultiEsdtPaymentRefs<'a, M> {
    pub fn to_owned_payment(&self) -> EgldOrMultiEsdtPayment<M> {
        match self {
            EgldOrMultiEsdtPaymentRefs::Egld(egld_value) => {
                EgldOrMultiEsdtPayment::Egld((*egld_value).clone())
            },
            EgldOrMultiEsdtPaymentRefs::MultiEsdt(esdt_payments) => {
                EgldOrMultiEsdtPayment::MultiEsdt((*esdt_payments).clone())
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            EgldOrMultiEsdtPaymentRefs::Egld(egld_value) => *egld_value == &0u32,
            EgldOrMultiEsdtPaymentRefs::MultiEsdt(esdt_payments) => esdt_payments.is_empty(),
        }
    }
}
