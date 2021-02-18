use crate::types::Address;
use crate::{
	api::{BigIntApi, BigUintApi, SendApi},
	TokenIdentifier,
};

pub trait ContractProxy<SA, BigInt, BigUint>
where
	BigUint: BigUintApi + 'static,
	BigInt: BigIntApi<BigUint> + 'static,
	SA: SendApi<BigUint> + Clone + 'static,
{
	fn new(send_api: SA, address: Address) -> Self;

	fn token_transfer(self, token: TokenIdentifier, amount: BigUint) -> Self;

	fn egld_transfer(self, amount: BigUint) -> Self;
}
