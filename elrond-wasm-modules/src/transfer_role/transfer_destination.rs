elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait TransferDestinationModule {
    #[only_owner]
    #[endpoint(addContractToWhitelist)]
    fn add_contract_to_whitelist(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        let mut mapper = self.contract_whitelist();
        for addr in addresses {
            let _ = mapper.insert(addr);
        }
    }

    #[only_owner]
    #[endpoint(removeContractFromWhitelist)]
    fn remove_contract_from_whitelist(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        let mut mapper = self.contract_whitelist();
        for addr in addresses {
            let _ = mapper.swap_remove(&addr);
        }
    }

    #[payable("*")]
    #[endpoint(receiveFunds)]
    fn receive_funds(&self, _original_caller: ManagedAddress) {}

    fn require_valid_sender(&self, addr: &ManagedAddress) {
        require!(
            self.contract_whitelist().contains(addr),
            "Address not in whitelist"
        );
    }

    #[view(getContractWhitelist)]
    #[storage_mapper("transfer_destination:contractWhitelist")]
    fn contract_whitelist(&self) -> UnorderedSetMapper<ManagedAddress>;
}
