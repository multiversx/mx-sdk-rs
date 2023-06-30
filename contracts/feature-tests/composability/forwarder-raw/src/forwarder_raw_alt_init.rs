multiversx_sc::imports!();

/// Several alternative constructors, used for testing.
///
/// We are using the multi-contract build system, to avoid having too many SC crates.
/// We need to generate a separate contract for each of these constructors.
#[multiversx_sc::module]
pub trait ForwarderRawAlterativeInit: super::forwarder_raw_common::ForwarderRawCommon {
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
        self.send()
            .contract_call::<()>(to, endpoint_name)
            .with_raw_arguments(args.to_arg_buffer())
            .async_call()
            .call_and_exit();
    }

    /// Will not work, only written for VM testing.
    ///
    /// Async calls are explicitly forbidden in upgrade constructors.
    ///
    /// TODO: write test once scenario tests support upgrades directly.
    #[endpoint(upgrade)]
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
        let payment = self.call_value().egld_value();
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self.send_raw().execute_on_dest_context_raw(
            half_gas,
            &to,
            &payment,
            &endpoint_name,
            &args.to_arg_buffer(),
        );

        self.execute_on_dest_context_result(result);
    }
}
