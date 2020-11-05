#![no_std]
#![allow(non_snake_case)]
#![allow(clippy::string_lit_as_bytes)]

// TODO: modernize this contract and improve tests

imports!();

static OWNER_KEY: &[u8] = &[0u8; 32];

#[elrond_wasm_derive::contract(CryptoBubblesImpl)]
pub trait CryptoBubbles {
	/// constructor function
	/// is called immediately after the contract is created
	/// will set the fixed global token supply and give all the supply to the creator
	#[init]
	fn init(&self) {
		let sender = self.get_caller();
		self.storage_store_bytes32(OWNER_KEY, &sender.into());
	}

	/// generates the balance key that maps balances with their owners
	fn _player_balance_key(&self, address: &Address) -> H256 {
		let mut raw_key: Vec<u8> = Vec::with_capacity(33);
		raw_key.push(1u8); // "1" is for balance keys
		raw_key.extend_from_slice(address.as_bytes()); // append the entire address
		let key = self.keccak256(&raw_key); // this compresses the key down to 32 bytes
		key.into()
	}

	/// getter function: retrieves balance for an account
	#[view]
	fn balanceOf(&self, subject: Address) -> BigUint {
		let balance_key = self._player_balance_key(&subject);
		let balance = self.storage_load_big_uint(balance_key.as_bytes());
		balance
	}

	/// player adds funds
	#[payable]
	#[endpoint]
	fn topUp(&self, #[payment] payment: BigUint) {
		let caller = self.get_caller();

		let balance_key = self._player_balance_key(&caller);
		let mut balance = self.storage_load_big_uint(balance_key.as_bytes());
		balance += &payment;
		self.storage_store_big_uint(balance_key.as_bytes(), &balance);

		self.top_up_event(&caller, &payment);
	}

	/// player withdraws funds
	#[endpoint]
	fn withdraw(&self, amount: &BigUint) -> SCResult<()> {
		self._transferBackToPlayerWallet(&self.get_caller(), amount)
	}

	/// server calls withdraw on behalf of the player
	fn _transferBackToPlayerWallet(&self, player: &Address, amount: &BigUint) -> SCResult<()> {
		let balance_key = self._player_balance_key(&player);
		let mut balance = self.storage_load_big_uint(balance_key.as_bytes());
		if amount > &balance {
			return sc_error!("amount to withdraw must be less or equal to balance");
		}
		balance -= amount;
		self.storage_store_big_uint(balance_key.as_bytes(), &balance);

		self.send_tx(player, &amount, "crypto bubbles");

		self.withdraw_event(player, amount);

		Ok(())
	}

	/// player joins game
	fn _addPlayerToGameStateChange(
		&self,
		game_index: &BigUint,
		player: &Address,
		bet: &BigUint,
	) -> SCResult<()> {
		let balance_key = self._player_balance_key(&player);
		let mut balance = self.storage_load_big_uint(balance_key.as_bytes());
		if bet > &balance {
			return sc_error!("insufficient funds to join game");
		}
		balance -= bet;
		self.storage_store_big_uint(balance_key.as_bytes(), &balance);

		self.player_joins_game_event(game_index, player, bet);

		Ok(())
	}

	// player tops up + joins a game
	#[payable]
	#[endpoint]
	fn joinGame(&self, game_index: BigUint) -> SCResult<()> {
		let player = self.get_caller();
		let bet = self.get_call_value_big_uint();

		self.topUp(self.get_call_value_big_uint());
		self._addPlayerToGameStateChange(&game_index, &player, &bet)
	}

	// owner transfers prize into winner SC account
	#[endpoint]
	fn rewardWinner(
		&self,
		game_index: &BigUint,
		winner: &Address,
		prize: &BigUint,
	) -> SCResult<()> {
		let caller = self.get_caller();
		let owner: Address = self.storage_load_bytes32(OWNER_KEY).into();
		if caller != owner {
			return sc_error!("invalid sender: only contract owner can reward winner");
		}

		let balance_key = self._player_balance_key(&winner);
		let mut balance = self.storage_load_big_uint(balance_key.as_bytes());
		balance += prize;
		self.storage_store_big_uint(balance_key.as_bytes(), &balance);

		self.reward_winner_event(game_index, &winner, &prize);

		Ok(())
	}

	// owner transfers prize into winner SC account, then transfers funds to player wallet
	#[endpoint]
	fn rewardAndSendToWallet(
		&self,
		game_index: &BigUint,
		winner: &Address,
		prize: &BigUint,
	) -> SCResult<()> {
		sc_try!(self.rewardWinner(game_index, winner, prize));
		sc_try!(self._transferBackToPlayerWallet(winner, prize));
		Ok(())
	}

	#[event("0x1000000000000000000000000000000000000000000000000000000000000001")]
	fn top_up_event(&self, player: &Address, amount: &BigUint);

	#[event("0x1000000000000000000000000000000000000000000000000000000000000002")]
	fn withdraw_event(&self, player: &Address, amount: &BigUint);

	#[event("0x1000000000000000000000000000000000000000000000000000000000000003")]
	fn player_joins_game_event(&self, game_index: &BigUint, player: &Address, bet: &BigUint);

	#[event("0x1000000000000000000000000000000000000000000000000000000000000004")]
	fn reward_winner_event(&self, game_index: &BigUint, winner: &Address, prize: &BigUint);
}
