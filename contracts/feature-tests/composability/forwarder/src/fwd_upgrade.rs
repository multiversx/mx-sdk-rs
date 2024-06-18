use crate::vault_upgrade_proxy;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait UpgradeContractModule {
    #[endpoint(upgradeVault)]
    fn upgrade_vault(
        &self,
        child_sc_address: ManagedAddress,
        new_code: ManagedBuffer,
        opt_arg: OptionalValue<ManagedBuffer>,
    ) {
        self.tx()
            .to(child_sc_address)
            .typed(vault_upgrade_proxy::VaultProxy)
            .upgrade(opt_arg)
            .code(new_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .upgrade_async_call_and_exit();
    }

    #[endpoint]
    fn upgrade_vault_from_source(
        &self,
        child_sc_address: ManagedAddress,
        source_address: ManagedAddress,
        opt_arg: OptionalValue<ManagedBuffer>,
    ) {
        self.tx()
            .to(child_sc_address)
            .typed(vault_upgrade_proxy::VaultProxy)
            .upgrade(opt_arg)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .from_source(source_address)
            .upgrade_async_call_and_exit();
    }
}
