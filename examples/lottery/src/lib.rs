#![no_std]

imports!();

const INVALID_TOTAL_TICKETS: i32 = -1;
const INVALID_DEADLINE: u64 = 0;

#[elrond_wasm_derive::contract(LotteryImpl)]
pub trait Lottery {
    #[init]
    fn init(&self) {
        
    }

    fn start(&self, 
        ticket_price: BigUint, 
        total_tickets: i32, 
        deadline: u64) 
        -> SCResult<()> {

        if self.get_caller() != self.get_owner_address() {
            return sc_error!("Only owner may start the lottery!");
        }
        if self.status() != Status::Inactive {
            return sc_error!("Lottery is already active!");
        }
        if ticket_price == 0 {
            return sc_error!("Ticket price must be higher than 0!");
        }

        self.set_ticket_price(ticket_price);
        self.set_tickets_left(total_tickets);
        self.set_deadline(deadline);

        Ok(())
    }

    #[endpoint]
    fn start_limited_tickets_and_fixed_deadline(&self,
        ticket_price: BigUint, 
        total_tickets: i32, 
        deadline: u64) 
        -> SCResult<()> {

        if total_tickets <= 0 {
            return sc_error!("Must have more than 0 tickets available!");
        }
        if deadline <= self.get_block_nonce() {
            return sc_error!("Deadline can't be in the past!");
        }

        self.start(ticket_price, total_tickets, deadline)
    }

    #[endpoint]
    fn start_limited_tickets(&self, 
        ticket_price: BigUint, 
        total_tickets: i32) 
        -> SCResult<()> {
        
        if total_tickets == 0 {
            return sc_error!("Must have more than 0 tickets available!");
        }

        self.start(ticket_price, total_tickets, INVALID_DEADLINE)
    }

    #[endpoint]
    fn start_fixed_deadline(&self,
        ticket_price: BigUint, 
        deadline: u64) 
        -> SCResult<()> {

        if deadline <= self.get_block_nonce() {
            return sc_error!("Deadline can't be in the past!");
        }

        self.start(ticket_price, INVALID_TOTAL_TICKETS, deadline)
    }

    #[endpoint]
    #[payable]
    fn buy_ticket(&self, #[payment] _payment : BigUint) -> SCResult<()> {
        match self.status() {
            Status::Inactive => {
                sc_error!("Lottery is currently inactive.")
            },
            Status::Running => {
                let ticket_price = self.get_ticket_price();

                if _payment != ticket_price {
                    return sc_error!("Wrong ticket fee!");
                }

                let mut ticket_number = self.get_mut_current_ticket_number();
                let mut tickets_left = self.get_mut_tickets_left();

                if *tickets_left != INVALID_TOTAL_TICKETS {
                    *tickets_left -= 1;
                }

                self.set_ticket_holder(*ticket_number, &self.get_caller());
                *ticket_number += 1;

                Ok(())
            },
            Status::Ended => {
                sc_error!("Lottery entry period has ended! Awaiting winner announcement.")
            }
        }
    }

    #[endpoint]
    fn determine_winner(&self) -> SCResult<()> {
        if self.get_owner_address() != self.get_caller() {
            return sc_error!("Only owner may call this function!");
        }

        let winning_ticket_id = self.random();
        let winner_address = self.get_ticket_holder(winning_ticket_id);

        self.send_tx(&winner_address, &self.get_sc_balance(), "You won the lottery! Congratulations!");
        self.clear_storage();

        Ok(())
    }

    #[view]
    fn status(&self) -> Status {
        if self.get_ticket_price() == 0 {
            return Status::Inactive;
        }
        if self.deadline_passed() || self.tickets_sold_out() {
            return Status::Ended;
        }

        return Status::Running;
    }

    fn deadline_passed(&self) -> bool {
        let deadline = self.get_deadline();

        return deadline != INVALID_DEADLINE && deadline > self.get_block_nonce();
    }

    fn tickets_sold_out(&self) -> bool {
        let tickets_left = self.get_mut_tickets_left();

        return *tickets_left != INVALID_TOTAL_TICKETS && *tickets_left == 0;
    }

    fn random(&self) -> u32 {
        let hash = self.get_tx_hash();
        let hash_array = hash.as_bytes();
        let first_byte = (hash_array[28] as u32) << 24;
        let second_byte = (hash_array[29] as u32) << 16;
        let third_byte = (hash_array[30] as u32) << 8;
        let fourth_byte = hash_array[31] as u32;

        return first_byte | second_byte | third_byte | fourth_byte;
    }

    fn clear_storage(&self) {
        self.set_deadline(0);
        self.set_ticket_price(BigUint::zero());
        self.set_tickets_left(0);

        let mut last_ticket = self.get_mut_current_ticket_number();

        for i in 0..*last_ticket {
            self.set_ticket_holder(i, &Address::zero());
        }

        *last_ticket = 0;
    }

    #[storage_set("deadline")]
    fn set_deadline(&self, deadline: u64);

    #[view]
    #[storage_get("deadline")]
    fn get_deadline(&self) -> u64;

    #[storage_set("ticketPrice")]
    fn set_ticket_price(&self, price: BigUint);

    #[view]
    #[storage_get("ticketPrice")]
    fn get_ticket_price(&self) -> BigUint;

    #[view]
    #[storage_get_mut("ticketsLeft")]
    fn get_mut_tickets_left(&self) -> mut_storage!(i32);

    #[storage_set("ticketsLeft")]
    fn set_tickets_left(&self, tickets: i32);

    #[view]
    #[storage_get_mut("currentTicketNumber")]
    fn get_mut_current_ticket_number(&self) -> mut_storage!(u32);

    #[storage_set("ticketHolder")]
    fn set_ticket_holder(&self, ticket_id: u32, ticket_holder: &Address);

    #[storage_get("ticketHolder")]
    fn get_ticket_holder(&self, ticket_id: u32) -> Address;
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
