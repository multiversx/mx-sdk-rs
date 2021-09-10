elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait UpgradeContractModule {
    #[proxy]
    fn vault_proxy(&self, sc_address: ManagedAddress) -> vault::Proxy<Self::Api>;

    #[endpoint(upgradeChildContract)]
    fn upgrade_child_contract(
        &self,
        child_sc_address: ManagedAddress,
        new_code: ManagedBuffer,
        #[var_args] arguments: VarArgs<ManagedBuffer>,
    ) {
        self.upgrade(
            &child_sc_address,
            &new_code,
            arguments.into_vec().managed_into(),
        );
    }

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

    fn upgrade(
        &self,
        child_sc_address: &ManagedAddress,
        new_code: &ManagedBuffer,
        arguments: ManagedVec<Self::TypeManager, ManagedBuffer>,
    ) {
        // TODO: use proxies to perform upgrade here
        // raw upgrade belongs to forwarder-raw
        self.raw_vm_api().upgrade_contract(
            child_sc_address,
            self.blockchain().get_gas_left(),
            &self.types().big_uint_zero(),
            new_code,
            CodeMetadata::UPGRADEABLE,
            &arguments.managed_into(),
        );
    }
}
