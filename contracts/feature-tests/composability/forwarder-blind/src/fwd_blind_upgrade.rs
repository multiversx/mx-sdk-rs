multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderBlindUpgrade: super::fwd_blind_common::ForwarderBlindCommon {
    #[endpoint(blindUpgrade)]
    fn blind_upgrade(
        &self,
        to: ManagedAddress,
        code: ManagedBuffer,
        code_metadata: CodeMetadata,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.tx()
            .to(to)
            .raw_upgrade()
            .code(code)
            .code_metadata(code_metadata)
            .arguments_raw(args.to_arg_buffer())
            .gas(self.tx_gas())
            .upgrade_async_call_and_exit();
    }

    #[endpoint(blindUpgradeFromSource)]
    fn blind_upgrade_from_source(
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
            .gas(self.tx_gas())
            .upgrade_async_call_and_exit();
    }
}
