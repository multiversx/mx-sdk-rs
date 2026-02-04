mod egld_or_esdt_token_identifier;
mod egld_or_esdt_token_payment;
mod egld_or_esdt_token_payment_refs;
mod egld_or_multi_esdt_payment;
mod esdt_token_data;
mod esdt_token_identifier;
mod esdt_token_payment;
mod fungible_payment;
mod multi_egld_or_esdt_token_payment;
mod multi_transfer_marker;
mod payment;
mod payment_refs;
mod payment_vec;
mod token_id;

pub use egld_or_esdt_token_identifier::EgldOrEsdtTokenIdentifier;
pub use egld_or_esdt_token_payment::EgldOrEsdtTokenPayment;
pub use egld_or_esdt_token_payment_refs::EgldOrEsdtTokenPaymentRefs;
pub use egld_or_multi_esdt_payment::{EgldOrMultiEsdtPayment, EgldOrMultiEsdtPaymentRefs};
pub use esdt_token_data::EsdtTokenData;
pub use esdt_token_identifier::{EsdtTokenIdentifier, TokenIdentifier};
pub use esdt_token_payment::{EsdtTokenPayment, EsdtTokenPaymentRefs, MultiEsdtPayment};
pub use fungible_payment::FungiblePayment;
pub use multi_egld_or_esdt_token_payment::MultiEgldOrEsdtPayment;
pub use multi_transfer_marker::{MultiTransfer, MultiTransferMarkerArg};
pub use payment::Payment;
pub use payment_refs::PaymentRefs;
pub use payment_vec::PaymentVec;
pub use token_id::TokenId;

/// The old representation of the EGLD token, before EGLD-000000.
pub(crate) const LEGACY_EGLD_REPRESENTATION: &[u8; 4] = b"EGLD";
