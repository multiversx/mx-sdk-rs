use super::{BigUintApi, ErrorApi};
// use crate::err_msg;
use crate::types::{Address, TokenIdentifier};

pub trait SendApi<BigUint>: Sized
where
	BigUint: BigUintApi + 'static,
{
	fn egld(&self, to: &Address, amount: &BigUint, data: &[u8]);
}
