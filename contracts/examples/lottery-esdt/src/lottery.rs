#![no_std]

use multiversx_sc::imports::*;

mod awarding_status;
mod lottery_info;
mod status;

use awarding_status::AwardingStatus;
use lottery_info::LotteryInfo;
use status::Status;

const PERCENTAGE_TOTAL: u32 = 100;
const THIRTY_DAYS_IN_SECONDS: u64 = 60 * 60 * 24 * 30;
const MAX_TICKETS: usize = 800;
const MAX_OPERATIONS: usize = 50;

#[multiversx_sc::contract]
pub trait Lottery {
    #[init]
    fn init(&self) {}

    #[allow_multiple_var_args]
    #[endpoint(createLotteryPool)]
    fn create_lottery_pool(
        &self,
        lottery_name: ManagedBuffer,
        token_identifier: EgldOrEsdtTokenIdentifier,
        ticket_price: BigUint,
        opt_total_tickets: Option<usize>,
        opt_deadline: Option<u64>,
        opt_max_entries_per_user: Option<usize>,
        opt_prize_distribution: ManagedOption<ManagedVec<u8>>,
        opt_whitelist: ManagedOption<ManagedVec<ManagedAddress>>,
        opt_burn_percentage: OptionalValue<BigUint>,
    ) {
        self.start_lottery(
            lottery_name,
            token_identifier,
            ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution,
            opt_whitelist,
            opt_burn_percentage,
        );
    }

    #[allow_multiple_var_args]
    #[allow(clippy::too_many_arguments)]
    fn start_lottery(
        &self,
        lottery_name: ManagedBuffer,
        token_identifier: EgldOrEsdtTokenIdentifier,
        ticket_price: BigUint,
        opt_total_tickets: Option<usize>,
        opt_deadline: Option<u64>,
        opt_max_entries_per_user: Option<usize>,
        opt_prize_distribution: ManagedOption<ManagedVec<u8>>,
        opt_whitelist: ManagedOption<ManagedVec<ManagedAddress>>,
        opt_burn_percentage: OptionalValue<BigUint>,
    ) {
        require!(!lottery_name.is_empty(), "Name can't be empty!");

        let timestamp = self.blockchain().get_block_timestamp();
        let total_tickets = opt_total_tickets.unwrap_or(MAX_TICKETS);
        let deadline = opt_deadline.unwrap_or(timestamp + THIRTY_DAYS_IN_SECONDS);
        let max_entries_per_user = opt_max_entries_per_user.unwrap_or(MAX_TICKETS);
        let prize_distribution = opt_prize_distribution
            .unwrap_or_else(|| ManagedVec::from_single_item(PERCENTAGE_TOTAL as u8));

        require!(
            total_tickets > prize_distribution.len(),
            "Number of winners should be smaller than the number of available tickets"
        );
        require!(
            self.status(&lottery_name) == Status::Inactive,
            "Lottery is already active!"
        );
        require!(token_identifier.is_valid(), "Invalid token name provided!");
        require!(ticket_price > 0, "Ticket price must be higher than 0!");
        require!(
            total_tickets > 0,
            "Must have more than 0 tickets available!"
        );
        require!(
            total_tickets <= MAX_TICKETS,
            "Only 800 or less total tickets per lottery are allowed!"
        );
        require!(deadline > timestamp, "Deadline can't be in the past!");
        require!(
            deadline <= timestamp + THIRTY_DAYS_IN_SECONDS,
            "Deadline can't be later than 30 days from now!"
        );
        require!(
            max_entries_per_user > 0,
            "Must have more than 0 max entries per user!"
        );
        require!(
            self.sum_array(&prize_distribution) == PERCENTAGE_TOTAL,
            "Prize distribution must add up to exactly 100(%)!"
        );

        match opt_burn_percentage {
            OptionalValue::Some(burn_percentage) => {
                require!(!token_identifier.is_egld(), "EGLD can't be burned!");

                let roles = self
                    .blockchain()
                    .get_esdt_local_roles(&token_identifier.clone().unwrap_esdt());
                require!(
                    roles.has_role(&EsdtLocalRole::Burn),
                    "The contract can't burn the selected token!"
                );

                require!(
                    burn_percentage < PERCENTAGE_TOTAL,
                    "Invalid burn percentage!"
                );
                self.burn_percentage_for_lottery(&lottery_name)
                    .set(burn_percentage);
            },
            OptionalValue::None => {},
        }

        if let Some(whitelist) = opt_whitelist.as_option() {
            let mut mapper = self.lottery_whitelist(&lottery_name);
            for addr in &*whitelist {
                let addr_id = self.addres_to_id_mapper().get_id_or_insert(&addr);
                mapper.insert(addr_id);
            }
        }

        let info = LotteryInfo {
            token_identifier,
            ticket_price,
            tickets_left: total_tickets,
            deadline,
            max_entries_per_user,
            prize_distribution,
            prize_pool: BigUint::zero(),
            unawarded_amount: BigUint::zero(),
        };

        self.lottery_info(&lottery_name).set(&info);
    }

