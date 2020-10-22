#![no_std]

imports!();

#[elrond_wasm_derive::contract(LotteryImpl)]
pub trait Lottery {
    
    #[init]
    fn init(&self) {
        
    }

    // TODO: ticket limit/user, whitelist, prize distribution

    #[endpoint]
    fn start(&self,
        lottery_name: &Vec<u8>,
        ticket_price: BigUint, 
        total_tickets: Option<u32>, 
        deadline: Option<u64>) 
        -> SCResult<()> {
           
        let tt = total_tickets.unwrap_or(u32::MAX);
        let d = deadline.unwrap_or(i64::MAX as u64);

        if self.status(lottery_name.clone()) != Status::Inactive {
            return sc_error!("Lottery is already active!");
        }
        if ticket_price == 0 {
            return sc_error!("Ticket price must be higher than 0!");
        }
        if tt == 0 {
            return sc_error!("Must have more than 0 tickets available!");
        }
        if d <= self.get_block_nonce() {
            return sc_error!("Deadline can't be in the past!");
        }
    
        self.set_ticket_price(lottery_name, ticket_price);
        self.set_tickets_left(lottery_name, tt);
        self.set_deadline(lottery_name, d);
    
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
                let ticket_price = self.get_ticket_price(&lottery_name);

                if _payment != ticket_price {
                    return sc_error!("Wrong ticket fee!");
                }

                let mut ticket_number = self.get_mut_current_ticket_number(&lottery_name);
                let mut tickets_left = self.get_mut_tickets_left(&lottery_name);
                let mut prize_pool = self.get_mut_prize_pool(&lottery_name);

                self.set_ticket_holder(&lottery_name, *ticket_number, &self.get_caller());
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
                    let winning_ticket_id = self.random() % *total_tickets;
                    let winner_address = self.get_ticket_holder(&lottery_name, winning_ticket_id);

                    self.send_tx(&winner_address, &self.get_sc_balance(), "You won the lottery! Congratulations!");
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
        if self.get_block_nonce() > self.get_deadline(&lottery_name) || 
            *self.get_mut_tickets_left(&lottery_name) == 0 {
            return Status::Ended;
        }

        return Status::Running;
    }

    fn random(&self) -> u32 {
        let current_timestamp = self.get_block_timestamp();
        let hash_array = self.sha256(&current_timestamp.to_be_bytes());
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

        let last_ticket = self.get_mut_current_ticket_number(lottery_name);

        for i in 0..*last_ticket {
            let key = ["ticketHolder".as_bytes(),
                appended_name_in_key, &i.to_be_bytes()].concat();

            self.storage_store(&key, &[0u8; 0]);
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

    #[view]
    #[storage_get_mut("currentTicketNumber")]
    fn get_mut_current_ticket_number(&self, lottery_name: &Vec<u8>) -> mut_storage!(u32);

    #[storage_set("ticketHolder")]
    fn set_ticket_holder(&self, lottery_name: &[u8], ticket_id: u32, ticket_holder: &Address);

    #[storage_get("ticketHolder")]
    fn get_ticket_holder(&self, lottery_name: &Vec<u8>, ticket_id: u32) -> Address;

    #[view]
    #[storage_get_mut("prizePool")]
    fn get_mut_prize_pool(&self, lottery_name: &Vec<u8>) -> mut_storage!(BigUint);
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
