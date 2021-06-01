elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum SupplyType {
	Limited,
	Limitless,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct CurveArguments<BigUint: BigUintApi> {
	pub supply_type: SupplyType,
	pub max_supply: BigUint,
	pub current_supply: BigUint,
	pub balance: BigUint,
}

impl<BigUint> CurveArguments<BigUint>
where
	BigUint: BigUintApi,
{
	fn first_token_available(&self) -> BigUint {
		self.current_supply.clone() - self.balance.clone()
	}
}

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone)]
pub struct Token {
	pub name: BoxedBytes,
	pub identifier: TokenIdentifier,
}

pub trait BCFunction<BigUint: BigUintApi> {
	fn function(
		&self,
		token_start: BigUint,
		amount: BigUint,
		arguments: &CurveArguments<BigUint>,
	) -> SCResult<BigUint>;

	fn sell(&self, amount: BigUint, arguments: CurveArguments<BigUint>) -> SCResult<BigUint> {
		let token_start = arguments.first_token_available();
		self.function(token_start, amount, &arguments)
	}

	fn buy(&self, amount: BigUint, arguments: CurveArguments<BigUint>) -> SCResult<BigUint> {
		let token_start = arguments.first_token_available() - BigUint::from(1u64) - amount.clone();
		self.function(token_start, amount, &arguments)
	}
}
