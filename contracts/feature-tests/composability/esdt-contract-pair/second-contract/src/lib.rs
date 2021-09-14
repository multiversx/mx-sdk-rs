#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait SecondContract {
    #[init]
    fn init(&self, esdt_token_identifier: TokenIdentifier) {
        self.set_contract_esdt_token_identifier(&esdt_token_identifier);
    }

    #[payable("*")]
    #[endpoint(acceptEsdtPayment)]
    fn accept_esdt_payment(
        &self,
        #[payment_token] actual_token_identifier: TokenIdentifier,
    ) -> SCResult<()> {
        let expected_token_identifier = self.get_contract_esdt_token_identifier();
        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong esdt token"
        );
        Ok(())
    }

    #[payable("*")]
    #[endpoint(rejectEsdtPayment)]
    fn reject_esdt_payment(&self) -> SCResult<()> {
        sc_error!("Rejected")
    }

    // storage

    #[storage_set("esdtTokenName")]
    fn set_contract_esdt_token_identifier(&self, esdt_token_identifier: &TokenIdentifier);

    #[view(getesdtTokenName)]
    #[storage_get("esdtTokenName")]
    fn get_contract_esdt_token_identifier(&self) -> TokenIdentifier;
}
