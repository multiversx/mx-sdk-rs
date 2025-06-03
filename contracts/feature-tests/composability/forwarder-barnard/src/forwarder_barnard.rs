#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait ForwarderBarnard {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn call_execute_on_dest_context_fallible(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self
            .tx()
            .to(&to)
            .gas(half_gas)
            .raw_call(endpoint_name)
            .arguments_raw(args.to_arg_buffer())
            .returns(ReturnsHandledOrError::new().returns(ReturnsRawResult))
            .sync_call_fallible();

        match result {
            Ok(success) => {
                self.sync_call_fallible_success(success);
            },
            Err(error_code) => {
                self.sync_call_fallible_error(error_code);
            },
        }
    }

    #[event("sync_call_fallible_success")]
    fn sync_call_fallible_success(&self, result: ManagedVec<Self::Api, ManagedBuffer>);

    #[event("sync_call_fallible_error")]
    fn sync_call_fallible_error(&self, error_code: u32);
}
