use super::BigUintApi;
use crate::types::BoxedBytes;

pub trait CallValueApi<BigUint>: Sized
where
	BigUint: BigUintApi + 'static,
{
	fn check_not_payable(&self);

	fn get_call_value_big_uint(&self) -> BigUint;

	fn get_esdt_value_big_uint(&self) -> BigUint;

	fn get_esdt_token_name(&self) -> BoxedBytes;
}