    #[endpoint]
    #[payable("*")]
    fn buy_ticket(&self, lottery_name: ManagedBuffer) {
        let (token_identifier, payment) = self.call_value().egld_or_single_fungible_esdt();

        match self.status(&lottery_name) {
            Status::Inactive => sc_panic!("Lottery is currently inactive."),
            Status::Running => {
                self.update_after_buy_ticket(&lottery_name, &token_identifier, &payment)
            },
            Status::Ended => {
                sc_panic!("Lottery entry period has ended! Awaiting winner announcement.")
            },
        };
    }

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
        if self.total_winning_tickets(&lottery_name).is_empty() {
            self.prepare_awarding(&lottery_name);
        }
        self.distribute_prizes(&lottery_name)
    }

    #[view]
    fn status(&self, lottery_name: &ManagedBuffer) -> Status {
        if self.lottery_info(lottery_name).is_empty() {
            return Status::Inactive;
        }

        let info = self.lottery_info(lottery_name).get();
        let current_time = self.blockchain().get_block_timestamp();
        if current_time > info.deadline || info.tickets_left == 0 {
            return Status::Ended;
        }

        Status::Running
    }

    fn update_after_buy_ticket(
        &self,
        lottery_name: &ManagedBuffer,
        token_identifier: &EgldOrEsdtTokenIdentifier,
        payment: &BigUint,
    ) {
        let info_mapper = self.lottery_info(lottery_name);
        let mut info = info_mapper.get();
        let caller = self.blockchain().get_caller();
        let caller_id = self.addres_to_id_mapper().get_id_or_insert(&caller);
        let whitelist = self.lottery_whitelist(lottery_name);

        require!(
            whitelist.is_empty() || whitelist.contains(&caller_id),
            "You are not allowed to participate in this lottery!"
        );
        require!(
            token_identifier == &info.token_identifier && payment == &info.ticket_price,
            "Wrong ticket fee!"
        );

        let entries_mapper = self.number_of_entries_for_user(lottery_name, &caller_id);
        let mut entries = entries_mapper.get();
        require!(
            entries < info.max_entries_per_user,
            "Ticket limit exceeded for this lottery!"
        );

        self.ticket_holders(lottery_name).push(&caller_id);

        entries += 1;
        info.tickets_left -= 1;
        info.prize_pool += &info.ticket_price;
        info.unawarded_amount += &info.ticket_price;

        entries_mapper.set(entries);
        info_mapper.set(&info);
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
        let esdt_token_id = info.token_identifier.clone().unwrap_esdt();
        let roles = self.blockchain().get_esdt_local_roles(&esdt_token_id);
        if roles.has_role(&EsdtLocalRole::Burn) {
            self.send().esdt_local_burn(&esdt_token_id, 0, &burn_amount);
        }

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
        self.lottery_info(lottery_name).set(info);
        self.index_last_winner(lottery_name).set(index_last_winner);
        if index_last_winner > total_winning_tickets {
            self.clear_storage(&lottery_name);
            return AwardingStatus::Finished;
        }
        AwardingStatus::Ongoing
    }

    fn award_winner(
        &self,
        lottery_name: &ManagedBuffer,
        index_last_winner: &usize,
        total_tickets: usize,
        total_winning_tickets: usize,
        info: &mut LotteryInfo<Self::Api>,
    ) {
        let rand_index = self.get_distinct_random(*index_last_winner, total_tickets);
        let ticket_holders_mapper = self.ticket_holders(lottery_name);

        // swap indexes of the winner addresses - we are basically bringing the winners in the first indexes of the mapper
        let winner_address = self.ticket_holders(lottery_name).get(rand_index);
        let last_index_winner_address = self.ticket_holders(lottery_name).get(*index_last_winner);

        self.ticket_holders(lottery_name)
            .set(rand_index, &last_index_winner_address);
        self.ticket_holders(lottery_name)
            .set(*index_last_winner, &winner_address);

        // distribute to the first place last. Laws of probability say that order doesn't matter.
        // this is done to mitigate the effects of BigUint division leading to "spare" prize money being left out at times
        // 1st place will get the spare money instead.
        if *index_last_winner <= total_winning_tickets {
            let prize = self.calculate_percentage_of(
                &info.prize_pool,
                &BigUint::from(
                    info.prize_distribution
                        .get(total_winning_tickets - *index_last_winner),
                ),
            );
            if prize > 0 {
                self.assign_prize_to_winner(info.token_identifier.clone(), &prize, &winner_address);

                info.unawarded_amount -= prize;
            }
        } else {
            // insert token in accumulated rewards first place
            let first_place_winner = ticket_holders_mapper.get(*index_last_winner);

            self.assign_prize_to_winner(
                info.token_identifier.clone(),
                &info.unawarded_amount,
                &first_place_winner,
            );
        }
    }

    fn assign_prize_to_winner(
        &self,
        token_id: EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
        winner_id: &u64,
    ) {
        self.accumulated_rewards(&token_id, winner_id)
            .update(|value| *value += amount);
        self.user_accumulated_token_rewards(winner_id)
            .insert(token_id);
    }

    #[endpoint]
    fn claim_rewards(&self, tokens: MultiValueEncoded<EgldOrEsdtTokenIdentifier>) {
        let caller = self.blockchain().get_caller();
        let caller_id = self.addres_to_id_mapper().get_id_or_insert(&caller);
        require!(
            !self.user_accumulated_token_rewards(&caller_id).is_empty(),
            "You have no rewards to claim"
        );

        let mut accumulated_egld_rewards = BigUint::zero();
        let mut accumulated_esdt_rewards = ManagedVec::<Self::Api, EsdtTokenPayment>::new();

        // to save reviewers time, these 2 iterators have different generics, so it was not possible to make just 1 for loop

        if tokens.is_empty() {
            // if wanted tokens were not specified claim all, and clear user_accumulated_token_rewards storage mapper

            let mut all_tokens: ManagedVec<Self::Api, EgldOrEsdtTokenIdentifier> =
                ManagedVec::new();

            for token_id in self.user_accumulated_token_rewards(&caller_id).iter() {
                require!(
                    !self.accumulated_rewards(&token_id, &caller_id).is_empty(),
                    "Token requested not available for claim"
                );
                all_tokens.push(token_id);
            }

            self.claim_rewards_user(
                all_tokens,
                &caller_id,
                &mut accumulated_egld_rewards,
                &mut accumulated_esdt_rewards,
            )
        } else {
            // otherwise claim just what was requested and remove those tokens from the user_accumulated_token_rewards storage mapper

            self.claim_rewards_user(
                tokens.to_vec(),
                &caller_id,
                &mut accumulated_egld_rewards,
                &mut accumulated_esdt_rewards,
            )
        };
        if !accumulated_esdt_rewards.is_empty() {
            self.tx()
                .to(&caller)
                .multi_esdt(accumulated_esdt_rewards)
                .transfer();
        }

        if accumulated_egld_rewards > 0u64 {
            self.tx()
                .to(&caller)
                .egld(accumulated_egld_rewards)
                .transfer();
        }
    }

    fn claim_rewards_user(
        &self,
        tokens: ManagedVec<Self::Api, EgldOrEsdtTokenIdentifier>,
        caller_id: &u64,
        accumulated_egld_rewards: &mut BigUint,
        accumulated_esdt_rewards: &mut ManagedVec<Self::Api, EsdtTokenPayment>,
    ) {
        for token_id in tokens.iter().rev() {
            let _ = &self
                .user_accumulated_token_rewards(caller_id)
                .swap_remove(&token_id);

            self.prepare_token_for_claim(
                token_id,
                caller_id,
                accumulated_egld_rewards,
                accumulated_esdt_rewards,
            );
        }
    }

    fn prepare_token_for_claim(
        &self,
        token_id: EgldOrEsdtTokenIdentifier,
        caller_id: &u64,
        accumulated_egld_rewards: &mut BigUint,
        accumulated_esdt_rewards: &mut ManagedVec<Self::Api, EsdtTokenPayment>,
    ) {
        let value = self.accumulated_rewards(&token_id, caller_id).take();
        if token_id.is_egld() {
            *accumulated_egld_rewards += value;
        } else {
            accumulated_esdt_rewards.push(EsdtTokenPayment::new(token_id.unwrap_esdt(), 0, value));
        }
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

    fn sum_array(&self, array: &ManagedVec<u8>) -> u32 {
        let mut sum = 0;

        for item in array {
            sum += item as u32;
        }

        sum
    }

    /// does not check if max - min >= amount, that is the caller's job
    fn get_distinct_random(&self, min: usize, max: usize) -> usize {
        let mut rand = RandomnessSource::new();
        rand.next_usize_in_range(min, max)
    }

    fn calculate_percentage_of(&self, value: &BigUint, percentage: &BigUint) -> BigUint {
        value * percentage / PERCENTAGE_TOTAL
    }

    // storage

    #[view(getLotteryInfo)]
    #[storage_mapper("lotteryInfo")]
    fn lottery_info(
        &self,
        lottery_name: &ManagedBuffer,
    ) -> SingleValueMapper<LotteryInfo<Self::Api>>;

    #[view(getLotteryWhitelist)]
    #[storage_mapper("lotteryWhitelist")]
    fn lottery_whitelist(&self, lottery_name: &ManagedBuffer) -> UnorderedSetMapper<u64>;

    #[storage_mapper("ticketHolder")]
    fn ticket_holders(&self, lottery_name: &ManagedBuffer) -> VecMapper<u64>;

    #[storage_mapper("accumulatedRewards")]
    fn accumulated_rewards(
        &self,
        token_id: &EgldOrEsdtTokenIdentifier,
        user_id: &u64,
    ) -> SingleValueMapper<BigUint>;

    #[storage_mapper("totalWinning_tickets")]
    fn total_winning_tickets(&self, lottery_name: &ManagedBuffer) -> SingleValueMapper<usize>;

    #[storage_mapper("indexLastWinner")]
    fn index_last_winner(&self, lottery_name: &ManagedBuffer) -> SingleValueMapper<usize>;

    #[storage_mapper("accumulatedRewards")]
    fn user_accumulated_token_rewards(
        &self,
        user_id: &u64,
    ) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier>;

    #[storage_mapper("numberOfEntriesForUser")]
    fn number_of_entries_for_user(
        &self,
        lottery_name: &ManagedBuffer,
        user_id: &u64,
    ) -> SingleValueMapper<usize>;

    #[storage_mapper("addressToIdMapper")]
    fn addres_to_id_mapper(&self) -> AddressToIdMapper;

    #[storage_mapper("burnPercentageForLottery")]
    fn burn_percentage_for_lottery(
        &self,
        lottery_name: &ManagedBuffer,
    ) -> SingleValueMapper<BigUint>;
}
