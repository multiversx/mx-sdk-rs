#![no_std]

imports!();

const PERCENTAGE_TOTAL: u16 = 100;
const THIRTY_DAYS_IN_SECONDS: u64 = 60 * 60 * 24 * 30;

#[elrond_wasm_derive::contract(LotteryImpl)]
pub trait Lottery {
    
    #[init]
    fn init(&self) {
        
    }

    #[endpoint]
    fn start(&self,
        lottery_name: Vec<u8>,
        ticket_price: BigUint, 
        total_tickets: Option<u32>, 
        deadline: Option<u64>,
        max_entries_per_user: Option<u32>,
        prize_distribution: Option<Vec<u8>>,
        whitelist: Option<Vec<Address>>) 
        -> SCResult<()> {
        
        let timestamp = self.get_block_timestamp();
        
        let tt = total_tickets.unwrap_or(u32::MAX);
        let d = deadline.unwrap_or(timestamp + THIRTY_DAYS_IN_SECONDS);
        let max = max_entries_per_user.unwrap_or(u32::MAX);
        let pd = prize_distribution.unwrap_or([PERCENTAGE_TOTAL as u8].to_vec());
        let wl = whitelist.unwrap_or(Vec::new());

        if self.status(lottery_name.clone()) != Status::Inactive {
            return sc_error!("Lottery is already active!");
        }
        if ticket_price == 0 {
            return sc_error!("Ticket price must be higher than 0!");
        }
        if tt == 0 {
            return sc_error!("Must have more than 0 tickets available!");
        }
        if d <= timestamp {
            return sc_error!("Deadline can't be in the past!");
        }
        if d > timestamp + THIRTY_DAYS_IN_SECONDS {
            return sc_error!("Deadline can't be later than 30 days from now!");
        }
        if max == 0 {
            return sc_error!("Must have more than 0 max entries per user!");
        }
        {
            let mut sum = 0u16; // u16 to protect against overflow

            for i in 0..pd.len() {
                sum += pd[i] as u16;

                if sum > PERCENTAGE_TOTAL {
                    return sc_error!("Prize distribution must add up to exactly 100(%)!");
                }
            }
            
            if sum != PERCENTAGE_TOTAL {
                return sc_error!("Prize distribution must add up to exactly 100(%)!");
            }
        }
    
        self.set_ticket_price(&lottery_name, ticket_price);
        self.set_tickets_left(&lottery_name, tt);
        self.set_deadline(&lottery_name, d);
        self.set_max_entries_per_user(&lottery_name, max);
        self.set_prize_distribution(&lottery_name, &pd);
        self.set_whitelist(&lottery_name, &wl);
    
        Ok(())
    }

    #[endpoint]
    #[payable]
    fn buy_ticket(&self, lottery_name: Vec<u8>, #[payment] _payment: BigUint) -> SCResult<()> {
        match self.status(lottery_name.clone()) {
            Status::Inactive => {
                sc_error!("Lottery is currently inactive.")
            },
            Status::Running => {
                let whitelist = self.get_whitelist(&lottery_name);
                let caller = self.get_caller();

                if !whitelist.is_empty() && !whitelist.contains(&caller) {
                    return sc_error!("You are not allowed to participate in this lottery!");
                }

                let ticket_price = self.get_ticket_price(&lottery_name);

                if _payment != ticket_price {
                    return sc_error!("Wrong ticket fee!");
                }

                let mut entries = self.get_number_of_entries_for_user(&lottery_name, &caller);
                let max_entries = self.get_max_entries_per_user(&lottery_name);

                if *entries == max_entries {
                    return sc_error!("Ticket limit exceeded for this lottery!");
                }

                let mut ticket_number = self.get_mut_current_ticket_number(&lottery_name);
                let mut tickets_left = self.get_mut_tickets_left(&lottery_name);
                let mut prize_pool = self.get_mut_prize_pool(&lottery_name);

                self.set_ticket_holder(&lottery_name, *ticket_number, &caller);
                *entries += 1;
                *ticket_number += 1;
                *tickets_left -= 1;
                *prize_pool += ticket_price;

                Ok(())
            },
            Status::Ended => {
                sc_error!("Lottery entry period has ended! Awaiting winner announcement.")
            }
        }
    }

