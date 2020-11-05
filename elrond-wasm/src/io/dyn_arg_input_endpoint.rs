use crate::*;
use core::marker::PhantomData;

pub struct EndpointDynArgLoader<A, BigInt, BigUint>
where
	BigUint: BigUintApi + 'static,
	BigInt: BigIntApi<BigUint> + 'static,
	A: ContractIOApi<BigInt, BigUint>,
{
	api: A,
	current_index: i32,
	num_arguments: i32,
	_phantom1: PhantomData<BigInt>,
	_phantom2: PhantomData<BigUint>,
}

impl<A, BigInt, BigUint> EndpointDynArgLoader<A, BigInt, BigUint>
where
	BigUint: BigUintApi + 'static,
	BigInt: BigIntApi<BigUint> + 'static,
	A: ContractIOApi<BigInt, BigUint>,
{
	pub fn new(api: A) -> Self {
		let num_arguments = api.get_num_arguments();
		EndpointDynArgLoader {
			api,
			current_index: 0,
			num_arguments,
			_phantom1: PhantomData,
			_phantom2: PhantomData,
		}
	}
}

impl<A, BigInt, BigUint> SignalError for EndpointDynArgLoader<A, BigInt, BigUint>
where
	BigUint: BigUintApi + 'static,
	BigInt: BigIntApi<BigUint> + 'static,
	A: ContractIOApi<BigInt, BigUint> + 'static,
{
	#[inline]
	fn signal_error(&self, message: &[u8]) -> ! {
		self.api.signal_error(message)
	}
}

impl<A, BigInt, BigUint> DynArgInput<ArgDecodeInput<A, BigInt, BigUint>>
	for EndpointDynArgLoader<A, BigInt, BigUint>
where
	BigUint: BigUintApi + 'static,
	BigInt: BigIntApi<BigUint> + 'static,
	A: ContractIOApi<BigInt, BigUint> + 'static,
{
	fn has_next(&self) -> bool {
		self.current_index < self.num_arguments
	}

	fn next_arg_input(&mut self) -> ArgDecodeInput<A, BigInt, BigUint> {
		if self.current_index >= self.num_arguments {
			self.signal_arg_wrong_number()
		} else {
			let arg_input = ArgDecodeInput::new(self.api.clone(), self.current_index);
			self.current_index += 1;
			arg_input
		}
	}
}
