use crate::*;
use core::marker::PhantomData;


pub struct EndpointDynArgLoader<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    api: &'a A,
    current_index: i32,
    num_arguments: i32,
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'a, A, BigInt, BigUint> EndpointDynArgLoader<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    pub fn new(api: &'a A) -> Self {
        EndpointDynArgLoader {
            api,
            current_index : 0,
            num_arguments: api.get_num_arguments(),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<'a, A, BigInt, BigUint> SignalError for EndpointDynArgLoader<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    #[inline]
    fn signal_error(&self, message: &[u8]) -> ! {
        self.api.signal_error(message)
    }
}


impl<'a, A, BigInt, BigUint> DynArgInput<ArgDecodeInput<'a, A, BigInt, BigUint>> for EndpointDynArgLoader<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    #[inline]
    fn has_next(&self) -> bool {
        self.current_index < self.num_arguments
    }

    fn next_arg_input(&mut self) -> Option<ArgDecodeInput<'a, A, BigInt, BigUint>> {
        if self.current_index >= self.num_arguments {
            None
        } else {
            let arg_input = ArgDecodeInput::new(self.api, self.current_index);
            self.current_index += 1;
            Some(arg_input)
        }
    }
}
