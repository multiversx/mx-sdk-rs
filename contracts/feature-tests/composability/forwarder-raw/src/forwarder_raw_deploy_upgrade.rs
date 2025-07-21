multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderRawDeployUpgrade {
    #[endpoint]
    fn deploy_contract(
        &self,
        code: ManagedBuffer,
        code_metadata: CodeMetadata,
        args: MultiValueEncoded<ManagedBuffer>,
    ) -> MultiValue2<ManagedAddress, ManagedVec<Self::Api, ManagedBuffer>> {
        self.tx()
            .raw_deploy()
            .code(code)
            .code_metadata(code_metadata)
            .arguments_raw(args.into_arg_buffer())
            .gas(self.blockchain().get_gas_left())
            .returns(ReturnsNewManagedAddress)
            .returns(ReturnsRawResult)
            .sync_call()
            .into()
    }

    #[endpoint]
    fn deploy_from_source(
        &self,
        source_contract_address: ManagedAddress,
        code_metadata: CodeMetadata,
        args: MultiValueEncoded<ManagedBuffer>,
    ) -> ManagedAddress {
        self.tx()
            .raw_deploy()
            .from_source(source_contract_address)
            .code_metadata(code_metadata)
            .arguments_raw(args.into_arg_buffer())
            .gas(self.blockchain().get_gas_left())
            .returns(ReturnsNewManagedAddress)
            .sync_call()
    }

    #[endpoint]
    fn call_upgrade(
        &self,
        child_sc_address: ManagedAddress,
        new_code: ManagedBuffer,
        code_metadata: CodeMetadata,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.tx()
            .to(child_sc_address)
            .raw_upgrade()
            .code(new_code)
            .code_metadata(code_metadata)
            .arguments_raw(args.into_arg_buffer())
            .upgrade_async_call_and_exit();
    }

    #[endpoint]
    fn call_upgrade_from_source(
        &self,
        sc_address: ManagedAddress,
        source_contract_address: ManagedAddress,
        code_metadata: CodeMetadata,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.tx()
            .to(sc_address)
            .raw_upgrade()
            .from_source(source_contract_address)
            .code_metadata(code_metadata)
            .arguments_raw(args.into_arg_buffer())
            .gas(self.blockchain().get_gas_left())
            .upgrade_async_call_and_exit();
    }
}
