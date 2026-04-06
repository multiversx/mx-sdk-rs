use crate::{
    AwardingStatus, Status, constants::MAX_OPERATIONS, lottery_info::LotteryInfo, storage, utils,
    views,
};
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait AwardingModule: views::ViewsModule + storage::StorageModule + utils::UtilsModule {
    #[endpoint]
    fn determine_winner(&self, lottery_name: ManagedBuffer) -> AwardingStatus {
        let sc_address = self.blockchain().get_sc_address();
        let sc_address_shard = self.blockchain().get_shard_of_address(&sc_address);
        let caller = self.blockchain().get_caller();
        let caller_shard = self.blockchain().get_shard_of_address(&caller);
        require!(
            sc_address_shard != caller_shard,
            "Caller needs to be on a remote shard"
        );

        match self.status(&lottery_name) {
            Status::Inactive => sc_panic!("Lottery is inactive!"),
            Status::Running => sc_panic!("Lottery is still running!"),
            Status::Ended => self.handle_awarding(&lottery_name),
        }
    }

    fn handle_awarding(&self, lottery_name: &ManagedBuffer) -> AwardingStatus {
        if self.total_winning_tickets(lottery_name).is_empty() {
            self.prepare_awarding(lottery_name);
        }
        self.distribute_prizes(lottery_name)
    }

    fn prepare_awarding(&self, lottery_name: &ManagedBuffer) {
        let mut info = self.lottery_info(lottery_name).get();
        let ticket_holders_mapper = self.ticket_holders(lottery_name);
        let total_tickets = ticket_holders_mapper.len();

        if total_tickets == 0 {
            return;
        }

        self.burn_prize_percentage(lottery_name, &mut info);

        // if there are less tickets than the distributed prize pool,
        // the 1st place gets the leftover, maybe could split between the remaining
        // but this is a rare case anyway and it's not worth the overhead
        let total_winning_tickets = if total_tickets < info.prize_distribution.len() {
            total_tickets
        } else {
            info.prize_distribution.len()
        };

        self.total_winning_tickets(lottery_name)
            .set(total_winning_tickets);
        self.index_last_winner(lottery_name).set(1);

        self.lottery_info(lottery_name).set(info);
    }

    fn burn_prize_percentage(
        &self,
        lottery_name: &ManagedBuffer,
        info: &mut LotteryInfo<Self::Api>,
    ) {
        let burn_percentage = self.burn_percentage_for_lottery(lottery_name).get();
        if burn_percentage == 0 {
            return;
        }

        let burn_amount = self.calculate_percentage_of(&info.prize_pool, &burn_percentage);

        // Prevent crashing if the role was unset while the lottery was running
        // The tokens will simply remain locked forever
        if let Some(esdt_token_id) = info.token_id.as_esdt() {
            let roles = self.blockchain().get_esdt_local_roles(esdt_token_id);
            if roles.has_role(&EsdtLocalRole::Burn) {
                self.send().esdt_local_burn(esdt_token_id, 0, &burn_amount);
            }
        };

        info.prize_pool -= &burn_amount;
        info.unawarded_amount -= burn_amount;
    }

    fn distribute_prizes(&self, lottery_name: &ManagedBuffer) -> AwardingStatus {
        let mut info = self.lottery_info(lottery_name).get();
        let ticket_holders_mapper = self.ticket_holders(lottery_name);
        let total_tickets = ticket_holders_mapper.len();

        let mut index_last_winner = self.index_last_winner(lottery_name).get();
        let total_winning_tickets = self.total_winning_tickets(lottery_name).get();
        require!(
            index_last_winner <= total_winning_tickets,
            "Awarding has ended"
        );

        let mut iterations = 0;
        while index_last_winner <= total_winning_tickets && iterations < MAX_OPERATIONS {
            self.award_winner(
                lottery_name,
                &index_last_winner,
                total_tickets,
                total_winning_tickets,
                &mut info,
            );
            index_last_winner += 1;
            iterations += 1;
        }
        self.index_last_winner(lottery_name).set(index_last_winner);
        if index_last_winner > total_winning_tickets {
            // give any leftover unawarded_amount (from BigUint division rounding) to the 1st place winner
            if total_winning_tickets > 0 && info.unawarded_amount > 0u32 {
                let first_place_winner_id =
                    self.ticket_holders(lottery_name).get(total_winning_tickets);
                self.assign_prize_to_winner(
                    info.token_id.clone(),
                    &info.unawarded_amount,
                    &first_place_winner_id,
                );
            }
            self.clear_storage(lottery_name);
            return AwardingStatus::Finished;
        }
        self.lottery_info(lottery_name).set(info);
        AwardingStatus::Ongoing
    }

    fn clear_storage(&self, lottery_name: &ManagedBuffer) {
        let mut ticket_holders_mapper = self.ticket_holders(lottery_name);
        let current_ticket_number = ticket_holders_mapper.len();

        for i in 1..=current_ticket_number {
            let addr = ticket_holders_mapper.get(i);
            self.number_of_entries_for_user(lottery_name, &addr).clear();
        }

        ticket_holders_mapper.clear();
        self.lottery_info(lottery_name).clear();
        self.lottery_whitelist(lottery_name).clear();
        self.total_winning_tickets(lottery_name).clear();
        self.index_last_winner(lottery_name).clear();
        self.burn_percentage_for_lottery(lottery_name).clear();
    }

    fn award_winner(
        &self,
        lottery_name: &ManagedBuffer,
        index_last_winner: &usize,
        total_tickets: usize,
        total_winning_tickets: usize,
        info: &mut LotteryInfo<Self::Api>,
    ) {
        let rand_index = self.get_distinct_random(*index_last_winner, total_tickets + 1);

        // swap indexes of the winner addresses - we are basically bringing the winners in the first indexes of the mapper
        let winner_address = self.ticket_holders(lottery_name).get(rand_index);
        let last_index_winner_address = self.ticket_holders(lottery_name).get(*index_last_winner);

        self.ticket_holders(lottery_name)
            .set(rand_index, &last_index_winner_address);
        self.ticket_holders(lottery_name)
            .set(*index_last_winner, &winner_address);

        // distribute to the first place last. Laws of probability say that order doesn't matter.
        // this is done to mitigate the effects of BigUint division leading to "spare" prize money being left out at times
        // 1st place will get the spare money instead (handled in distribute_prizes after the loop).
        let prize = self.calculate_percentage_of(
            &info.prize_pool,
            &BigUint::from(
                info.prize_distribution
                    .get(total_winning_tickets - *index_last_winner),
            ),
        );
        if prize == 0 {
            return;
        }

        self.assign_prize_to_winner(info.token_id.clone(), &prize, &winner_address);

        info.unawarded_amount -= prize;
    }

    fn assign_prize_to_winner(&self, token_id: TokenId, amount: &BigUint, winner_id: &u64) {
        self.accumulated_rewards(&token_id, winner_id)
            .update(|value| *value += amount);
        self.user_accumulated_token_rewards(winner_id)
            .insert(token_id);
    }
}
