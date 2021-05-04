#![no_std]
#![allow(clippy::too_many_arguments)]

elrond_wasm::imports!();

mod lottery_info;
mod random;
mod status;

use lottery_info::LotteryInfo;
use random::Random;
use status::Status;

const PERCENTAGE_TOTAL: u16 = 100;
const THIRTY_DAYS_IN_SECONDS: u64 = 60 * 60 * 24 * 30;
const MAX_TICKETS: u32 = 800;

#[elrond_wasm_derive::contract(LotteryImpl)]
pub trait Lottery {
	#[init]
	fn init(&self) {}

	#[endpoint]
	fn start(
		&self,
		lottery_name: BoxedBytes,
		ticket_price: BigUint,
		opt_total_tickets: Option<u32>,
		opt_deadline: Option<u64>,
		opt_max_entries_per_user: Option<u32>,
		opt_prize_distribution: Option<Vec<u8>>,
		opt_whitelist: Option<Vec<Address>>,
	) -> SCResult<()> {
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
		opt_whitelist: Option<Vec<Address>>,
	) -> SCResult<()> {
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

	fn start_lottery(
		&self,
		lottery_name: BoxedBytes,
		ticket_price: BigUint,
		opt_total_tickets: Option<u32>,
		opt_deadline: Option<u64>,
		opt_max_entries_per_user: Option<u32>,
		opt_prize_distribution: Option<Vec<u8>>,
		opt_whitelist: Option<Vec<Address>>,
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

		let info = LotteryInfo {
			ticket_price,
			tickets_left: total_tickets,
			deadline,
			max_entries_per_user,
			prize_distribution,
			whitelist,
			current_ticket_number: 0u32,
			prize_pool: BigUint::zero(),
		};

		self.lottery_info(&lottery_name).set(&info);

		Ok(())
	}

	#[endpoint]
	#[payable("EGLD")]
	fn buy_ticket(&self, lottery_name: BoxedBytes, #[payment] payment: BigUint) -> SCResult<()> {
		match self.status(&lottery_name) {
			Status::Inactive => sc_error!("Lottery is currently inactive."),
			Status::Running => self.update_after_buy_ticket(&lottery_name, &payment),
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

		let info = self.lottery_info(&lottery_name).get();
		let current_time = self.blockchain().get_block_timestamp();
		if current_time > info.deadline || info.tickets_left == 0 {
			return Status::Ended;
		}

		Status::Running
	}

	fn update_after_buy_ticket(
		&self,
		lottery_name: &BoxedBytes,
		payment: &BigUint,
	) -> SCResult<()> {
		let mut info = self.lottery_info(&lottery_name).get();
		let caller = self.blockchain().get_caller();

		require!(
			info.whitelist.is_empty() || info.whitelist.contains(&caller),
			"You are not allowed to participate in this lottery!"
		);
		require!(payment == &info.ticket_price, "Wrong ticket fee!");

		let mut entries = self
			.number_of_entries_for_user(&lottery_name, &caller)
			.get();
		require!(
			entries < info.max_entries_per_user,
			"Ticket limit exceeded for this lottery!"
		);

		self.ticket_holder(&lottery_name, info.current_ticket_number)
			.set(&caller);
		entries += 1;
		info.current_ticket_number += 1;
		info.tickets_left -= 1;

		let ticket_price = info.ticket_price.clone();
		info.prize_pool += ticket_price;

		self.number_of_entries_for_user(lottery_name, &caller)
			.set(&entries);
		self.lottery_info(lottery_name).set(&info);

		Ok(())
	}

	fn distribute_prizes(&self, lottery_name: &BoxedBytes) {
		let mut info = self.lottery_info(&lottery_name).get();
		let total_tickets = info.current_ticket_number;

		if info.current_ticket_number > 0 {
			let mut prev_winning_tickets: Vec<u32> = Vec::new();

			let seed = self.blockchain().get_block_random_seed();
			let mut rand = Random::new(*seed);

			// if there are less tickets that the distributed prize pool,
			// the 1st place gets the leftover, maybe could split between the remaining
			// but this is a rare case anyway and it's not worth the overhead
			let for_loop_end: usize;

			if total_tickets < info.prize_distribution.len() as u32 {
				for_loop_end = total_tickets as usize;
			} else {
				for_loop_end = info.prize_distribution.len();
			}

			// distribute to the first place last. Laws of probability say that order doesn't matter.
			// this is done to mitigate the effects of BigUint division leading to "spare" prize money being left out at times
			// 1st place will get the spare money instead.
			let total_prize = info.prize_pool.clone();

			for i in (0..for_loop_end).rev() {
				let mut winning_ticket_id: u32;

				loop {
					// smallest change in input results in entirely different hash, creating "randomness"
					// +1 just to protect against infinite loop for id = 0
					winning_ticket_id = rand.next() % total_tickets;

					if !prev_winning_tickets.contains(&winning_ticket_id) {
						let winner_address =
							self.ticket_holder(&lottery_name, winning_ticket_id).get();
						let prize: BigUint;

						if i != 0 {
							prize = &BigUint::from(info.prize_distribution[i] as u32)
								* &total_prize / BigUint::from(PERCENTAGE_TOTAL as u32);
						} else {
							prize = info.prize_pool.clone();
						}

						self.send().direct_egld(
							&winner_address,
							&prize,
							b"You won the lottery! Congratulations!",
						);
						info.prize_pool -= prize;

						prev_winning_tickets.push(winning_ticket_id);

						break;
					}
				}
			}
		}

		self.lottery_info(lottery_name).set(&info);
	}

	fn clear_storage(&self, lottery_name: &BoxedBytes) {
		let info = self.lottery_info(lottery_name).get();

		for i in 0..info.current_ticket_number {
			let addr = self.ticket_holder(lottery_name, i).get();

			self.ticket_holder(lottery_name, i).clear();
			self.number_of_entries_for_user(lottery_name, &addr).clear();
		}

		self.lottery_info(lottery_name).clear();
	}

	fn sum_array(&self, array: &[u8]) -> u16 {
		let mut sum = 0u16; // u16 to protect against overflow

		for &item in array {
			sum += item as u16;
		}

		sum
	}

	// storage

	#[view(getLotteryInfo)]
	#[storage_mapper("lotteryInfo")]
	fn lottery_info(
		&self,
		lottery_name: &BoxedBytes,
	) -> SingleValueMapper<Self::Storage, LotteryInfo<BigUint>>;

	#[storage_mapper("ticketHolder")]
	fn ticket_holder(
		&self,
		lottery_name: &BoxedBytes,
		ticket_id: u32,
	) -> SingleValueMapper<Self::Storage, Address>;

	#[storage_mapper("numberOfEntriesForUser")]
	fn number_of_entries_for_user(
		&self,
		lottery_name: &BoxedBytes,
		user: &Address,
	) -> SingleValueMapper<Self::Storage, u32>;
}
