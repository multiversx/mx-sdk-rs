elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait UpgradeContractModule {
    #[proxy]
    fn vault_proxy(&self, sc_address: ManagedAddress) -> vault::Proxy<Self::SendApi>;

    #[endpoint(upgradeChildContract)]
    fn upgrade_child_contract(
        &self,
        child_sc_address: ManagedAddress,
        new_code: ManagedBuffer,
        #[var_args] arguments: VarArgs<ManagedBuffer>,
    ) -> ManagedVec<Self::TypeManager, ManagedBuffer> {
        self.upgrade(
            &child_sc_address,
            &new_code,
            arguments.into_vec().managed_into(self.type_manager()),
        )
    }

    #[endpoint(upgradeVault)]
    fn upgrade_vault(
        &self,
        child_sc_address: ManagedAddress,
        new_code: ManagedBuffer,
        #[var_args] opt_arg: OptionalArg<ManagedBuffer>,
    ) -> ManagedVec<Self::TypeManager, ManagedBuffer> {
        self.vault_proxy(child_sc_address)
            .init(opt_arg)
            .upgrade_contract(&new_code, CodeMetadata::UPGRADEABLE)
    }

    fn upgrade(
        &self,
        child_sc_address: &ManagedAddress,
        new_code: &ManagedBuffer,
        arguments: ManagedVec<Self::TypeManager, ManagedBuffer>,
    ) -> ManagedVec<Self::TypeManager, ManagedBuffer> {
        self.send().upgrade_contract(
            child_sc_address,
            self.blockchain().get_gas_left(),
            &self.types().big_uint_zero(),
            new_code,
            CodeMetadata::UPGRADEABLE,
            &arguments.managed_into(self.type_manager()),
        )
    }
}
