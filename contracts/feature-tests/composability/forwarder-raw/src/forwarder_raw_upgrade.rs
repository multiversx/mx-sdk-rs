multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderRawUpgrade {
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
            .arguments_raw(args.to_arg_buffer())
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
            .arguments_raw(args.to_arg_buffer())
            .gas(self.blockchain().get_gas_left())
            .upgrade_async_call_and_exit();
    }
}
