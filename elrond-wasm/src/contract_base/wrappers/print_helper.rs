use crate::{
    api::{BlockchainApi, ManagedTypeApi, PrintApi},
    types::BigUint,
};

pub struct PrintHelper<M: ManagedTypeApi> {
    api: M,
}

impl<M: ManagedTypeApi> PrintHelper<M>
where
    M: PrintApi + ManagedTypeApi + BlockchainApi,
{
    pub(crate) fn new(api: M) -> Self {
        PrintHelper { api }
    }

    pub fn print_biguint(&self, biguint: &BigUint<M>) {
        self.api.print_biguint(biguint);
    }
}
