/// A wrapper that forces payments to go via MultiESDTtransfer, even if it wouldn't be necessary, such as:
/// - Just EGLD
/// - Single fungible ESDT transfers,
/// - Single NFT transfers.
///
/// This contrasts with unwrapped PaymentVec, tries to use the simplest possible transfer type.
pub struct MultiTransfer<P>(pub P);
