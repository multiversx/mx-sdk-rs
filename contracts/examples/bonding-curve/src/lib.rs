#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod bc_function;
use bc_function::{CurveArguments, Token};

const EGLD_NUM_DECIMALS: usize = 18;

#[elrond_wasm_derive::contract]
pub trait BondingCurve {
	#[init]
	fn init(
		&self,
		issued_token: Token,
		exchanging_token: Token,
		arguments: CurveArguments<Self::BigUint>,
	) {
		self.arguments().set(&arguments);
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
					num_decimals: EGLD_NUM_DECIMALS,
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
		let identifier = self.issued_token().get().identifier.clone();
		match result {
			AsyncCallResult::Ok(()) => {
				self.issue_success_event(caller, &identifier, &amount);
				self.minted_supply().set(&amount);
				self.available_supply().set(&amount);
			},
			AsyncCallResult::Err(message) => {
				self.issue_failure_event(caller, message.err_msg.as_slice());

				// return issue cost to the owner
				// TODO: test that it works
				if identifier.is_egld() && amount > 0 {
					self.send().direct_egld(caller, &amount, &[]);
				}
			},
		}
	}

	#[endpoint(mintToken)]
	fn mint_token(&self, amount: Self::BigUint) -> SCResult<AsyncCall<Self::SendApi>> {
		only_owner!(self, "only owner may call this function");

		let identifier = self.issued_token().get().identifier.clone();
		if self.issued_token().is_empty() {
			return Err(SCError::from(BoxedBytes::from_concat(&[
				identifier.as_esdt_identifier(),
				b" was not issued yet",
			])));
		}

		let esdt_token_id = identifier.as_esdt_identifier();
		let caller = self.blockchain().get_caller();
		self.mint_started_event(&caller, &amount);

		Ok(ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.mint(esdt_token_id, &amount)
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
				self.minted_supply()
					.update(|minted_supply| *minted_supply += amount);
				self.available_supply()
					.update(|available_supply| *available_supply += amount);
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
		if self.available_supply().get() == Self::BigUint::zero() {
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

	fn calculate_buy_price(&self, amount: Self::BigUint) -> SCResult<Self::BigUint> {
		Ok(Self::BigUint::zero())
	}

	fn calculate_sale_price(&self, amount: Self::BigUint) -> SCResult<Self::BigUint> {
		Ok(Self::BigUint::zero())
	}

	#[payable("*")]
	#[endpoint(buyToken)]
	fn buy_token(
		&self,
		#[payment] amount: Self::BigUint,
		#[payment_token] token_identifier: TokenIdentifier,
	) -> SCResult<()> {
		self.check_buy_requirements(&token_identifier, &amount)?;

		let calculated_price = &self.calculate_buy_price(amount).unwrap();

		self.available_supply()
			.update(|available_supply| *available_supply += calculated_price);

		let caller = self.blockchain().get_caller();

		let _ = self.send().direct_esdt_via_transf_exec(
			&caller,
			token_identifier.as_esdt_identifier(),
			&calculated_price,
			b"buying",
		);

		self.buy_token_event(&caller, &calculated_price);

		Ok(())
	}

	#[payable("*")]
	#[endpoint(sellToken)]
	fn sell_token(
		&self,
		#[payment] amount: Self::BigUint,
		#[payment_token] token_identifier: TokenIdentifier,
	) -> SCResult<()> {
		self.check_sell_requirements(&token_identifier, &amount)?;

		let calculated_price = &self.calculate_sale_price(amount).unwrap();

		self.available_supply()
			.update(|available_supply| *available_supply -= calculated_price);

		let caller = self.blockchain().get_caller();

		let _ = self.send().direct_esdt_via_transf_exec(
			&caller,
			token_identifier.as_esdt_identifier(),
			&calculated_price,
			b"selling",
		);

		self.sell_token_event(&caller, &calculated_price);
		Ok(())
	}

	// storage

	#[view(getArguments)]
	#[storage_mapper("arguments")]
	fn arguments(&self) -> SingleValueMapper<Self::Storage, CurveArguments<Self::BigUint>>;

	#[view(getIssuedToken)]
	#[storage_mapper("issued_token")]
	fn issued_token(&self) -> SingleValueMapper<Self::Storage, Token>;

	#[view(getExchangingToken)]
	#[storage_mapper("exchanging_token")]
	fn exchanging_token(&self) -> SingleValueMapper<Self::Storage, Token>;

	#[view(getMintedSupply)]
	#[storage_mapper("minted_supply")]
	fn minted_supply(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getAvailableSupply)]
	#[storage_mapper("available_supply")]
	fn available_supply(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

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
