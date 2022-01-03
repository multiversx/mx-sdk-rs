#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait CryptoBubbles {
    /// constructor function
    /// is called immediately after the contract is created
    #[init]
    fn init(&self) {}

    /// player adds funds
    #[payable("EGLD")]
    #[endpoint(topUp)]
    fn top_up(&self, #[payment] payment: BigUint) {
        let caller = self.blockchain().get_caller_legacy();
        self.player_balance(&caller)
            .update(|balance| *balance += &payment);

        self.top_up_event(&caller, &payment);
    }

    /// player withdraws funds
    #[endpoint]
    fn withdraw(&self, amount: &BigUint) -> SCResult<()> {
        self.transfer_back_to_player_wallet(&self.blockchain().get_caller_legacy(), amount)
    }

    /// server calls withdraw on behalf of the player
    fn transfer_back_to_player_wallet(&self, player: &Address, amount: &BigUint) -> SCResult<()> {
        self.player_balance(player).update(|balance| {
            require!(
                amount <= balance,
                "amount to withdraw must be less or equal to balance"
            );

            *balance -= amount;

            Ok(())
        })?;

        self.send()
            .direct_egld(&player.into(), amount, b"crypto bubbles");

        self.withdraw_event(player, amount);

        Ok(())
    }

    /// player joins game
    fn add_player_to_game_state_change(
        &self,
        game_index: &BigUint,
        player: &Address,
        bet: &BigUint,
    ) -> SCResult<()> {
        self.player_balance(player).update(|balance| {
            require!(bet <= balance, "insufficient funds to join game");

            *balance -= bet;

            Ok(())
        })?;

        self.player_joins_game_event(game_index, player, bet);

        Ok(())
    }

    // player tops up + joins a game
    #[payable("EGLD")]
    #[endpoint(joinGame)]
    fn join_game(&self, game_index: BigUint, #[payment] bet: BigUint) -> SCResult<()> {
        let player = self.blockchain().get_caller_legacy();
        self.top_up(bet.clone());
        self.add_player_to_game_state_change(&game_index, &player, &bet)
    }

    // owner transfers prize into winner SC account
    #[only_owner]
    #[endpoint(rewardWinner)]
    fn reward_winner(
        &self,
        game_index: &BigUint,
        winner: &Address,
        prize: &BigUint,
    ) -> SCResult<()> {
        self.player_balance(winner)
            .update(|balance| *balance += prize);

        self.reward_winner_event(game_index, winner, prize);

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
        self.reward_winner(game_index, winner, prize)?;
        self.transfer_back_to_player_wallet(winner, prize)?;
        Ok(())
    }

    // Storage

    #[view(balanceOf)]
    #[storage_mapper("playerBalance")]
    fn player_balance(&self, player: &Address) -> SingleValueMapper<BigUint>;

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
