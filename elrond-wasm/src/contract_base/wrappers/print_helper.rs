use core::marker::PhantomData;

use crate::{
    api::{BlockchainApi, ManagedTypeApi, PrintApi, PrintApiImpl},
    types::BigUint,
};

pub struct PrintHelper<A>
where
    A: PrintApi + ManagedTypeApi,
{
    _phantom: PhantomData<A>,
}

impl<A> PrintHelper<A>
where
    A: PrintApi + ManagedTypeApi,
{
    pub(crate) fn new() -> Self {
        PrintHelper {
            _phantom: PhantomData,
        }
    }

    pub fn print_biguint(&self, biguint: &BigUint<A>) {
        A::print_api_impl().print_biguint(biguint);
    }
}
