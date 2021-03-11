use super::{BigUintApi, ErrorApi};
use crate::err_msg;
use crate::types::{EsdtTokenType, TokenIdentifier};

pub trait CallValueApi<BigUint>: ErrorApi + Sized
where
	BigUint: BigUintApi + 'static,
{
	fn check_not_payable(&self);

	/// Retrieves the EGLD call value from the VM.
	/// Will return 0 in case of an ESDT transfer (cannot have both EGLD and ESDT transfer simultaneously).
	fn egld_value(&self) -> BigUint;

	/// Retrieves the ESDT call value from the VM.
	/// Will return 0 in case of an EGLD transfer (cannot have both EGLD and ESDT transfer simultaneously).
	fn esdt_value(&self) -> BigUint;

	/// Returns the call value token identifier of the current call.
	/// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
	///
	/// A note on implementation: even though the underlying api returns an empty name for EGLD,
	/// but the EGLD TokenIdentifier is serialized as `EGLD`.
	fn token(&self) -> TokenIdentifier;

	/// Returns the nonce of the received ESDT token.
	/// Will return 0 in case of EGLD or fungible ESDT transfer.
	fn esdt_token_nonce(&self) -> u64;

	/// Returns the ESDT token type.
	/// Will return "Fungible" for EGLD.
	fn esdt_token_type(&self) -> EsdtTokenType;

	/// Will return the EGLD call value,
	/// but also fail with an error if ESDT is sent.
	/// Especially used in the auto-generated call value processing.
	fn require_egld(&self) -> BigUint {
		if !self.token().is_egld() {
			self.signal_error(err_msg::NON_PAYABLE_FUNC_ESDT);
		}
		self.egld_value()
	}

	/// Will return the ESDT call value,
	/// but also fail with an error if EGLD or the wrong ESDT token is sent.
	/// Especially used in the auto-generated call value processing.
	fn require_esdt(&self, token: &[u8]) -> BigUint {
		if self.token() != token {
			self.signal_error(err_msg::BAD_TOKEN_PROVIDED);
		}
		self.esdt_value()
	}

	/// Returns both the call value (either EGLD or ESDT) and the token identifier.
	/// Especially used in the `#[payable("*")] auto-generated snippets.
	/// The method might seem redundant, but there is such a hook in Arwen
	/// that might be used in this scenario in the future.
	fn payment_token_pair(&self) -> (BigUint, TokenIdentifier) {
		let token = self.token();
		if token.is_egld() {
			(self.egld_value(), token)
		} else {
			(self.esdt_value(), token)
		}
	}
}