    #[endpoint]
    fn determine_winner(&self, lottery_name: Vec<u8>) -> SCResult<()> {
        match self.status(lottery_name.clone()) {
            Status::Inactive => { 
                sc_error!("Lottery is inactive!")
            },
            Status::Running => {
                sc_error!("Lottery is still running!")
            },
            Status::Ended => {
                let total_tickets = self.get_mut_current_ticket_number(&lottery_name);

                if *total_tickets > 0 {
                    let mut seed = self.get_block_nonce() + self.get_block_timestamp();
                    let mut prev_winning_tickets: Vec<u32> = Vec::new();
                    let mut prize_pool = self.get_mut_prize_pool(&lottery_name);
                    let dist = self.get_prize_distribution(&lottery_name);

                    // if there are less tickets that the distributed prize pool,
                    // the 1st place gets the leftover, maybe could split between the remaining
                    // but this is a rare case anyway and it's not worth the overhead
                    let for_loop_end: usize;

                    if *total_tickets < dist.len() as u32 {
                        for_loop_end = *total_tickets as usize;
                    }
                    else {
                        for_loop_end = dist.len();
                    }
                    
                    // distribute to the first place last. Laws of probability say that order doesn't matter.
                    // this is done to mitigate the effects of BigUint division leading to "spare" prize money being left out at times
                    // 1st place will get the spare money instead.
                    for i in (0..for_loop_end).rev() {
                        let mut winning_ticket_id: u32;

                        loop {
                            // smallest change in input results in entirely different hash, creating "randomness"
                            // +1 just to protect against infinite loop for id = 0
                            winning_ticket_id = self.random(seed) % *total_tickets;
                            seed += (winning_ticket_id as u64) + 1;

                            if !prev_winning_tickets.contains(&winning_ticket_id) {
                                let winner_address = self.get_ticket_holder(&lottery_name, winning_ticket_id);
                                let prize: BigUint;

                                if i != 0 {
                                    prize = BigUint::from(dist[i] as u32) * prize_pool.clone() / BigUint::from(PERCENTAGE_TOTAL as u32);
                                }
                                else {
                                    prize = prize_pool.clone();
                                }

                                self.send_tx(&winner_address, &prize, "You won the lottery! Congratulations!");
                                *prize_pool -= prize;
                                
                                prev_winning_tickets.push(winning_ticket_id);

                                break;
                            }
                        }
                    }
                }

                self.clear_storage(&lottery_name);

                Ok(())
            }
        }
    }

    #[view]
    fn status(&self, lottery_name: Vec<u8>) -> Status {
        // Ticket_price 0 is invalid. Using the fact that memory is initialized to 0 by default.
        if self.get_ticket_price(&lottery_name) == 0 {
            return Status::Inactive;
        }
        if self.get_block_timestamp() > self.get_deadline(&lottery_name) || 
            *self.get_mut_tickets_left(&lottery_name) == 0 {
            return Status::Ended;
        }

        return Status::Running;
    }

    fn random(&self, seed: u64) -> u32 {
        let hash_array = self.sha256(&seed.to_be_bytes());
        let first_byte = (hash_array[28] as u32) << 24;
        let second_byte = (hash_array[29] as u32) << 16;
        let third_byte = (hash_array[30] as u32) << 8;
        let fourth_byte = hash_array[31] as u32;

        return first_byte | second_byte | third_byte | fourth_byte;
    }

    fn clear_storage(&self, lottery_name: &Vec<u8>) {
        let name_len_vec = &(lottery_name.len() as u32).to_be_bytes().to_vec();
        let temp = [&name_len_vec[..], &lottery_name[..]].concat(); // "temporary value dropped" otherwise
        let appended_name_in_key = temp.as_slice();

        self.storage_store(&["deadline".as_bytes(), appended_name_in_key].concat(), &[0u8; 0]);
        self.storage_store(&["ticketPrice".as_bytes(), appended_name_in_key].concat(), &[0u8; 0]);
        self.storage_store(&["ticketsLeft".as_bytes(), appended_name_in_key].concat(), &[0u8; 0]);
        self.storage_store(&["prizePool".as_bytes(), appended_name_in_key].concat(), &[0u8; 0]);
        self.storage_store(&["maxEntriesPerUser".as_bytes(), appended_name_in_key].concat(), &[0u8; 0]);
        self.storage_store(&["prizeDistribution".as_bytes(), appended_name_in_key].concat(), &[0u8; 0]);
        self.storage_store(&["whitelist".as_bytes(), appended_name_in_key].concat(), &[0u8; 0]);

        let last_ticket = self.get_mut_current_ticket_number(lottery_name);

        for i in 0..*last_ticket {
            let addr = self.get_ticket_holder(lottery_name, i);
            let key_ticket_holder = ["ticketHolder".as_bytes(),
                appended_name_in_key, &i.to_be_bytes()].concat();
            let key_number_of_entries = ["numberOfEntriesForUser".as_bytes(),
                appended_name_in_key, addr.as_bytes()].concat();

            self.storage_store(&key_ticket_holder, &[0u8; 0]);
            self.storage_store(&key_number_of_entries, &[0u8; 0]);
        }

        self.storage_store(&["currentTicketNumber".as_bytes(), appended_name_in_key].concat(), &[0u8; 0]);
    }

