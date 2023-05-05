#![no_std]

multiversx_sc::imports!();

mod lottery_info;
mod random;
mod status;

use lottery_info::LotteryInfo;
use random::Random;
use status::Status;

const PERCENTAGE_TOTAL: u16 = 100;
const THIRTY_DAYS_IN_SECONDS: u64 = 60 * 60 * 24 * 30;

#[multiversx_sc::contract]
pub trait Lottery {
    #[proxy]
    fn erc20_proxy(&self, to: ManagedAddress) -> erc20::Proxy<Self::Api>;

    #[init]
    fn init(&self, erc20_contract_address: ManagedAddress) {
        self.set_erc20_contract_address(&erc20_contract_address);
    }

    #[endpoint]
    fn start(
        &self,
        lottery_name: BoxedBytes,
        ticket_price: BigUint,
        opt_total_tickets: Option<u32>,
        opt_deadline: Option<u64>,
        opt_max_entries_per_user: Option<u32>,
        opt_prize_distribution: Option<Vec<u8>>,
        opt_whitelist: Option<Vec<ManagedAddress>>,
    ) {
        self.start_lottery(
            lottery_name,
            ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution,
            opt_whitelist,
        )
    }

    #[endpoint(createLotteryPool)]
    fn create_lottery_pool(
        &self,
        lottery_name: BoxedBytes,
        ticket_price: BigUint,
        opt_total_tickets: Option<u32>,
        opt_deadline: Option<u64>,
        opt_max_entries_per_user: Option<u32>,
        opt_prize_distribution: Option<Vec<u8>>,
        opt_whitelist: Option<Vec<ManagedAddress>>,
    ) {
        self.start_lottery(
            lottery_name,
            ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution,
            opt_whitelist,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn start_lottery(
        &self,
        lottery_name: BoxedBytes,
        ticket_price: BigUint,
        opt_total_tickets: Option<u32>,
        opt_deadline: Option<u64>,
        opt_max_entries_per_user: Option<u32>,
        opt_prize_distribution: Option<Vec<u8>>,
        opt_whitelist: Option<Vec<ManagedAddress>>,
    ) {
        require!(!lottery_name.is_empty(), "Name can't be empty!");

        let timestamp = self.blockchain().get_block_timestamp();

        let total_tickets = opt_total_tickets.unwrap_or(u32::MAX);
        let deadline = opt_deadline.unwrap_or(timestamp + THIRTY_DAYS_IN_SECONDS);
        let max_entries_per_user = opt_max_entries_per_user.unwrap_or(u32::MAX);
        let prize_distribution =
            opt_prize_distribution.unwrap_or_else(|| [PERCENTAGE_TOTAL as u8].to_vec());
        let whitelist = opt_whitelist.unwrap_or_default();

        require!(
            self.status(&lottery_name) == Status::Inactive,
            "Lottery is already active!"
        );

        require!(ticket_price > 0, "Ticket price must be higher than 0!");

        require!(
            total_tickets > 0,
            "Must have more than 0 tickets available!"
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

        let info = LotteryInfo {
            ticket_price,
            tickets_left: total_tickets,
            deadline,
            max_entries_per_user,
            prize_distribution,
            whitelist,
            current_ticket_number: 0u32,
            prize_pool: BigUint::zero(),
            queued_tickets: 0u32,
        };

        self.set_lottery_info(&lottery_name, &info);
    }

    #[endpoint]
    fn buy_ticket(&self, lottery_name: BoxedBytes, token_amount: BigUint) {
        match self.status(&lottery_name) {
            Status::Inactive => sc_panic!("Lottery is currently inactive."),
            Status::Running => self.update_after_buy_ticket(&lottery_name, token_amount),
            Status::Ended => {
                sc_panic!("Lottery entry period has ended! Awaiting winner announcement.")
            },
            Status::DistributingPrizes => {
                sc_panic!("Prizes are currently being distributed. Can't buy tickets!")
            },
        }
    }

    #[endpoint]
    fn determine_winner(&self, lottery_name: BoxedBytes) {
        match self.status(&lottery_name) {
            Status::Inactive => sc_panic!("Lottery is inactive!"),
            Status::Running => sc_panic!("Lottery is still running!"),
            Status::Ended => {
                let info = self.get_lottery_info(&lottery_name);

                if info.queued_tickets > 0 {
                    sc_panic!("There are still tickets being processed. Please try again later.");
                }

                self.distribute_prizes(&lottery_name);
            },
            Status::DistributingPrizes => sc_panic!("Prizes are currently being distributed!"),
        }
    }

    #[view]
    fn status(&self, lottery_name: &BoxedBytes) -> Status {
        if self.is_empty_lottery_info(lottery_name) {
            return Status::Inactive;
        }

        let prev_winners = self.get_prev_winners(lottery_name);

        if !prev_winners.is_empty() {
            return Status::DistributingPrizes;
        }

        let info = self.get_lottery_info(lottery_name);

        if self.blockchain().get_block_timestamp() > info.deadline || info.tickets_left == 0 {
            return Status::Ended;
        }

        Status::Running
    }

    fn update_after_buy_ticket(&self, lottery_name: &BoxedBytes, token_amount: BigUint) {
        let info = self.get_lottery_info(lottery_name);
        let caller = self.blockchain().get_caller();

        require!(
            info.whitelist.is_empty() || info.whitelist.contains(&caller),
            "You are not allowed to participate in this lottery!"
        );

        require!(token_amount == info.ticket_price, "Wrong ticket fee!");

        let entries = self.get_number_of_entries_for_user(lottery_name, &caller);

        require!(
            entries < info.max_entries_per_user,
            "Ticket limit exceeded for this lottery!"
        );

        // reserve the ticket, but don't update the other fields yet.
        self.reserve_ticket(lottery_name);

        let erc20_address = self.get_erc20_contract_address();
        let lottery_contract_address = self.blockchain().get_sc_address();
        self.erc20_proxy(erc20_address)
            .transfer_from(caller.clone(), lottery_contract_address, token_amount)
            .async_call()
            .with_callback(
                self.callbacks()
                    .transfer_from_callback(lottery_name, &caller),
            )
            .call_and_exit()
    }

    fn reserve_ticket(&self, lottery_name: &BoxedBytes) {
        let mut info = self.get_lottery_info(lottery_name);

        info.tickets_left -= 1;
        info.queued_tickets += 1;

        self.set_lottery_info(lottery_name, &info);
    }

    fn reduce_prize_pool(&self, lottery_name: &BoxedBytes, value: BigUint) {
        let mut info = self.get_lottery_info(lottery_name);
        info.prize_pool -= value;

        self.set_lottery_info(lottery_name, &info);
    }

    fn distribute_prizes(&self, lottery_name: &BoxedBytes) {
        let info = self.get_lottery_info(lottery_name);

        let total_tickets = info.current_ticket_number;
        let total_winning_tickets = info.prize_distribution.len();
        let mut prev_winners = self.get_prev_winners(lottery_name);
        let prev_winners_count = prev_winners.len();
        let winners_left = total_winning_tickets - prev_winners_count;

        if winners_left == 0 {
            self.clear_storage(lottery_name);

            return;
        }

        // less tickets purchased than total winning tickets
        let last_winning_ticket_index = if total_tickets < total_winning_tickets as u32 {
            (total_tickets - 1) as usize
        } else {
            info.prize_distribution.len() - 1
        };

        let current_winning_ticket_index = last_winning_ticket_index - prev_winners_count;
        let winning_ticket_id = self.get_random_winning_ticket_id(&prev_winners, total_tickets);

        let winner_address = self.get_ticket_holder(lottery_name, winning_ticket_id);

        let prize = if current_winning_ticket_index != 0 {
            BigUint::from(info.prize_distribution[current_winning_ticket_index] as u32)
                * &info.prize_pool
                / PERCENTAGE_TOTAL as u32
        } else {
            info.prize_pool.clone()
        };

        self.reduce_prize_pool(lottery_name, prize.clone());

        prev_winners.push(winning_ticket_id);
        self.set_prev_winners(lottery_name, &prev_winners);

        self.set_lottery_info(lottery_name, &info);

        let erc20_address = self.get_erc20_contract_address();

        self.erc20_proxy(erc20_address)
            .transfer(winner_address, prize)
            .async_call()
            .with_callback(self.callbacks().distribute_prizes_callback(lottery_name))
            .call_and_exit()
    }

    fn get_random_winning_ticket_id(&self, prev_winners: &[u32], total_tickets: u32) -> u32 {
        #[allow(deprecated)]
        let seed = self.blockchain().get_block_random_seed_legacy();
        let mut rand = Random::new(*seed);

        loop {
            let winner = rand.next() % total_tickets;

            if !prev_winners.contains(&winner) {
                return winner;
            }
        }
    }

    fn clear_storage(&self, lottery_name: &BoxedBytes) {
        let info = self.get_lottery_info(lottery_name);

        for i in 0..info.current_ticket_number {
            let addr = self.get_ticket_holder(lottery_name, i);

            self.clear_ticket_holder(lottery_name, i);
            self.clear_number_of_entries_for_user(lottery_name, &addr);
        }

        self.clear_previous_winners(lottery_name);
        self.clear_lottery_info(lottery_name);
    }

    fn sum_array(&self, array: &[u8]) -> u16 {
        let mut sum = 0u16; // u16 to protect against overflow

        for &item in array {
            sum += item as u16;
        }

        sum
    }

    #[callback]
    fn transfer_from_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        cb_lottery_name: &BoxedBytes,
        cb_sender: &ManagedAddress,
    ) {
        let mut info = self.get_lottery_info(cb_lottery_name);

        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let mut entries = self.get_number_of_entries_for_user(cb_lottery_name, cb_sender);

                self.set_ticket_holder(cb_lottery_name, info.current_ticket_number, cb_sender);

                entries += 1;
                info.current_ticket_number += 1;

                let ticket_price = info.ticket_price.clone();
                info.prize_pool += ticket_price;

                self.set_number_of_entries_for_user(cb_lottery_name, cb_sender, entries);
            },
            ManagedAsyncCallResult::Err(_) => {
                // payment error, return ticket to pool
                info.tickets_left += 1;
            },
        }

        info.queued_tickets -= 1;

        self.set_lottery_info(cb_lottery_name, &info);
    }

