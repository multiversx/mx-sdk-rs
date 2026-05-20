multiversx_sc::imports!();

/// Several alternative constructors, used for testing.
///
/// We are using the multi-contract build system, to avoid having too many SC crates.
/// We need to generate a separate contract for each of these constructors.
#[multiversx_sc::module]
pub trait ForwarderRawAlternativeInit: super::forwarder_raw_common::ForwarderRawCommon {
    /// Will not work, only written for VM testing.
    ///
    /// Async calls are explicitly forbidden in constructors.
    #[init]
    #[label("init-async-call")]
    fn init_async_call(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.tx()
            .to(&to)
            .raw_call(endpoint_name)
            .arguments_raw(args.into_arg_buffer())
            .async_call_and_exit();
    }

    /// Will not work, only written for VM testing.
    ///
    /// Async calls are explicitly forbidden in upgrade constructors.
    ///
    /// TODO: write test once scenario tests support upgrades directly.
    #[upgrade]
    #[label("init-async-call")]
    fn upgrade_async_call(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.init_async_call(to, endpoint_name, args)
    }

    /// Works, but without forwarding EGLD.
    ///
    /// Forwarding EGLD only shows up in a VM test.
    #[init]
    #[payable("EGLD")]
    #[label("init-sync-call")]
    fn init_sync_call(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().egld();
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result = self
            .tx()
            .to(&to)
            .gas(half_gas)
            .egld(payment)
            .raw_call(endpoint_name)
            .arguments_raw(args.into_arg_buffer())
            .returns(ReturnsRawResult)
            .sync_call();

        self.execute_on_dest_context_result(result);
    }
}