    #[storage_set("deadline")]
    fn set_deadline(&self, lottery_name: &[u8], deadline: u64);

    #[view]
    #[storage_get("deadline")]
    fn get_deadline(&self, lottery_name: &Vec<u8>) -> u64;

    #[storage_set("ticketPrice")]
    fn set_ticket_price(&self, lottery_name: &[u8], price: BigUint);

    #[view]
    #[storage_get("ticketPrice")]
    fn get_ticket_price(&self, lottery_name: &Vec<u8>) -> BigUint;

    #[view]
    #[storage_get_mut("ticketsLeft")]
    fn get_mut_tickets_left(&self, lottery_name: &Vec<u8>) -> mut_storage!(u32);

    #[storage_set("ticketsLeft")]
    fn set_tickets_left(&self, lottery_name: &[u8], tickets: u32);

    #[storage_get_mut("currentTicketNumber")]
    fn get_mut_current_ticket_number(&self, lottery_name: &Vec<u8>) -> mut_storage!(u32);

    #[storage_set("ticketHolder")]
    fn set_ticket_holder(&self, lottery_name: &[u8], ticket_id: u32, ticket_holder: &Address);

    #[storage_get("ticketHolder")]
    fn get_ticket_holder(&self, lottery_name: &Vec<u8>, ticket_id: u32) -> Address;

    #[view]
    #[storage_get_mut("prizePool")]
    fn get_mut_prize_pool(&self, lottery_name: &Vec<u8>) -> mut_storage!(BigUint);

    #[storage_set("maxEntriesPerUser")]
    fn set_max_entries_per_user(&self, lottery_name: &[u8], max_entries: u32);

    #[view]
    #[storage_get("maxEntriesPerUser")]
    fn get_max_entries_per_user(&self, lottery_name: &Vec<u8>) -> u32;

    #[storage_get_mut("numberOfEntriesForUser")]
    fn get_number_of_entries_for_user(&self, lottery_name: &Vec<u8>, user: &Address) -> mut_storage!(u32);

    #[storage_set("prizeDistribution")]
    fn set_prize_distribution(&self, lottery_name: &[u8], dist: &[u8]);

    #[view]
    #[storage_get("prizeDistribution")]
    fn get_prize_distribution(&self, lottery_name: &Vec<u8>) -> Vec<u8>;

    #[storage_set("whitelist")]
    fn set_whitelist(&self, lottery_name: &[u8], list: &[Address]);

    #[view]
    #[storage_get("whitelist")]
    fn get_whitelist(&self, lottery_name: &Vec<u8>) -> Vec<Address>;
}

use elrond_wasm::elrond_codec::*;

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
    Inactive,
    Running,
    Ended,
}

impl Status {
    pub fn to_u8(&self) -> u8 {
        match self {
            Status::Inactive => 0,
            Status::Running => 1,
            Status::Ended => 2,
        }
    }

    fn from_u8(v: u8) -> Result<Self, DecodeError> {
        match v {
            0 => core::result::Result::Ok(Status::Inactive),
            1 => core::result::Result::Ok(Status::Running),
            2 => core::result::Result::Ok(Status::Ended),
            _ => core::result::Result::Err(DecodeError::InvalidValue),
        }
    }
}

impl Encode for Status {
    fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        return self.to_u8().dep_encode_to(dest);
    }
}

impl Decode for Status {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        return Status::from_u8(u8::dep_decode(input)?);
    }
}
