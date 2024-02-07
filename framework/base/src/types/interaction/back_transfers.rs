use crate::{
    api::ManagedTypeApi,
    types::{BigUint, MultiEsdtPayment},
};

/// Holding back-transfer data, as retrieved from the VM.
pub struct BackTransfers<A>
where
    A: ManagedTypeApi,
{
    pub total_egld_amount: BigUint<A>,
    pub esdt_payments: MultiEsdtPayment<A>,
}
