elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait UpgradeContractModule {
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
            arguments.into_vec().managed_into(self.type_manager()),
        );
    }

    fn upgrade(
        &self,
        child_sc_address: &ManagedAddress,
        new_code: &ManagedBuffer,
        arguments: ManagedVec<Self::TypeManager, ManagedBuffer>,
    ) {
        self.raw_vm_api().upgrade_contract(
            child_sc_address,
            self.blockchain().get_gas_left(),
            &self.types().big_uint_zero(),
            new_code,
            CodeMetadata::DEFAULT,
            &arguments.managed_into(self.type_manager()),
        );
    }
}
