use crate::api::{BigIntApi, BigUintApi, SendApi};
use crate::types::{Address, TokenIdentifier};

pub trait ContractProxy<SA, BigInt, BigUint>
where
	BigUint: BigUintApi + 'static,
	BigInt: BigIntApi<BigUint> + 'static,
	SA: SendApi<BigUint> + Clone + 'static,
{
	fn new(send_api: SA, address: Address) -> Self;

	fn with_token_transfer(self, token: TokenIdentifier, amount: BigUint) -> Self;
}
