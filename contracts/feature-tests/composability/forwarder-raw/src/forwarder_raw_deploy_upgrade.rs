multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderRawDeployUpgrade {
    #[endpoint]
    fn deploy_contract(
        &self,
        code: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) -> MultiValue2<ManagedAddress, ManagedVec<Self::Api, ManagedBuffer>> {
        self.send_raw()
            .deploy_contract(
                self.blockchain().get_gas_left(),
                &BigUint::zero(),
                &code,
                CodeMetadata::DEFAULT,
                &args.to_arg_buffer(),
            )
            .into()
    }

    #[endpoint]
    fn deploy_from_source(
        &self,
        source_contract_address: ManagedAddress,
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) -> ManagedAddress {
        let (address, _) = self.send_raw().deploy_from_source_contract(
            self.blockchain().get_gas_left(),
            &BigUint::zero(),
            &source_contract_address,
            CodeMetadata::DEFAULT,
            &arguments.to_arg_buffer(),
        );

        address
    }

    #[endpoint]
    fn call_upgrade(
        &self,
        child_sc_address: &ManagedAddress,
        new_code: &ManagedBuffer,
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.send_raw().upgrade_contract(
            child_sc_address,
            self.blockchain().get_gas_left(),
            &BigUint::zero(),
            new_code,
            CodeMetadata::UPGRADEABLE,
            &arguments.to_arg_buffer(),
        );
    }

    #[endpoint]
    fn call_upgrade_from_source(
        &self,
        sc_address: ManagedAddress,
        source_contract_address: ManagedAddress,
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.send_raw().upgrade_from_source_contract(
            &sc_address,
            self.blockchain().get_gas_left(),
            &BigUint::zero(),
            &source_contract_address,
            CodeMetadata::DEFAULT,
            &arguments.to_arg_buffer(),
        )
    }
}
