use crate::{
    api::ManagedTypeApi,
    types::{ManagedRef, PaymentVec},
};

/// A wrapper that causes payments to be sent in the simplest form, if possible:
/// - Direct EGLD
/// - ESDTTransfer for single fungible tranasfers,
/// - MultiESDTNFTTransfer for everything else.
///
/// This contrasts with unwrapped PaymentVec, which always does a multi-transfer.
pub struct Compact<P>(pub P)
where
    P: CompactPayment;

pub trait CompactPayment {}

impl<M: ManagedTypeApi> CompactPayment for &PaymentVec<M> {}
impl<M: ManagedTypeApi> CompactPayment for ManagedRef<'_, M, PaymentVec<M>> {}
