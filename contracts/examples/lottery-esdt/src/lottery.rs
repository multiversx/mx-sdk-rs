#![no_std]

elrond_wasm::imports!();

mod lottery_info;
mod random;
mod status;

use lottery_info::LotteryInfo;
use random::Random;
use status::Status;

const PERCENTAGE_TOTAL: u32 = 100;
const THIRTY_DAYS_IN_SECONDS: u64 = 60 * 60 * 24 * 30;
const MAX_TICKETS: u32 = 800;

#[elrond_wasm::contract]
pub trait Lottery {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn start(
        &self,
        lottery_name: BoxedBytes,
        token_identifier: TokenIdentifier,
        ticket_price: BigUint,
        opt_total_tickets: Option<u32>,
        opt_deadline: Option<u64>,
        opt_max_entries_per_user: Option<u32>,
        opt_prize_distribution: Option<Vec<u8>>,
        opt_whitelist: Option<Vec<ManagedAddress>>,
        #[var_args] opt_burn_percentage: OptionalArg<BigUint>,
    ) -> SCResult<()> {
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
        )
    }

    #[endpoint(createLotteryPool)]
    fn create_lottery_pool(
        &self,
        lottery_name: BoxedBytes,
        token_identifier: TokenIdentifier,
        ticket_price: BigUint,
        opt_total_tickets: Option<u32>,
        opt_deadline: Option<u64>,
        opt_max_entries_per_user: Option<u32>,
        opt_prize_distribution: Option<Vec<u8>>,
        opt_whitelist: Option<Vec<ManagedAddress>>,
        #[var_args] opt_burn_percentage: OptionalArg<BigUint>,
    ) -> SCResult<()> {
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
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn start_lottery(
        &self,
        lottery_name: BoxedBytes,
        token_identifier: TokenIdentifier,
        ticket_price: BigUint,
        opt_total_tickets: Option<u32>,
        opt_deadline: Option<u64>,
        opt_max_entries_per_user: Option<u32>,
        opt_prize_distribution: Option<Vec<u8>>,
        opt_whitelist: Option<Vec<ManagedAddress>>,
        #[var_args] opt_burn_percentage: OptionalArg<BigUint>,
    ) -> SCResult<()> {
        require!(!lottery_name.is_empty(), "Name can't be empty!");

        let timestamp = self.blockchain().get_block_timestamp();
        let total_tickets = opt_total_tickets.unwrap_or(MAX_TICKETS);
        let deadline = opt_deadline.unwrap_or_else(|| timestamp + THIRTY_DAYS_IN_SECONDS);
        let max_entries_per_user = opt_max_entries_per_user.unwrap_or(MAX_TICKETS);
        let prize_distribution =
            opt_prize_distribution.unwrap_or_else(|| [PERCENTAGE_TOTAL as u8].to_vec());
        let whitelist = opt_whitelist.unwrap_or_default();

        require!(
            self.status(&lottery_name) == Status::Inactive,
            "Lottery is already active!"
        );
        require!(!lottery_name.is_empty(), "Can't have empty lottery name!");
        require!(
            token_identifier.is_egld() || token_identifier.is_valid_esdt_identifier(),
            "Invalid token name provided!"
        );
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
            OptionalArg::Some(burn_percentage) => {
                require!(!token_identifier.is_egld(), "EGLD can't be burned!");

                let roles = self.blockchain().get_esdt_local_roles(&token_identifier);
                require!(
                    roles.has_role(&EsdtLocalRole::Burn),
                    "The contract can't burn the selected token!"
                );

                require!(
                    burn_percentage < PERCENTAGE_TOTAL,
                    "Invalid burn percentage!"
                );
                self.burn_percentage_for_lottery(&lottery_name)
                    .set(&burn_percentage);
            },
            OptionalArg::None => {},
        }

        let info = LotteryInfo {
            token_identifier,
            ticket_price,
            tickets_left: total_tickets,
            deadline,
            max_entries_per_user,
            prize_distribution,
            whitelist,
            prize_pool: BigUint::zero(),
        };

        self.lottery_info(&lottery_name).set(&info);

        Ok(())
    }

    #[endpoint]
    #[payable("*")]
    fn buy_ticket(
        &self,
        lottery_name: BoxedBytes,
        #[payment_token] token_identifier: TokenIdentifier,
        #[payment] payment: BigUint,
    ) -> SCResult<()> {
        match self.status(&lottery_name) {
            Status::Inactive => sc_error!("Lottery is currently inactive."),
            Status::Running => {
                self.update_after_buy_ticket(&lottery_name, &token_identifier, &payment)
            },
            Status::Ended => {
                sc_error!("Lottery entry period has ended! Awaiting winner announcement.")
            },
        }
    }

    #[endpoint]
    fn determine_winner(&self, lottery_name: BoxedBytes) -> SCResult<()> {
        match self.status(&lottery_name) {
            Status::Inactive => sc_error!("Lottery is inactive!"),
            Status::Running => sc_error!("Lottery is still running!"),
            Status::Ended => {
                self.distribute_prizes(&lottery_name);
                self.clear_storage(&lottery_name);
                Ok(())
            },
        }
    }

    #[view]
    fn status(&self, lottery_name: &BoxedBytes) -> Status {
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
        lottery_name: &BoxedBytes,
        token_identifier: &TokenIdentifier,
        payment: &BigUint,
    ) -> SCResult<()> {
        let mut info = self.lottery_info(lottery_name).get();
        let caller = self.blockchain().get_caller();

        require!(
            info.whitelist.is_empty() || info.whitelist.contains(&caller),
            "You are not allowed to participate in this lottery!"
        );
        require!(
            token_identifier == &info.token_identifier && payment == &info.ticket_price,
            "Wrong ticket fee!"
        );

        let mut entries = self.number_of_entries_for_user(lottery_name, &caller).get();
        require!(
            entries < info.max_entries_per_user,
            "Ticket limit exceeded for this lottery!"
        );

        self.ticket_holders(lottery_name).push(&caller);

        entries += 1;
        info.tickets_left -= 1;
        info.prize_pool += &info.ticket_price;

        self.number_of_entries_for_user(lottery_name, &caller)
            .set(&entries);
        self.lottery_info(lottery_name).set(&info);

        Ok(())
    }

    fn distribute_prizes(&self, lottery_name: &BoxedBytes) {
        let mut info = self.lottery_info(lottery_name).get();
        let total_tickets = self.ticket_holders(lottery_name).len();

        if total_tickets == 0 {
            return;
        }

        let burn_percentage = self.burn_percentage_for_lottery(lottery_name).get();
        if burn_percentage > 0 {
            let burn_amount = self.calculate_percentage_of(&info.prize_pool, &burn_percentage);

            // Prevent crashing if the role was unset while the lottery was running
            // The tokens will simply remain locked forever
            let roles = self
                .blockchain()
                .get_esdt_local_roles(&info.token_identifier);
            if roles.has_role(&EsdtLocalRole::Burn) {
                self.send()
                    .esdt_local_burn(&info.token_identifier, 0, &burn_amount);
            }

            info.prize_pool -= burn_amount;
        }

        // if there are less tickets than the distributed prize pool,
        // the 1st place gets the leftover, maybe could split between the remaining
        // but this is a rare case anyway and it's not worth the overhead
        let total_winning_tickets = if total_tickets < info.prize_distribution.len() {
            total_tickets as usize
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
            let winner_address = self.ticket_holders(lottery_name).get(winning_ticket_id);
            let prize = self
                .calculate_percentage_of(&total_prize, &BigUint::from(info.prize_distribution[i]));

            self.send().direct(
                &winner_address,
                &info.token_identifier,
                0,
                &prize,
                b"You won the lottery! Congratulations!",
            );
            info.prize_pool -= prize;
        }

        // send leftover to first place
        let first_place_winner = self.ticket_holders(lottery_name).get(winning_tickets[0]);
        self.send().direct(
            &first_place_winner,
            &info.token_identifier,
            0,
            &info.prize_pool,
            b"You won the lottery, 1st place! Congratulations!",
        );
    }

    fn clear_storage(&self, lottery_name: &BoxedBytes) {
        let current_ticket_number = self.ticket_holders(lottery_name).len();

        for i in 1..=current_ticket_number {
            let addr = self.ticket_holders(lottery_name).get(i);
            self.number_of_entries_for_user(lottery_name, &addr).clear();
        }

        self.ticket_holders(lottery_name).clear();
        self.lottery_info(lottery_name).clear();
        self.burn_percentage_for_lottery(lottery_name).clear();
    }

    fn sum_array(&self, array: &[u8]) -> u32 {
        let mut sum = 0;

        for &item in array {
            sum += item as u32;
        }

        sum
    }

    /// does not check if max - min >= amount, that is the caller's job
    fn get_distinct_random(&self, min: usize, max: usize, amount: usize) -> Vec<usize> {
        let mut rand_numbers: Vec<usize> = (min..=max).collect();
        let total_numbers = rand_numbers.len();
        let seed = self.blockchain().get_block_random_seed_legacy();
        let mut rand = Random::new(*seed);

        for i in 0..amount {
            let rand_index = (rand.next() as usize) % total_numbers;
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
    fn lottery_info(&self, lottery_name: &BoxedBytes) -> SingleValueMapper<LotteryInfo<Self::Api>>;

    #[storage_mapper("ticketHolder")]
    fn ticket_holders(&self, lottery_name: &BoxedBytes) -> VecMapper<ManagedAddress>;

    #[storage_mapper("numberOfEntriesForUser")]
    fn number_of_entries_for_user(
        &self,
        lottery_name: &BoxedBytes,
        user: &ManagedAddress,
    ) -> SingleValueMapper<u32>;

    #[storage_mapper("burnPercentageForLottery")]
    fn burn_percentage_for_lottery(&self, lottery_name: &BoxedBytes) -> SingleValueMapper<BigUint>;
}
