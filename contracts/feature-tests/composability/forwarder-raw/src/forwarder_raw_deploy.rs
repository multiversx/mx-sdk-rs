multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderRawDeploy {
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
            .arguments_raw(args.to_arg_buffer())
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
            .arguments_raw(args.to_arg_buffer())
            .gas(self.blockchain().get_gas_left())
            .returns(ReturnsNewManagedAddress)
            .sync_call()
    }
}
