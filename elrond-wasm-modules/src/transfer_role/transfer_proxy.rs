elrond_wasm::imports!();

use elrond_wasm::elrond_codec::Empty;

const CALLBACK_RESERVED_GAS_PER_TOKEN: u64 = 1_000_000;

#[elrond_wasm::module]
pub trait TransferProxyModule {
    fn transfer_to_user(
        &self,
        original_caller: ManagedAddress,
        dest: ManagedAddress,
        payments: ManagedVec<EsdtTokenPayment<Self::Api>>,
        data: ManagedBuffer,
    ) -> ! {
        let contract_call =
            ContractCall::<Self::Api, Empty>::new_with_esdt_payment(dest, data, payments);

        self.execute_async_call(original_caller, contract_call);
    }

    fn transfer_to_contract_typed_call(
        &self,
        original_caller: ManagedAddress,
        mut contract_call: ContractCall<Self::Api, Empty>,
    ) -> ! {
        let mut original_caller_arg = ManagedArgBuffer::new_empty();
        original_caller_arg.push_arg(original_caller.clone());
        contract_call.arg_buffer = original_caller_arg.concat(contract_call.arg_buffer);

        self.execute_async_call(original_caller, contract_call);
    }

    fn transfer_to_contract_raw(
        &self,
        original_caller: ManagedAddress,
        dest: ManagedAddress,
        payments: ManagedVec<EsdtTokenPayment<Self::Api>>,
        endpoint_name: ManagedBuffer,
        args: ManagedArgBuffer<Self::Api>,
    ) -> ! {
        let mut contract_call =
            ContractCall::<Self::Api, Empty>::new_with_esdt_payment(dest, endpoint_name, payments);
        contract_call.arg_buffer.push_arg(original_caller.clone());
        contract_call.arg_buffer = contract_call.arg_buffer.concat(args);

        self.execute_async_call(original_caller, contract_call);
    }

    fn execute_async_call(
        &self,
        original_caller: ManagedAddress,
        contract_call: ContractCall<Self::Api, Empty>,
    ) -> ! {
        let remaining_gas = self.blockchain().get_gas_left();
        let cb_gas_needed = CALLBACK_RESERVED_GAS_PER_TOKEN * contract_call.payments.len() as u64;
        require!(
            remaining_gas > cb_gas_needed,
            "Not enough gas to launch async call"
        );

        let async_call_gas = remaining_gas - cb_gas_needed;
        let cb = TransferProxyModule::callbacks(self).transfer_callback(original_caller);
        contract_call
            .with_gas_limit(async_call_gas)
            .async_call()
            .with_callback(cb)
            .call_and_exit();
    }

    #[callback]
    fn transfer_callback(
        &self,
        original_caller: ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<MultiValueEncoded<ManagedBuffer>>,
    ) -> MultiValueEncoded<ManagedBuffer> {
        match result {
            ManagedAsyncCallResult::Ok(return_values) => return_values,
            ManagedAsyncCallResult::Err(err) => {
                let returned_tokens = self.call_value().all_esdt_transfers();
                if returned_tokens.len() > 0 {
                    self.send()
                        .direct_multi(&original_caller, &returned_tokens, &[]);
                }

                let mut err_result = MultiValueEncoded::new();
                err_result.push(err.err_msg);

                err_result
            },
        }
    }
}