    #[callback]
    fn distribute_prizes_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        cb_lottery_name: &BoxedBytes,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => self.distribute_prizes(cb_lottery_name),
            ManagedAsyncCallResult::Err(_) => {
                // nothing we can do if an error occurs in the erc20 contract
            },
        }
    }

    // storage

    #[storage_set("lotteryInfo")]
    fn set_lottery_info(&self, lottery_name: &BoxedBytes, lottery_info: &LotteryInfo<Self::Api>);

    #[view(lotteryInfo)]
    #[storage_get("lotteryInfo")]
    fn get_lottery_info(&self, lottery_name: &BoxedBytes) -> LotteryInfo<Self::Api>;

    #[storage_is_empty("lotteryInfo")]
    fn is_empty_lottery_info(&self, lottery_name: &BoxedBytes) -> bool;

    #[storage_clear("lotteryInfo")]
    fn clear_lottery_info(&self, lottery_name: &BoxedBytes);

    #[storage_set("ticketHolder")]
    fn set_ticket_holder(
        &self,
        lottery_name: &BoxedBytes,
        ticket_id: u32,
        ticket_holder: &ManagedAddress,
    );

    #[storage_get("ticketHolder")]
    fn get_ticket_holder(&self, lottery_name: &BoxedBytes, ticket_id: u32) -> ManagedAddress;

    #[storage_clear("ticketHolder")]
    fn clear_ticket_holder(&self, lottery_name: &BoxedBytes, ticket_id: u32);

    #[storage_set("numberOfEntriesForUser")]
    fn set_number_of_entries_for_user(
        &self,
        lottery_name: &BoxedBytes,
        user: &ManagedAddress,
        nr_entries: u32,
    );

    #[storage_get("numberOfEntriesForUser")]
    fn get_number_of_entries_for_user(
        &self,
        lottery_name: &BoxedBytes,
        user: &ManagedAddress,
    ) -> u32;

    #[storage_clear("numberOfEntriesForUser")]
    fn clear_number_of_entries_for_user(&self, lottery_name: &BoxedBytes, user: &ManagedAddress);

    #[storage_set("erc20ContractAddress")]
    fn set_erc20_contract_address(&self, address: &ManagedAddress);

    #[view(erc20ContractManagedAddress)]
    #[storage_get("erc20ContractAddress")]
    fn get_erc20_contract_address(&self) -> ManagedAddress;

    // temporary storage between "determine_winner" proxy callbacks

    #[storage_get("previousWinners")]
    fn get_prev_winners(&self, lottery_name: &BoxedBytes) -> Vec<u32>;

    #[storage_set("previousWinners")]
    fn set_prev_winners(&self, lottery_name: &BoxedBytes, prev_winners: &[u32]);

    #[storage_clear("previousWinners")]
    fn clear_previous_winners(&self, lottery_name: &BoxedBytes);
}
