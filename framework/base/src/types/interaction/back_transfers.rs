use crate::{
    api::ManagedTypeApi,
    types::{BigUint, MultiEsdtPayment},
};

/// Holding back-transfer data, as retrieved from the VM.
pub struct BackTransfers<'a, A>
where
    A: ManagedTypeApi<'a>,
{
    pub total_egld_amount: BigUint<'a, A>,
    pub esdt_payments: MultiEsdtPayment<'a, A>,
}
