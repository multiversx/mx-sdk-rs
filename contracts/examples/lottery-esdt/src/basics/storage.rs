use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("ticketHolder")]
    fn ticket_holders(&self, lottery_name: &ManagedBuffer) -> VecMapper<u64>;

    #[storage_mapper("accumulatedRewards")]
    fn accumulated_rewards(
        &self,
        token_id: &TokenIdentifier,
        user_id: &u64,
    ) -> SingleValueMapper<BigUint>;

    #[storage_mapper("totalWinning_tickets")]
    fn total_winning_tickets(&self, lottery_name: &ManagedBuffer) -> SingleValueMapper<usize>;

    #[storage_mapper("indexLastWinner")]
    fn index_last_winner(&self, lottery_name: &ManagedBuffer) -> SingleValueMapper<usize>;

    #[storage_mapper("accumulatedRewards")]
    fn user_accumulated_token_rewards(&self, user_id: &u64) -> UnorderedSetMapper<TokenIdentifier>;

    #[storage_mapper("numberOfEntriesForUser")]
    fn number_of_entries_for_user(
        &self,
        lottery_name: &ManagedBuffer,
        user_id: &u64,
    ) -> SingleValueMapper<usize>;

    #[storage_mapper("addressToIdMapper")]
    fn address_to_id_mapper(&self) -> AddressToIdMapper;

    #[storage_mapper("burnPercentageForLottery")]
    fn burn_percentage_for_lottery(
        &self,
        lottery_name: &ManagedBuffer,
    ) -> SingleValueMapper<BigUint>;
}
