#![no_std]

multiversx_sc::imports!();

mod lottery_info;
mod status;

use lottery_info::LotteryInfo;
use status::Status;

const PERCENTAGE_TOTAL: u32 = 100;
const THIRTY_DAYS_IN_SECONDS: u64 = 60 * 60 * 24 * 30;
const MAX_TICKETS: usize = 800;

#[multiversx_sc::contract]
pub trait Lottery {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn start(
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
            self.status(&lottery_name) == Status::Inactive,
            "Lottery is already active!"
        );
        require!(!lottery_name.is_empty(), "Can't have empty lottery name!");
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
                mapper.insert(addr);
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
    fn determine_winner(&self, lottery_name: ManagedBuffer) {
        match self.status(&lottery_name) {
            Status::Inactive => sc_panic!("Lottery is inactive!"),
            Status::Running => sc_panic!("Lottery is still running!"),
            Status::Ended => {
                self.distribute_prizes(&lottery_name);
                self.clear_storage(&lottery_name);
            },
        };
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
        let whitelist = self.lottery_whitelist(lottery_name);

        require!(
            whitelist.is_empty() || whitelist.contains(&caller),
            "You are not allowed to participate in this lottery!"
        );
        require!(
            token_identifier == &info.token_identifier && payment == &info.ticket_price,
            "Wrong ticket fee!"
        );

        let entries_mapper = self.number_of_entries_for_user(lottery_name, &caller);
        let mut entries = entries_mapper.get();
        require!(
            entries < info.max_entries_per_user,
            "Ticket limit exceeded for this lottery!"
        );

        self.ticket_holders(lottery_name).push(&caller);

        entries += 1;
        info.tickets_left -= 1;
        info.prize_pool += &info.ticket_price;

        entries_mapper.set(entries);
        info_mapper.set(&info);
    }

    fn distribute_prizes(&self, lottery_name: &ManagedBuffer) {
        let mut info = self.lottery_info(lottery_name).get();
        let ticket_holders_mapper = self.ticket_holders(lottery_name);
        let total_tickets = ticket_holders_mapper.len();

        if total_tickets == 0 {
            return;
        }

        let burn_percentage = self.burn_percentage_for_lottery(lottery_name).get();
        if burn_percentage > 0 {
            let burn_amount = self.calculate_percentage_of(&info.prize_pool, &burn_percentage);

            // Prevent crashing if the role was unset while the lottery was running
            // The tokens will simply remain locked forever
            let esdt_token_id = info.token_identifier.clone().unwrap_esdt();
            let roles = self.blockchain().get_esdt_local_roles(&esdt_token_id);
            if roles.has_role(&EsdtLocalRole::Burn) {
                self.send().esdt_local_burn(&esdt_token_id, 0, &burn_amount);
            }

            info.prize_pool -= burn_amount;
        }

        // if there are less tickets than the distributed prize pool,
        // the 1st place gets the leftover, maybe could split between the remaining
        // but this is a rare case anyway and it's not worth the overhead
        let total_winning_tickets = if total_tickets < info.prize_distribution.len() {
            total_tickets
        } else {
            info.prize_distribution.len()
        };
        let total_prize = info.prize_pool.clone();
        let winning_tickets = self.get_distinct_random(1, total_tickets, total_winning_tickets);

        // distribute to the first place last. Laws of probability say that order doesn't matter.
        // this is done to mitigate the effects of BigUint division leading to "spare" prize money being left out at times
        // 1st place will get the spare money instead.
        for i in (1..total_winning_tickets).rev() {
            let winning_ticket_id = winning_tickets[i];
            let winner_address = ticket_holders_mapper.get(winning_ticket_id);
            let prize = self.calculate_percentage_of(
                &total_prize,
                &BigUint::from(info.prize_distribution.get(i)),
            );

            self.send()
                .direct(&winner_address, &info.token_identifier, 0, &prize);
            info.prize_pool -= prize;
        }

        // send leftover to first place
        let first_place_winner = ticket_holders_mapper.get(winning_tickets[0]);
        self.send().direct(
            &first_place_winner,
            &info.token_identifier,
            0,
            &info.prize_pool,
        );
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
    fn get_distinct_random(
        &self,
        min: usize,
        max: usize,
        amount: usize,
    ) -> ArrayVec<usize, MAX_TICKETS> {
        let mut rand_numbers = ArrayVec::new();

        for num in min..=max {
            rand_numbers.push(num);
        }

        let total_numbers = rand_numbers.len();
        let mut rand = RandomnessSource::new();

        for i in 0..amount {
            let rand_index = rand.next_usize_in_range(0, total_numbers);
            rand_numbers.swap(i, rand_index);
        }

        rand_numbers
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
    fn lottery_whitelist(&self, lottery_name: &ManagedBuffer)
        -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("ticketHolder")]
    fn ticket_holders(&self, lottery_name: &ManagedBuffer) -> VecMapper<ManagedAddress>;

    #[storage_mapper("numberOfEntriesForUser")]
    fn number_of_entries_for_user(
        &self,
        lottery_name: &ManagedBuffer,
        user: &ManagedAddress,
    ) -> SingleValueMapper<usize>;

    #[storage_mapper("burnPercentageForLottery")]
    fn burn_percentage_for_lottery(
        &self,
        lottery_name: &ManagedBuffer,
    ) -> SingleValueMapper<BigUint>;
}
