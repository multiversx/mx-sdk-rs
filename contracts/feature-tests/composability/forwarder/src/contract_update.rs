elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait UpgradeContractModule {
    #[endpoint(upgradeChildContract)]
    fn upgrade_child_contract(
        &self,
        child_sc_address: Address,
        new_code: BoxedBytes,
        #[var_args] arguments: VarArgs<BoxedBytes>,
    ) {
        self.upgrade(&child_sc_address, &new_code, &arguments.into_vec());
    }

    fn upgrade(&self, child_sc_address: &Address, new_code: &BoxedBytes, arguments: &[BoxedBytes]) {
        self.send().upgrade_contract(
            child_sc_address,
            self.blockchain().get_gas_left(),
            &Self::BigUint::zero(),
            new_code,
            CodeMetadata::DEFAULT,
            &arguments.into(),
        );
    }
}
