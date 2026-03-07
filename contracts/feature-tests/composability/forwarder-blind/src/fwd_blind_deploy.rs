multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderBlindDeploy: super::fwd_blind_common::ForwarderBlindCommon {
    #[endpoint]
    fn blind_deploy(
        &self,
        code: ManagedBuffer,
        code_metadata: CodeMetadata,
        args: MultiValueEncoded<ManagedBuffer>,
    ) -> ManagedAddress {
        let (new_address, raw_results) = self
            .tx()
            .raw_deploy()
            .code(code)
            .code_metadata(code_metadata)
            .arguments_raw(args.to_arg_buffer())
            .gas(self.tx_gas())
            .returns(ReturnsNewManagedAddress)
            .returns(ReturnsRawResult)
            .sync_call();

        self.blind_deploy_ok_event(&new_address, &raw_results.into());

        new_address
    }

    #[event("blind_deploy_ok")]
    fn blind_deploy_ok_event(
        &self,
        #[indexed] new_address: &ManagedAddress,
        #[indexed] results: &MultiValueEncoded<ManagedBuffer>,
    );
}
