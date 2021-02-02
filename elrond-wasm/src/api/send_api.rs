use super::BigUintApi;
use crate::types::{Address, TokenIdentifier};

pub const DIRECT_ESDT_DEFAULT_GAS: u64 = 500000;

pub trait SendApi<BigUint>: Sized
where
	BigUint: BigUintApi + 'static,
{
	fn direct_egld(&self, to: &Address, amount: &BigUint, data: &[u8]);

	#[inline]
	fn direct_esdt(&self, to: &Address, token: &[u8], amount: &BigUint, data: &[u8]) {
		self.direct_esdt_explicit_gas(to, token, amount, DIRECT_ESDT_DEFAULT_GAS, data);
	}

	fn direct_esdt_explicit_gas(
		&self,
		to: &Address,
		token: &[u8],
		amount: &BigUint,
		gas: u64,
		data: &[u8],
	);

	fn direct(&self, to: &Address, token: &TokenIdentifier, amount: &BigUint, data: &[u8]) {
		if token.is_egld() {
			self.direct_egld(to, amount, data);
		} else {
			self.direct_esdt(to, token.as_slice(), amount, data);
		}
	}
}
