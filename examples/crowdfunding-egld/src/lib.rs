#![no_std]
#![allow(unused_attributes)]

imports!();

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed
}

#[elrond_wasm_derive::contract(CrowdfundingImpl)]
pub trait Crowdfunding {
    #[storage_set("owner")]
    fn set_owner(&self, address: &Address);

    #[view]
    #[storage_get("owner")]
    fn get_owner(&self) -> Address;

    #[storage_set("target")]
    fn set_target(&self, target: &BigUint);

    #[view]
    #[storage_get("target")]
    fn get_target(&self) -> BigUint;

    #[storage_set("deadline")]
    fn set_deadline(&self, deadline: u64);

    #[view]
    #[storage_get("deadline")]
    fn get_deadline(&self) -> u64;

    #[storage_set("deposit")]
    fn set_deposit(&self, donor: &Address, amount: &BigUint);

    #[view]
    #[storage_get("deposit")]
    fn get_deposit(&self, donor: &Address) -> BigUint;

    #[init]
    fn init(&self, target: &BigUint, deadline: u64) {
        let my_address : Address = self.get_caller();
        self.set_owner(&my_address);
        self.set_target(target);
        self.set_deadline(deadline);
    }

    #[payable]
    #[endpoint]
    fn fund(&self, #[payment] payment: &BigUint) -> SCResult<()> {
        if self.get_block_nonce() > self.get_deadline() {
            return sc_error!("cannot fund after deadline");
        }

        let caller = self.get_caller();
        let mut deposit = self.get_deposit(&caller);
        deposit += payment;
        self.set_deposit(&caller, &deposit);

        return Ok(());
    }

    #[view]    
    fn status(&self) -> Status {
        if self.get_block_nonce() <= self.get_deadline() {
            return Status::FundingPeriod;
        } else if self.get_sc_balance() >= self.get_target() {
            return Status::Successful;
        } else {
            return Status::Failed;
        }
    }

    #[view(currentFunds)]
    fn current_funds(&self) -> SCResult<BigUint> {
        Ok(self.get_sc_balance())
    }

    #[endpoint]
    fn claim(&self) -> SCResult<()> {
        match self.status() {
            Status::FundingPeriod => {
                sc_error!("cannot claim before deadline")
            },
            Status::Successful => {
                let caller = self.get_caller();
                if &caller != &self.get_owner() {
                    return sc_error!("only owner can claim successful funding");
                }
                self.send_tx(&caller, &self.get_sc_balance(), "funding success");
                Ok(())
            },
            Status::Failed => {
                let caller = self.get_caller();
                let deposit = self.get_deposit(&caller);
                if &deposit > &0 {
                    self.send_tx(&caller, &deposit, "reclaim failed funding");
                    self.set_deposit(&caller, &BigUint::zero());
                }
                Ok(())
            },
        }
    }
}

use elrond_wasm::elrond_codec::*;

impl Status {
    pub fn to_u8(&self) -> u8 {
        match self {
            Status::FundingPeriod => 0,
            Status::Successful => 1,
            Status::Failed => 2,
        }
    }

    fn from_u8(v: u8) -> Result<Self, DecodeError> {
        match v {
            0 => core::result::Result::Ok(Status::FundingPeriod),
            1 => core::result::Result::Ok(Status::Successful),
            2 => core::result::Result::Ok(Status::Failed),
            _ => core::result::Result::Err(DecodeError::INVALID_VALUE),
        }
    }
}

impl TopEncode for Status {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.to_u8().top_encode(output)
    }
}

impl TopDecode for Status {
    fn top_decode<I: TopDecodeInput, R, F: FnOnce(Result<Self, DecodeError>) -> R>(input: I, f: F) -> R {
        u8::top_decode(input, |res| match res {
            core::result::Result::Ok(num) => f(Status::from_u8(num)),
            core::result::Result::Err(e) => f(core::result::Result::Err(e)),
        })
    }
}
