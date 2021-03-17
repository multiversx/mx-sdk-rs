#![no_std]

elrond_wasm::imports!();

/// Test contract for investigating executeOnDestContext and other synchonous calls.
#[elrond_wasm_derive::contract(ForwarderSameShardRawImpl)]
pub trait ForwarderSameShardRaw {
	#[init]
	fn init(&self) {}

	#[endpoint]
	#[payable("EGLD")]
	fn call_execute_on_dest_context(
		&self,
		to: Address,
		#[payment] payment: BigUint,
		endpoint_name: BoxedBytes,
		#[var_args] args: VarArgs<BoxedBytes>,
	) {
		let half_gas = self.get_gas_left() / 2;
		let result = self.send().execute_on_dest_context(
			half_gas,
			&to,
			&payment,
			endpoint_name.as_slice(),
			&ArgBuffer::from(args.into_vec().as_slice()),
		);

		self.execute_on_dest_context_result(result.as_slice());
	}

	#[endpoint]
	#[payable("EGLD")]
	fn call_execute_on_dest_context_twice(
		&self,
		to: Address,
		#[payment] payment: BigUint,
		endpoint_name: BoxedBytes,
		#[var_args] args: VarArgs<BoxedBytes>,
	) {
		let one_third_gas = self.get_gas_left() / 3;
		let half_payment = payment / 2u32.into();
		let arg_buffer = ArgBuffer::from(args.into_vec().as_slice());

		let result = self.send().execute_on_dest_context(
			one_third_gas,
			&to,
			&half_payment,
			endpoint_name.as_slice(),
			&arg_buffer,
		);
		self.execute_on_dest_context_result(result.as_slice());

		let result = self.send().execute_on_dest_context(
			one_third_gas,
			&to,
			&half_payment,
			endpoint_name.as_slice(),
			&arg_buffer,
		);
		self.execute_on_dest_context_result(result.as_slice());
	}

	#[event("execute_on_dest_context_result")]
	fn execute_on_dest_context_result(&self, result: &[BoxedBytes]);
}
