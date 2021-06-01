#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod bc_function;
use bc_function::{BCFunction, CurveArguments, SupplyType, Token};

mod linear_function;
use linear_function::LinearFunction;

const TOKEN_NUM_DECIMALS: usize = 18;

#[elrond_wasm_derive::contract]
pub trait BondingCurve {
	#[init]
	fn init(&self, issued_token: Token, exchanging_token: Token) {
		self.issued_token().set(&issued_token);
		self.exchanging_token().set(&exchanging_token);
	}

	// endpoint - owner-only

	#[payable("*")]
	#[endpoint(issueToken)]
	fn issue_token(
		&self,
		initial_supply: Self::BigUint,
		#[payment] issue_cost: Self::BigUint,
	) -> SCResult<AsyncCall<Self::SendApi>> {
		only_owner!(self, "only owner may call this function");

		require!(
			self.issued_token().is_empty(),
			"wrapped egld was already issued"
		);

		let caller = self.blockchain().get_caller();

		self.issue_started_event(
			&caller,
			&self.issued_token().get().identifier,
			&initial_supply,
		);

		Ok(ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.issue_fungible(
				issue_cost,
				&self.issued_token().get().name,
				&self.issued_token().get().identifier.into_boxed_bytes(),
				&initial_supply,
				FungibleTokenProperties {
					num_decimals: TOKEN_NUM_DECIMALS,
					can_freeze: false,
					can_wipe: false,
					can_pause: false,
					can_mint: true,
					can_burn: false,
					can_change_owner: true,
					can_upgrade: true,
					can_add_special_roles: false,
				},
			)
			.async_call()
			.with_callback(self.callbacks().issue_callback(&caller)))
	}

	#[callback]
	fn issue_callback(
		&self,
		caller: &Address,
		#[payment] amount: Self::BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) {
		let token = self.issued_token().get().identifier;
		match result {
			AsyncCallResult::Ok(()) => {
				self.issue_success_event(caller, &token, &amount);
				self.supply().set(&amount);
				self.balance().set(&amount);
			},
			AsyncCallResult::Err(message) => {
				self.issue_failure_event(caller, message.err_msg.as_slice());
				if token.is_egld() && amount > 0 {
					self.send().direct_egld(caller, &amount, &[]);
				}
			},
		}
	}

	#[endpoint(mintToken)]
	fn mint_token(&self, amount: Self::BigUint) -> SCResult<AsyncCall<Self::SendApi>> {
		only_owner!(self, "only owner may call this function");

		let token = self.issued_token().get().identifier;
		if self.issued_token().is_empty() {
			return Err(SCError::from(BoxedBytes::from_concat(&[
				token.as_esdt_identifier(),
				b" was not issued yet",
			])));
		}

		require!(
			self.supply_type().get() != SupplyType::Limited
				|| self.max_supply().get() > self.supply().get(),
			"Maximum supply limit reached!"
		);

		require!(
			self.supply_type().get() != SupplyType::Limited
				|| self.max_supply().get() >= self.supply().get() + amount.clone(),
			"Minting will exceed the maximum supply limit!"
		);

		let caller = self.blockchain().get_caller();
		self.mint_started_event(&caller, &amount);

		Ok(ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.mint(&token, &amount)
			.async_call()
			.with_callback(self.callbacks().mint_callback(&caller, &amount)))
	}

	#[callback]
	fn mint_callback(
		&self,
		caller: &Address,
		amount: &Self::BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) {
		match result {
			AsyncCallResult::Ok(()) => {
				self.mint_success_event(caller);
				self.supply().update(|supply| *supply += amount);
				self.balance().update(|balance| *balance += amount);
			},
			AsyncCallResult::Err(message) => {
				self.mint_failure_event(caller, message.err_msg.as_slice());
			},
		}
	}

	// endpoints

	fn check_sell_requirements(
		&self,
		token: &TokenIdentifier,
		amount: &Self::BigUint,
	) -> SCResult<()> {
		let issued_token = self.issued_token().get().identifier;
		if &issued_token != token {
			return Err(SCError::from(BoxedBytes::from_concat(&[
				b"Only ",
				token.as_esdt_identifier(),
				b" tokens accepted",
			])));
		}
		if self.balance().get() < amount.clone() {
			return Err(SCError::from(BoxedBytes::from_concat(&[
				b"Contract does not have enough ",
				token.as_esdt_identifier(),
				b". Please try again once more is minted.",
			])));
		}
		require!(
			amount > &Self::BigUint::zero(),
			"Must pay more than 0 tokens!"
		);
		Ok(())
	}

