multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderRawSync: super::forwarder_raw_common::ForwarderRawCommon {
    #[endpoint]
    #[payable("EGLD")]
    fn call_execute_on_dest_context(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().egld().clone_value();
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self
            .tx()
            .to(to)
            .egld(payment)
            .raw_call(endpoint_name)
            .argument(&args)
            .gas(half_gas)
            .returns(ReturnsRawResult)
            .sync_call();

        self.execute_on_dest_context_result(result);
    }

    #[endpoint]
    #[payable("EGLD")]
    fn call_execute_on_dest_context_twice(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().egld();
        let one_third_gas = self.blockchain().get_gas_left() / 3;
        let half_payment = &*payment / 2u32;
        let arg_buffer = args.to_arg_buffer();

        let result = self
            .tx()
            .to(&to)
            .gas(one_third_gas)
            .egld(&half_payment)
            .raw_call(endpoint_name.clone())
            .arguments_raw(arg_buffer.clone())
            .returns(ReturnsRawResult)
            .sync_call();

        self.execute_on_dest_context_result(result);

        let result = self
            .tx()
            .to(&to)
            .gas(one_third_gas)
            .egld(&half_payment)
            .raw_call(endpoint_name)
            .arguments_raw(arg_buffer)
            .returns(ReturnsRawResult)
            .sync_call();

        self.execute_on_dest_context_result(result);
    }

    #[endpoint]
    #[payable("EGLD")]
    fn call_execute_on_same_context(
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
            .arguments_raw(args.to_arg_buffer())
            .returns(ReturnsRawResult)
            .sync_call_same_context();

        self.execute_on_same_context_result(result);
    }

    #[endpoint]
    fn call_execute_on_dest_context_readonly(
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
            .payment(NotPayable) // `()` and `NotPayable` both work
            .returns(ReturnsRawResult)
            .sync_call_readonly();

        self.execute_on_dest_context_result(result);
    }
}
