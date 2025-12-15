#![no_std]
#![allow(unused_attributes)]

use multiversx_sc::imports::*;

mod constants;
mod deposit_info;
pub mod digital_cash_proxy;
mod helpers;
mod pay_fee_and_fund;
mod signature_operations;
mod storage;

use constants::*;

#[multiversx_sc::contract]
pub trait DigitalCash:
    pay_fee_and_fund::PayFeeAndFund
    + signature_operations::SignatureOperationsModule
    + helpers::HelpersModule
    + storage::StorageModule
{
    #[init]
    fn init(&self, fee: NonZeroBigUint, token: TokenId) {
        self.whitelist_fee_token(fee, token);
    }

    #[endpoint(whitelistFeeToken)]
    #[only_owner]
    fn whitelist_fee_token(&self, fee: NonZeroBigUint, token: TokenId) {
        require!(self.fee(&token).is_empty(), "Token already whitelisted");
        self.fee(&token).set(fee);
        self.whitelisted_fee_tokens().insert(token.clone());
        self.all_time_fee_tokens().insert(token);
    }

    #[endpoint(blacklistFeeToken)]
    #[only_owner]
    fn blacklist_fee_token(&self, token: TokenId) {
        require!(!self.fee(&token).is_empty(), "Token is not whitelisted");
        self.fee(&token).clear();
        self.whitelisted_fee_tokens().swap_remove(&token);
    }

    #[endpoint(claimFees)]
    #[only_owner]
    fn claim_fees(&self) {
        let fee_tokens_mapper = self.all_time_fee_tokens();
        let fee_tokens = fee_tokens_mapper.iter();
        let caller_address = self.blockchain().get_caller();
        let mut collected_esdt_fees = ManagedVec::new();
        for token in fee_tokens {
            let fee = self.collected_fees(&token).take();
            if fee == 0 {
                continue;
            }
            let collected_fee = Payment::new(token, 0, fee);

            collected_esdt_fees.push(collected_fee);
        }
        if !collected_esdt_fees.is_empty() {
            self.tx()
                .to(&caller_address)
                .payment(&collected_esdt_fees)
                .transfer();
        }
    }

    #[view(getAmount)]
    fn get_amount(&self, address: ManagedAddress, token: TokenId, nonce: u64) -> BigUint {
        let deposit_mapper = self.deposit(&address);
        require!(!deposit_mapper.is_empty(), NON_EXISTENT_KEY_ERR_MSG);

        let deposit = deposit_mapper.get();

        for fund in deposit.funds.into_iter() {
            if fund.token_identifier == token && fund.token_nonce == nonce {
                return fund.amount.as_big_uint().clone();
            }
        }

        BigUint::zero()
    }
}
