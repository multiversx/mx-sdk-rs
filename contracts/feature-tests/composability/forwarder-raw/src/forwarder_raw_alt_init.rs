multiversx_sc::imports!();

/// Several alternative constructors, used for testing.
/// 
/// We are using the multi-contract build system, to avoid having too many SC crates.
/// We need to generate a separate contract for each of these constructors.
#[multiversx_sc::module]
pub trait ForwarderRawAlterativeInit {
    #[init]
    #[label("init-async-call")]
    fn init_async_call(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.send()
            .contract_call::<()>(to, endpoint_name)
            .with_raw_arguments(args.to_arg_buffer())
            .async_call()
            .call_and_exit();
    }

    #[init]
    #[label("init-sync-call")]
    fn init_sync_call(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.send()
            .contract_call::<()>(to, endpoint_name)
            .with_raw_arguments(args.to_arg_buffer())
            .execute_on_dest_context::<()>();
    }
}
