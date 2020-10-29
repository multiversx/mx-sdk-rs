
#![no_std]
#![allow(unused_attributes)]
#![allow(unused_variables)]

imports!();

#[elrond_wasm_derive::callable(Erc20Proxy)]
pub trait Erc20 {
    #[callback(transfer_from_callback)]
    fn transferFrom(&self,
        sender: &Address,
        recipient: &Address,
        amount: BigUint,
        #[callback_arg] cb_sender: &Address,
        #[callback_arg] cb_amount: BigUint);

    fn transfer(&self, to: &Address, amount: BigUint);
}

#[elrond_wasm_derive::contract(CrowdfundingImpl)]
pub trait Crowdfunding {

    #[init]
    fn init(&self, target: &BigUint, deadline: u64, erc20_contract_address: Address) {
        let my_address : Address = self.get_caller();

        self.set_owner(&my_address);
        self.set_erc20_contract_address(&erc20_contract_address);
        self.set_target(target);
        self.set_deadline(deadline);
    }

    #[endpoint]
    fn fund(&self, token_amount: BigUint) -> SCResult<()> {
        if self.get_block_nonce() > self.get_deadline() {
            return sc_error!("cannot fund after deadline");
        }

        let caller = self.get_caller();
        let erc20_address = self.get_erc20_contract_address();
        let cf_contract_address = self.get_sc_address();

        let erc20_proxy = contract_proxy!(self, &erc20_address, Erc20);
        erc20_proxy.transferFrom(&caller, &cf_contract_address,
            token_amount.clone(),
            &caller, token_amount.clone());

        Ok(())
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

                let erc20_address = self.get_erc20_contract_address();
                let erc20_proxy = contract_proxy!(self, &erc20_address, Erc20);
                let mut balance = self.get_mut_total_balance();

                erc20_proxy.transfer( &caller, balance.clone());

                *balance = BigUint::zero();

                Ok(())
            },
            Status::Failed => {
                let caller = self.get_caller();
                let deposit = self.get_deposit(&caller);

                if &deposit > &0 {
                    let erc20_address = self.get_erc20_contract_address();
                    let erc20_proxy = contract_proxy!(self, &erc20_address, Erc20);
                    
                    erc20_proxy.transfer( &caller, deposit);
                    self.set_deposit(&caller, &BigUint::zero());
                }

                Ok(())
            },
        }
    }

    #[callback]
    fn transfer_from_callback(&self, 
        result: AsyncCallResult<()>,
        #[callback_arg] cb_sender: Address,
        #[callback_arg] cb_amount: BigUint) {

        // transaction started before deadline, ended after -> refund
        if self.get_block_nonce() > self.get_deadline() {
            let erc20_address = self.get_erc20_contract_address();
            let erc20_proxy = contract_proxy!(self, &erc20_address, Erc20);
            
            erc20_proxy.transfer( &cb_sender, cb_amount);

            return;
        }

        match result {
            AsyncCallResult::Ok(()) => {
                let mut deposit = self.get_deposit(&cb_sender);
                let mut balance = self.get_mut_total_balance();
                deposit += &cb_amount;
                *balance += &cb_amount;

                self.set_deposit(&cb_sender, &deposit);
            },
            AsyncCallResult::Err(_) => {}
        }
    }

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

    #[storage_set("erc20_contract_address")]
    fn set_erc20_contract_address(&self, address: &Address);

    #[view]
    #[storage_get("erc20_contract_address")]
    fn get_erc20_contract_address(&self) -> Address;

    #[view]
    #[storage_get_mut("erc20_balance")]
    fn get_mut_total_balance(&self) -> mut_storage!(BigUint);
}

use elrond_wasm::elrond_codec::*;

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed
}

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
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        Status::from_u8(u8::top_decode(input)?)
    }
}
