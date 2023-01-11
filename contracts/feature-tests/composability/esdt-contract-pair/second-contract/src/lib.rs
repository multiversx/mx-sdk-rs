#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait SecondContract {
    #[init]
    fn init(&self, esdt_token_identifier: EgldOrEsdtTokenIdentifier) {
        self.set_contract_esdt_token_identifier(&esdt_token_identifier);
    }

    #[payable("*")]
    #[endpoint(acceptEsdtPayment)]
    fn accept_esdt_payment(&self) {
        let actual_token_identifier = self.call_value().egld_or_single_esdt().token_identifier;
        let expected_token_identifier = self.get_contract_esdt_token_identifier();
        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong esdt token"
        );
    }

    #[payable("*")]
    #[endpoint(rejectEsdtPayment)]
    fn reject_esdt_payment(&self) {
        sc_panic!("Rejected")
    }

    // storage

    #[storage_set("esdtTokenName")]
    fn set_contract_esdt_token_identifier(&self, esdt_token_identifier: &EgldOrEsdtTokenIdentifier);

    #[view(getesdtTokenName)]
    #[storage_get("esdtTokenName")]
    fn get_contract_esdt_token_identifier(&self) -> EgldOrEsdtTokenIdentifier;
}
