#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait SecondContract {
    #[init]
    fn init(&self, esdt_token_identifier: TokenId) {
        self.set_contract_esdt_token_identifier(&esdt_token_identifier);
    }

    #[payable("*")]
    #[endpoint(acceptEsdtPayment)]
    fn accept_esdt_payment(&self) {
        let payment = self.call_value().single();
        let expected_token_identifier = self.get_contract_esdt_token_identifier();
        require!(
            payment.token_identifier == expected_token_identifier,
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
    fn set_contract_esdt_token_identifier(&self, esdt_token_identifier: &TokenId);

    #[view(getesdtTokenName)]
    #[storage_get("esdtTokenName")]
    fn get_contract_esdt_token_identifier(&self) -> TokenId;
}
