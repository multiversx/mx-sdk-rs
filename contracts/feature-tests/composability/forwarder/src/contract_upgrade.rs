elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait UpgradeContractModule {
    #[proxy]
    fn vault_proxy(&self, sc_address: ManagedAddress) -> vault::Proxy<Self::Api>;

    #[endpoint(upgradeVault)]
    fn upgrade_vault(
        &self,
        child_sc_address: ManagedAddress,
        new_code: ManagedBuffer,
        #[var_args] opt_arg: OptionalArg<ManagedBuffer>,
    ) {
        self.vault_proxy(child_sc_address)
            .init(opt_arg)
            .upgrade_contract(&new_code, CodeMetadata::UPGRADEABLE);
    }

    #[endpoint]
    fn upgrade_vault_from_source(
        &self,
        child_sc_address: ManagedAddress,
        source_address: ManagedAddress,
        #[var_args] opt_arg: OptionalArg<ManagedBuffer>,
    ) {
        self.vault_proxy(child_sc_address)
            .init(opt_arg)
            .upgrade_from_source(&source_address, CodeMetadata::DEFAULT)
    }
}
