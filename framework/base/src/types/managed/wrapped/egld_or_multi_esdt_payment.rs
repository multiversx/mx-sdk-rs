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
pub enum EgldOrMultiEsdtPayment<'a, M: ManagedTypeApi<'a>> {
    Egld(BigUint<'a, M>),
    MultiEsdt(ManagedVec<'a, M, EsdtTokenPayment<'a, M>>),
}

impl<'a, M> CodecFromSelf for EgldOrMultiEsdtPayment<'a, M> where M: ManagedTypeApi<'a> {}
