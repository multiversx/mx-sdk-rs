#![no_std]
#![allow(non_snake_case)]
#![allow(clippy::string_lit_as_bytes)]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract(CryptoBubblesImpl)]
pub trait CryptoBubbles {
	/// constructor function
	/// is called immediately after the contract is created
	#[init]
	fn init(&self) {
		let caller = self.blockchain().get_caller();
		self.set_owner(&caller);
	}

	/// player adds funds
	#[payable("EGLD")]
	#[endpoint(topUp)]
	fn top_up(&self, #[payment] payment: BigUint) {
		let caller = self.blockchain().get_caller();

		let mut balance = self.get_player_balance(&caller);
		balance += &payment;
		self.set_player_balance(&caller, &balance);

		self.top_up_event(&caller, &payment);
	}

	/// player withdraws funds
	#[endpoint]
	fn withdraw(&self, amount: &BigUint) -> SCResult<()> {
		self._transfer_back_to_player_wallet(&self.blockchain().get_caller(), amount)
	}

	/// server calls withdraw on behalf of the player
	fn _transfer_back_to_player_wallet(&self, player: &Address, amount: &BigUint) -> SCResult<()> {
		let mut balance = self.get_player_balance(player);

		require!(
			amount <= &balance,
			"amount to withdraw must be less or equal to balance"
		);

		balance -= amount;
		self.set_player_balance(player, &balance);

		self.send().direct_egld(player, &amount, b"crypto bubbles");

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
		let mut balance = self.get_player_balance(player);

		require!(bet <= &balance, "insufficient funds to join game");

		balance -= bet;
		self.set_player_balance(player, &balance);

		self.player_joins_game_event(game_index, player, bet);

		Ok(())
	}

	// player tops up + joins a game
	#[payable("EGLD")]
	#[endpoint(joinGame)]
	fn join_game(&self, game_index: BigUint, #[payment] bet: BigUint) -> SCResult<()> {
		let player = self.blockchain().get_caller();
		self.top_up(self.call_value().egld_value());
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
		let caller = self.blockchain().get_caller();
		let owner: Address = self.get_owner();
		require!(
			caller == owner,
			"invalid sender: only contract owner can reward winner"
		);

		let mut balance = self.get_player_balance(winner);
		balance += prize;
		self.set_player_balance(winner, &balance);

		self.reward_winner_event(game_index, &winner, &prize);

		Ok(())
	}

	// owner transfers prize into winner SC account, then transfers funds to player wallet
	#[endpoint(rewardAndSendToWallet)]
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

	#[view(balanceOf)]
	#[storage_get("playerBalance")]
	fn get_player_balance(&self, player: &Address) -> BigUint;

	#[storage_set("playerBalance")]
	fn set_player_balance(&self, player: &Address, balance: &BigUint);

	#[storage_get("owner")]
	fn get_owner(&self) -> Address;

	#[storage_set("owner")]
	fn set_owner(&self, owner: &Address);

	// Events

	#[legacy_event("0x1000000000000000000000000000000000000000000000000000000000000001")]
	fn top_up_event(&self, player: &Address, amount: &BigUint);

	#[legacy_event("0x1000000000000000000000000000000000000000000000000000000000000002")]
	fn withdraw_event(&self, player: &Address, amount: &BigUint);

	#[legacy_event("0x1000000000000000000000000000000000000000000000000000000000000003")]
	fn player_joins_game_event(&self, game_index: &BigUint, player: &Address, bet: &BigUint);

	#[legacy_event("0x1000000000000000000000000000000000000000000000000000000000000004")]
	fn reward_winner_event(&self, game_index: &BigUint, winner: &Address, prize: &BigUint);
}
