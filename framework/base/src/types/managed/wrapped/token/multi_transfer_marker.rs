use crate::{
    api::ManagedTypeApi,
    types::{ManagedRef, PaymentVec},
};

/// A wrapper that forces payments to go via MultiESDTtransfer, even if it wouldn't be necessary, such as:
/// - Just EGLD
/// - Single fungible ESDT transfers,
/// - Single NFT transfers.
///
/// This contrasts with unwrapped PaymentVec, tries to use the simplest possible transfer type.
pub struct MultiTransfer<P>(pub P)
where
    P: MultiTransferMarkerArg;

/// Marks an allowed generic argument for MultiTransfer.
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot be used in `MultiTransfer`",
    label = "unsupported MultiTransfer argument",
    note = "only `PaymentVec` and its references can be used as `MultiTransfer` arguments for now, please do signal the team if any other type is desired"
)]
pub trait MultiTransferMarkerArg {}

impl<M: ManagedTypeApi> MultiTransferMarkerArg for PaymentVec<M> {}
impl<M: ManagedTypeApi> MultiTransferMarkerArg for &PaymentVec<M> {}
impl<M: ManagedTypeApi> MultiTransferMarkerArg for ManagedRef<'_, M, PaymentVec<M>> {}
