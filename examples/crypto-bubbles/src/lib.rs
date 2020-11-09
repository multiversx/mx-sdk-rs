#![no_std]
#![allow(non_snake_case)]
#![allow(clippy::string_lit_as_bytes)]

imports!();

#[elrond_wasm_derive::contract(CryptoBubblesImpl)]
pub trait CryptoBubbles {
	/// constructor function
	/// is called immediately after the contract is created
	#[init]
	fn init(&self) {

	}

	/// getter function: retrieves balance for an account
	#[view(balanceOf)]
	fn get_player_balance(&self, player: Address) -> BigUint {
		self.storage_get_player_balance(&player)
	}

	/// player adds funds
	#[payable]
	#[endpoint(topUp)]
	fn add_funds(&self, #[payment] payment: BigUint) {
		let caller = self.get_caller();

		let mut balance = self.storage_get_player_balance(&caller);
		balance += &payment;
		self.storage_set_player_balance(&caller, &balance);

		self.top_up_event(&caller, &payment);
	}

	/// player withdraws funds
	#[endpoint]
	fn withdraw(&self, amount: &BigUint) -> SCResult<()> {
		self._transfer_back_to_player_wallet(&self.get_caller(), amount)
	}

	/// server calls withdraw on behalf of the player
	fn _transfer_back_to_player_wallet(&self, player: &Address, amount: &BigUint) -> SCResult<()> {
		let mut balance = self.storage_get_player_balance(player);

		require!(amount <= &balance,
			"amount to withdraw must be less or equal to balance");

		balance -= amount;
		self.storage_set_player_balance(player, &balance);

		self.send_tx(player, &amount, "crypto bubbles");

		self.withdraw_event(player, amount);

		Ok(())
	}

	/// player joins game
	fn _add_player_to_game_state_change(
		&self,
		game_index: &BigUint,
		player: &Address,
		bet: &BigUint,
	) -> SCResult<()> {
		let mut balance = self.storage_get_player_balance(player);
		
		require!(bet <= &balance, "insufficient funds to join game");

		balance -= bet;
		self.storage_set_player_balance(player, &balance);

		self.player_joins_game_event(game_index, player, bet);

		Ok(())
	}

	// player tops up + joins a game
	#[payable]
	#[endpoint(joinGame)]
	fn join_game(&self, game_index: BigUint) -> SCResult<()> {
		let player = self.get_caller();
		let bet = self.get_call_value_big_uint();

		self.add_funds(self.get_call_value_big_uint());
		self._add_player_to_game_state_change(&game_index, &player, &bet)
	}

	// owner transfers prize into winner SC account
	#[endpoint(rewardWinner)]
	fn reward_winner(
		&self,
		game_index: &BigUint,
		winner: &Address,
		prize: &BigUint,
	) -> SCResult<()> {
		let caller = self.get_caller();
		let owner: Address = self.get_owner_address();
		require!(caller == owner, "invalid sender: only contract owner can reward winner");

		let mut balance = self.storage_get_player_balance(winner);
		balance += prize;
		self.storage_set_player_balance(winner, &balance);

		self.reward_winner_event(game_index, &winner, &prize);

		Ok(())
	}

	// owner transfers prize into winner SC account, then transfers funds to player wallet
	#[endpoint(rewardAndSendToWaller)]
	fn reward_and_send_to_wallet(
		&self,
		game_index: &BigUint,
		winner: &Address,
		prize: &BigUint,
	) -> SCResult<()> {
		sc_try!(self.reward_winner(game_index, winner, prize));
		sc_try!(self._transfer_back_to_player_wallet(winner, prize));
		Ok(())
	}

	// Storage

	#[storage_get("playerBalance")]
	fn storage_get_player_balance(&self, player: &Address) -> BigUint;

	#[storage_set("playerBalance")]
	fn storage_set_player_balance(&self, player: &Address, balance: &BigUint);

	// Events

	#[event("0x1000000000000000000000000000000000000000000000000000000000000001")]
	fn top_up_event(&self, player: &Address, amount: &BigUint);

	#[event("0x1000000000000000000000000000000000000000000000000000000000000002")]
	fn withdraw_event(&self, player: &Address, amount: &BigUint);

	#[event("0x1000000000000000000000000000000000000000000000000000000000000003")]
	fn player_joins_game_event(&self, game_index: &BigUint, player: &Address, bet: &BigUint);

	#[event("0x1000000000000000000000000000000000000000000000000000000000000004")]
	fn reward_winner_event(&self, game_index: &BigUint, winner: &Address, prize: &BigUint);
}
