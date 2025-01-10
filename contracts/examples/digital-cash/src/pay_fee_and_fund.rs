use multiversx_sc::imports::*;

use crate::{constants::*, helpers, storage};

#[multiversx_sc::module]
pub trait PayFeeAndFund: storage::StorageModule + helpers::HelpersModule {
    #[endpoint(payFeeAndFundESDT)]
    #[payable]
    fn pay_fee_and_fund_esdt(&self, address: ManagedAddress, valability: u64) {
        let mut payments = self.call_value().all_esdt_transfers().clone();
        let fee = EgldOrEsdtTokenPayment::from(payments.get(0).clone());
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &address, fee);

        payments.remove(0);

        self.make_fund(0u64.into(), payments, address, valability)
    }
    #[endpoint(payFeeAndFundEGLD)]
    #[payable("EGLD")]
    fn pay_fee_and_fund_egld(&self, address: ManagedAddress, valability: u64) {
        let mut fund = self.call_value().egld().clone();
        let fee_value = self.fee(&EgldOrEsdtTokenIdentifier::egld()).get();
        require!(fund > fee_value, "payment not covering fees");

        fund -= fee_value.clone();
        let fee = EgldOrEsdtTokenPayment::new(EgldOrEsdtTokenIdentifier::egld(), 0, fee_value);
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &address, fee);

        self.make_fund(fund, ManagedVec::new(), address, valability);
    }

    #[endpoint]
    #[payable]
    fn fund(&self, address: ManagedAddress, valability: u64) {
        require!(!self.deposit(&address).is_empty(), FEES_NOT_COVERED_ERR_MSG);
        let deposit_mapper = self.deposit(&address).get();
        let depositor = deposit_mapper.depositor_address;
        require!(
            self.blockchain().get_caller() == depositor,
            "invalid depositor"
        );
        let deposited_fee_token = deposit_mapper.fees.value;
        let fee_amount = self.fee(&deposited_fee_token.token_identifier).get();
        // TODO: switch to egld+esdt multi transfer handling
        let egld_payment = self.call_value().egld_direct_non_strict().clone();
        let esdt_payment = self.call_value().all_esdt_transfers().clone();

        let num_tokens = self.get_num_token_transfers(&egld_payment, &esdt_payment);
        self.check_fees_cover_number_of_tokens(num_tokens, fee_amount, deposited_fee_token.amount);

        self.make_fund(egld_payment, esdt_payment, address, valability);
    }

    #[endpoint(depositFees)]
    #[payable("EGLD")]
    fn deposit_fees(&self, address: &ManagedAddress) {
        let payment = self.call_value().egld_or_single_esdt();
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, address, payment);
    }
}
