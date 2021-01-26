use super::{BigUintApi, ErrorApi};
use crate::err_msg;
use crate::types::TokenIdentifier;

pub const EGLD_TOKEN_NAME: &[u8] = b"eGLD";

pub trait CallValueApi<BigUint>: ErrorApi + Sized
where
	BigUint: BigUintApi + 'static,
{
	fn check_not_payable(&self);

	fn get_call_value_big_uint(&self) -> BigUint;

	fn get_esdt_value_big_uint(&self) -> BigUint;

	fn get_esdt_token_name(&self) -> TokenIdentifier;

	fn require_egld(&self) -> BigUint {
		if !self.get_esdt_token_name().is_egld() {
			self.signal_error(err_msg::NON_PAYABLE_FUNC_ESDT);
		}
		self.get_call_value_big_uint()
	}

	fn require_esdt(&self, token_identifier: &[u8]) -> BigUint {
		if self.get_esdt_token_name() != token_identifier {
			self.signal_error(err_msg::BAD_ESDT_TOKEN_PROVIDED);
		}
		self.get_esdt_value_big_uint()
	}

	fn require_token(&self, token_identifier: &[u8]) -> BigUint {
		if token_identifier == EGLD_TOKEN_NAME {
			self.require_egld()
		} else {
			self.require_esdt(token_identifier)
		}
	}

	fn get_call_value_token_name(&self) -> (BigUint, TokenIdentifier) {
		let token_identifier = self.get_esdt_token_name();
		if token_identifier.is_egld() {
			(self.get_call_value_big_uint(), token_identifier)
		} else {
			(self.get_esdt_value_big_uint(), token_identifier)
		}
	}
}
