#![no_std]

imports!();

mod lottery_info;
mod random;
mod status;

use lottery_info::LotteryInfo;
use random::Random;
use status::Status;

use elrond_wasm::CallDataSerializer;

const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";

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
		lottery_name: Vec<u8>,
		esdt_token_name: Vec<u8>,
		ticket_price: BigUint,
		opt_total_tickets: Option<u32>,
		opt_deadline: Option<u64>,
		opt_max_entries_per_user: Option<u32>,
		opt_prize_distribution: Option<Vec<u8>>,
		opt_whitelist: Option<Vec<Address>>,
	) -> SCResult<()> {
		self.start_lottery(
			lottery_name,
			esdt_token_name,
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
		lottery_name: Vec<u8>,
		esdt_token_name: Vec<u8>,
		ticket_price: BigUint,
		opt_total_tickets: Option<u32>,
		opt_deadline: Option<u64>,
		opt_max_entries_per_user: Option<u32>,
		opt_prize_distribution: Option<Vec<u8>>,
		opt_whitelist: Option<Vec<Address>>,
	) -> SCResult<()> {
		self.start_lottery(
			lottery_name,
			esdt_token_name,
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
		lottery_name: Vec<u8>,
		esdt_token_name: Vec<u8>,
		ticket_price: BigUint,
		opt_total_tickets: Option<u32>,
		opt_deadline: Option<u64>,
		opt_max_entries_per_user: Option<u32>,
		// prize distribution not supported in the current version
		// multiple async calls not supported
		// it will always be set to default 100% to the winner
		_opt_prize_distribution: Option<Vec<u8>>,
		opt_whitelist: Option<Vec<Address>>,
	) -> SCResult<()> {
		require!(!lottery_name.is_empty(), "Name can't be empty!");
		require!(
			!esdt_token_name.is_empty(),
			"Esdt token name can't be empty!"
		);

		let timestamp = self.get_block_timestamp();

		let total_tickets = opt_total_tickets.unwrap_or(MAX_TICKETS);
		let deadline = opt_deadline.unwrap_or_else(|| timestamp + THIRTY_DAYS_IN_SECONDS);
		let max_entries_per_user = opt_max_entries_per_user.unwrap_or(MAX_TICKETS);
		let prize_distribution = [PERCENTAGE_TOTAL as u8].to_vec();
		let whitelist = opt_whitelist.unwrap_or(Vec::new());

		require!(
			self.status(lottery_name.clone()) == Status::Inactive,
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
			esdt_token_name,
			ticket_price,
			tickets_left: total_tickets,
			deadline,
			max_entries_per_user,
			prize_distribution,
			whitelist,
			current_ticket_number: 0u32,
			prize_pool: BigUint::zero(),
		};

		self.set_lottery_exists(&lottery_name, true);
		self.set_lottery_info(&lottery_name, &info);

		Ok(())
	}

	#[endpoint]
	fn buy_ticket(&self, lottery_name: Vec<u8>) -> SCResult<()> {
		match self.status(lottery_name.clone()) {
			Status::Inactive => sc_error!("Lottery is currently inactive."),
			Status::Running => self.update_after_buy_ticket(&lottery_name),
			Status::Ended => {
				sc_error!("Lottery entry period has ended! Awaiting winner announcement.")
			},
		}
	}

	#[endpoint]
	fn determine_winner(&self, lottery_name: Vec<u8>) -> SCResult<()> {
		match self.status(lottery_name.clone()) {
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
	fn status(&self, lottery_name: Vec<u8>) -> Status {
		let exists = self.get_lottery_exists(&lottery_name);

		if !exists {
			return Status::Inactive;
		}

		let info = self.get_mut_lottery_info(&lottery_name);

		if self.get_block_timestamp() > info.deadline || info.tickets_left == 0 {
			return Status::Ended;
		}

		return Status::Running;
	}

	fn update_after_buy_ticket(&self, lottery_name: &Vec<u8>) -> SCResult<()> {
		let mut info = self.get_mut_lottery_info(&lottery_name);
		let caller = self.get_caller();
		let call_token_name = self.get_esdt_token_name();
		let payment = self.get_esdt_value_big_uint();

		require!(
			info.whitelist.is_empty() || info.whitelist.contains(&caller),
			"You are not allowed to participate in this lottery!"
		);

		require!(call_token_name == info.esdt_token_name, "Wrong esdt token!");

		require!(payment == info.ticket_price, "Wrong ticket fee!");

		let mut entries = self.get_mut_number_of_entries_for_user(&lottery_name, &caller);

		require!(
			*entries < info.max_entries_per_user,
			"Ticket limit exceeded for this lottery!"
		);

		self.set_ticket_holder(&lottery_name, info.current_ticket_number, &caller);
		*entries += 1;
		info.current_ticket_number += 1;
		info.tickets_left -= 1;
		info.prize_pool += payment;

		Ok(())
	}

	fn distribute_prizes(&self, lottery_name: &Vec<u8>) {
		let mut info = self.get_mut_lottery_info(&lottery_name);
		let total_tickets = info.current_ticket_number;

		if info.current_ticket_number > 0 {
			let mut prev_winning_tickets: Vec<u32> = Vec::new();

			let seed = self.get_block_random_seed();
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
			for i in (0..for_loop_end).rev() {
				let mut winning_ticket_id: u32;

				loop {
					winning_ticket_id = rand.next() % total_tickets;

					if !prev_winning_tickets.contains(&winning_ticket_id) {
						let winner_address =
							self.get_ticket_holder(&lottery_name, winning_ticket_id);
						let prize: BigUint;

						if i != 0 {
							prize =
								BigUint::from(info.prize_distribution[i] as u32)
									* info.prize_pool.clone() / BigUint::from(PERCENTAGE_TOTAL as u32);
						} else {
							prize = info.prize_pool.clone();
						}

						info.prize_pool -= &prize;
						prev_winning_tickets.push(winning_ticket_id);

						self.pay_esdt(&info.esdt_token_name, &prize, &winner_address);

						break;
					}
				}
			}
		}
	}

	fn clear_storage(&self, lottery_name: &Vec<u8>) {
		let name_len_vec = &(lottery_name.len() as u32).to_be_bytes().to_vec();
		let temp = [&name_len_vec[..], &lottery_name[..]].concat(); // "temporary value dropped" otherwise
		let appended_name_in_key = temp.as_slice();

		let info = self.get_mut_lottery_info(lottery_name);

		for i in 0..info.current_ticket_number {
			let addr = self.get_ticket_holder(lottery_name, i);
			let key_ticket_holder = [
				"ticketHolder".as_bytes(),
				appended_name_in_key,
				&i.to_be_bytes(),
			]
			.concat();
			let key_number_of_entries = [
				"numberOfEntriesForUser".as_bytes(),
				appended_name_in_key,
				addr.as_bytes(),
			]
			.concat();

			self.storage_store_slice_u8(&key_ticket_holder, &[0u8; 0]);
			self.storage_store_slice_u8(&key_number_of_entries, &[0u8; 0]);
		}

		self.storage_store_slice_u8(
			&["lotteryExists".as_bytes(), appended_name_in_key].concat(),
			&[0u8; 0],
		);
		self.storage_store_slice_u8(
			&["lotteryInfo".as_bytes(), appended_name_in_key].concat(),
			&[0u8; 0],
		);
	}

	fn pay_esdt(&self, esdt_token_name: &[u8], amount: &BigUint, to: &Address) {
		let mut serializer = CallDataSerializer::new(ESDT_TRANSFER_STRING);
		serializer.push_argument_bytes(esdt_token_name);
		serializer.push_argument_bytes(amount.to_bytes_be().as_slice());

		self.async_call(&to, &BigUint::zero(), serializer.as_slice());
	}

	fn sum_array(&self, array: &[u8]) -> u16 {
		let mut sum = 0u16; // u16 to protect against overflow

		for i in 0..array.len() {
			sum += array[i] as u16;
		}

		return sum;
	}

	#[storage_set("lotteryExists")]
	fn set_lottery_exists(&self, lottery_name: &[u8], exists: bool);

	#[view(lotteryExists)]
	#[storage_get("lotteryExists")]
	fn get_lottery_exists(&self, lottery_name: &Vec<u8>) -> bool;

	#[storage_set("lotteryInfo")]
	fn set_lottery_info(&self, lottery_name: &[u8], lottery_info: &LotteryInfo<BigUint>);

	#[view(lotteryInfo)]
	#[storage_get_mut("lotteryInfo")]
	fn get_mut_lottery_info(&self, lottery_name: &Vec<u8>) -> mut_storage!(LotteryInfo<BigUint>);

	#[storage_set("ticketHolder")]
	fn set_ticket_holder(&self, lottery_name: &[u8], ticket_id: u32, ticket_holder: &Address);

	#[storage_get("ticketHolder")]
	fn get_ticket_holder(&self, lottery_name: &[u8], ticket_id: u32) -> Address;

	#[storage_get_mut("numberOfEntriesForUser")]
	fn get_mut_number_of_entries_for_user(
		&self,
		lottery_name: &[u8],
		user: &Address,
	) -> mut_storage!(u32);
}