	fn check_buy_requirements(
		&self,
		token: &TokenIdentifier,
		amount: &Self::BigUint,
	) -> SCResult<()> {
		let exchanging_token = &self.exchanging_token().get().identifier;
		if exchanging_token != token {
			return Err(SCError::from(BoxedBytes::from_concat(&[
				b"Only ",
				exchanging_token.as_esdt_identifier(),
				b" tokens accepted",
			])));
		}
		require!(
			amount > &Self::BigUint::zero(),
			"Must pay more than 0 tokens!"
		);
		Ok(())
	}
	fn get_curve_arguments(self) -> CurveArguments<Self::BigUint> {
		CurveArguments {
			supply_type: self.supply_type().get(),
			max_supply: self.max_supply().get(),
			current_supply: self.supply().get(),
			balance: self.balance().get(),
		}
	}

	fn get_function(&self) -> LinearFunction<Self::BigUint> {
		LinearFunction {
			a: Self::BigUint::from(3u64),
			b: Self::BigUint::from(5u64),
		}
	}

	fn calculate_buy_price(&self, amount: Self::BigUint) -> SCResult<Self::BigUint> {
		self.get_function().buy(amount, self.get_curve_arguments())
	}

	fn calculate_sale_price(&self, amount: Self::BigUint) -> SCResult<Self::BigUint> {
		self.get_function().sell(amount, self.get_curve_arguments())
	}

	#[payable("*")]
	#[endpoint(buyToken)]
	fn buy_token(
		&self,
		#[payment] buy_amount: Self::BigUint,
		#[payment_token] token_identifier: TokenIdentifier,
	) -> SCResult<()> {
		self.check_buy_requirements(&token_identifier, &buy_amount)?;

		let calculated_price = &self.calculate_buy_price(buy_amount.clone()).unwrap();

		self.balance().update(|balance| *balance += buy_amount);

		let caller = self.blockchain().get_caller();

		let _ = self
			.send()
			.direct(&caller, &token_identifier, &calculated_price, b"buying");

		self.buy_token_event(&caller, &calculated_price);

		Ok(())
	}

	#[payable("*")]
	#[endpoint(sellToken)]
	fn sell_token(
		&self,
		#[payment] sell_amount: Self::BigUint,
		#[payment_token] token_identifier: TokenIdentifier,
	) -> SCResult<()> {
		self.check_sell_requirements(&token_identifier, &sell_amount)?;

		let calculated_price = &self.calculate_sale_price(sell_amount).unwrap();

		self.balance()
			.update(|balance| *balance -= calculated_price);

		let caller = self.blockchain().get_caller();

		let _ = self
			.send()
			.direct(&caller, &token_identifier, &calculated_price, b"selling");

		self.sell_token_event(&caller, &calculated_price);
		Ok(())
	}

	// storage

	#[view(supplyType)]
	#[storage_mapper("supply_type")]
	fn supply_type(&self) -> SingleValueMapper<Self::Storage, SupplyType>;

	#[view(maxSupply)]
	#[storage_mapper("max_supply")]
	fn max_supply(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getIssuedToken)]
	#[storage_mapper("issued_token")]
	fn issued_token(&self) -> SingleValueMapper<Self::Storage, Token>;

	#[view(getExchangingToken)]
	#[storage_mapper("exchanging_token")]
	fn exchanging_token(&self) -> SingleValueMapper<Self::Storage, Token>;

	#[view(getMintedSupply)]
	#[storage_mapper("supply")]
	fn supply(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getAvailableSupply)]
	#[storage_mapper("balance")]
	fn balance(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	//events

	#[event("issue-started")]
	fn issue_started_event(
		&self,
		#[indexed] caller: &Address,
		#[indexed] token_identifier: &TokenIdentifier,
		initial_supply: &Self::BigUint,
	);

	#[event("issue-success")]
	fn issue_success_event(
		&self,
		#[indexed] caller: &Address,
		#[indexed] token_identifier: &TokenIdentifier,
		initial_supply: &Self::BigUint,
	);

	#[event("issue-failure")]
	fn issue_failure_event(&self, #[indexed] caller: &Address, message: &[u8]);

	#[event("mint-started")]
	fn mint_started_event(&self, #[indexed] caller: &Address, amount: &Self::BigUint);

	#[event("mint-success")]
	fn mint_success_event(&self, #[indexed] caller: &Address);

	#[event("mint-failure")]
	fn mint_failure_event(&self, #[indexed] caller: &Address, message: &[u8]);

	#[event("buy-token")]
	fn buy_token_event(&self, #[indexed] user: &Address, amount: &Self::BigUint);

	#[event("sell-token")]
	fn sell_token_event(&self, #[indexed] user: &Address, amount: &Self::BigUint);
}
