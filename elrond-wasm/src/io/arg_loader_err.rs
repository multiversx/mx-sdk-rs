use crate::*;

use core::marker::PhantomData;

pub trait DynArgErrHandler {
    fn handle_sc_error(&self, err: SCError) -> !;
}

// TODO: split ContractIOApi and maybe we won't need this struct anymore
pub struct DynEndpointErrHandler<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static 
{
    api: &'a A,
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'a, A, BigInt, BigUint> DynEndpointErrHandler<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static 
{
    pub fn new(api: &'a A) -> Self {
        DynEndpointErrHandler {
            api: api,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<'a, A, BigInt, BigUint> DynArgErrHandler for DynEndpointErrHandler<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    fn handle_sc_error(&self, err: SCError) -> ! {
        self.api.signal_error(err.as_bytes())
    }
}
