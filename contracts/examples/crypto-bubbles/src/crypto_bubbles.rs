#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait CryptoBubbles {
    /// constructor function
    /// is called immediately after the contract is created
    #[init]
    fn init(&self) {}

    /// player adds funds
    #[payable("EGLD")]
    #[endpoint(topUp)]
    fn top_up(&self) {
        let payment = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();
        self.player_balance(&caller)
            .update(|balance| *balance += &*payment);

        self.top_up_event(&caller, &payment);
    }

    /// player withdraws funds
    #[endpoint]
    fn withdraw(&self, amount: &BigUint) {
        self.transfer_back_to_player_wallet(&self.blockchain().get_caller(), amount)
    }

    /// server calls withdraw on behalf of the player
    fn transfer_back_to_player_wallet(&self, player: &ManagedAddress, amount: &BigUint) {
        self.player_balance(player).update(|balance| {
            require!(
                amount <= balance,
                "amount to withdraw must be less or equal to balance"
            );

            *balance -= amount;
        });

        self.send().direct_egld(player, amount);

        self.withdraw_event(player, amount);
    }

    /// player joins game
    fn add_player_to_game_state_change(
        &self,
        game_index: &BigUint,
        player: &ManagedAddress,
        bet: &BigUint,
    ) {
        self.player_balance(player).update(|balance| {
            require!(bet <= balance, "insufficient funds to join game");

            *balance -= bet;
        });

        self.player_joins_game_event(game_index, player, bet);
    }

    // player tops up + joins a game
    #[payable("EGLD")]
    #[endpoint(joinGame)]
    fn join_game(&self, game_index: BigUint) {
        let bet = self.call_value().egld_value();
        let player = self.blockchain().get_caller();
        self.top_up();
        self.add_player_to_game_state_change(&game_index, &player, &bet)
    }

    // owner transfers prize into winner SC account
    #[only_owner]
    #[endpoint(rewardWinner)]
    fn reward_winner(&self, game_index: &BigUint, winner: &ManagedAddress, prize: &BigUint) {
        self.player_balance(winner)
            .update(|balance| *balance += prize);

        self.reward_winner_event(game_index, winner, prize);
    }

    // owner transfers prize into winner SC account, then transfers funds to player wallet
    #[endpoint(rewardAndSendToWallet)]
    fn reward_and_send_to_wallet(
        &self,
        game_index: &BigUint,
        winner: &ManagedAddress,
        prize: &BigUint,
    ) {
        self.reward_winner(game_index, winner, prize);
        self.transfer_back_to_player_wallet(winner, prize);
    }

    // Storage

    #[view(balanceOf)]
    #[storage_mapper("playerBalance")]
    fn player_balance(&self, player: &ManagedAddress) -> SingleValueMapper<BigUint>;

    // Events

    #[event("top_up")]
    fn top_up_event(&self, #[indexed] player: &ManagedAddress, amount: &BigUint);

    #[event("withdraw")]
    fn withdraw_event(&self, #[indexed] player: &ManagedAddress, amount: &BigUint);

    #[event("player_joins_game")]
    fn player_joins_game_event(
        &self,
        #[indexed] game_index: &BigUint,
        #[indexed] player: &ManagedAddress,
        bet: &BigUint,
    );

    #[event("reward_winner")]
    fn reward_winner_event(
        &self,
        #[indexed] game_index: &BigUint,
        #[indexed] winner: &ManagedAddress,
        prize: &BigUint,
    );
}
