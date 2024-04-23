#![no_std]

multiversx_sc::imports!();

const ESDT_TRANSFER_STRING: &str = "ESDTTransfer";
const SECOND_CONTRACT_ACCEPT_ESDT_PAYMENT: &str = "acceptEsdtPayment";
const SECOND_CONTRACT_REJECT_ESDT_PAYMENT: &str = "rejectEsdtPayment";

#[multiversx_sc::contract]
pub trait FirstContract {
    #[init]
    fn init(
        &self,
        esdt_token_identifier: TokenIdentifier,
        second_contract_address: ManagedAddress,
    ) {
        self.set_contract_esdt_token_identifier(&esdt_token_identifier);
        self.set_second_contract_address(&second_contract_address);
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractFull)]
    fn transfer_to_second_contract_full(&self) {
        let (actual_token_identifier, esdt_value) = self.call_value().single_fungible_esdt();
        let expected_token_identifier = self.get_contract_esdt_token_identifier();

        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong esdt token"
        );

        self.call_esdt_second_contract(
            &expected_token_identifier,
            &esdt_value,
            &self.get_second_contract_address(),
            &ManagedBuffer::from(SECOND_CONTRACT_ACCEPT_ESDT_PAYMENT),
            &ManagedVec::new(),
        );
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractHalf)]
    fn transfer_to_second_contract_half(&self) {
        let (actual_token_identifier, esdt_value) = self.call_value().single_fungible_esdt();
        let expected_token_identifier = self.get_contract_esdt_token_identifier();

        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong esdt token"
        );

        self.call_esdt_second_contract(
            &expected_token_identifier,
            &(esdt_value / 2u32),
            &self.get_second_contract_address(),
            &ManagedBuffer::from(SECOND_CONTRACT_ACCEPT_ESDT_PAYMENT),
            &ManagedVec::new(),
        );
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractRejected)]
    fn transfer_to_second_contract_rejected(&self) {
        let (actual_token_identifier, esdt_value) = self.call_value().single_fungible_esdt();
        let expected_token_identifier = self.get_contract_esdt_token_identifier();

        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong esdt token"
        );

        self.call_esdt_second_contract(
            &expected_token_identifier,
            &esdt_value,
            &self.get_second_contract_address(),
            &ManagedBuffer::from(SECOND_CONTRACT_REJECT_ESDT_PAYMENT),
            &ManagedVec::new(),
        );
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractRejectedWithTransferAndExecute)]
    fn transfer_to_second_contract_rejected_with_transfer_and_execute(&self) {
        let (actual_token_identifier, esdt_value) = self.call_value().single_fungible_esdt();
        let second_contract_address = self.get_second_contract_address();
        let expected_token_identifier = self.get_contract_esdt_token_identifier();

        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong esdt token"
        );

        let gas_left = self.blockchain().get_gas_left();
        self.tx()
            .to(&second_contract_address)
            .gas(gas_left)
            .raw_call(SECOND_CONTRACT_REJECT_ESDT_PAYMENT)
            .single_esdt(&expected_token_identifier, 0u64, &esdt_value)
            .transfer_execute();
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractFullWithTransferAndExecute)]
    fn transfer_to_second_contract_full_with_transfer_and_execute(&self) {
        let (actual_token_identifier, esdt_value) = self.call_value().single_fungible_esdt();
        let second_contract_address = self.get_second_contract_address();
        let expected_token_identifier = self.get_contract_esdt_token_identifier();

        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong esdt token"
        );

        let gas_left = self.blockchain().get_gas_left();
        self.tx()
            .to(&second_contract_address)
            .gas(gas_left)
            .raw_call(SECOND_CONTRACT_ACCEPT_ESDT_PAYMENT)
            .single_esdt(&expected_token_identifier, 0u64, &esdt_value)
            .transfer_execute();
    }

    fn call_esdt_second_contract(
        &self,
        esdt_token_identifier: &TokenIdentifier,
        amount: &BigUint,
        to: &ManagedAddress,
        func_name: &ManagedBuffer,
        args: &ManagedVec<Self::Api, ManagedBuffer>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        arg_buffer.push_arg(esdt_token_identifier);
        arg_buffer.push_arg(amount);
        arg_buffer.push_arg(func_name);
        for arg in args.into_iter() {
            arg_buffer.push_arg_raw(arg);
        }

        self.tx()
            .to(to)
            .raw_call(ESDT_TRANSFER_STRING)
            .arguments_raw(arg_buffer)
            .async_call_and_exit();
    }

    // storage

    #[storage_set("esdtTokenName")]
    fn set_contract_esdt_token_identifier(&self, esdt_token_identifier: &TokenIdentifier);

    #[view(getesdtTokenName)]
    #[storage_get("esdtTokenName")]
    fn get_contract_esdt_token_identifier(&self) -> TokenIdentifier;

    #[storage_set("secondContractAddress")]
    fn set_second_contract_address(&self, address: &ManagedAddress);

    #[view(getSecondContractAddress)]
    #[storage_get("secondContractAddress")]
    fn get_second_contract_address(&self) -> ManagedAddress;
}
