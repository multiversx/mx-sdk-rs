use multiversx_sc::{derive_imports::*, imports::*};

pub(crate) const PERCENTAGE_DIVISOR: u32 = 10_000; // dividing the percentage fee by this number will result in a 2 decimal percentage

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Eq, Clone)]
pub enum Fee<M>
where
    M: ManagedTypeApi,
{
    Unset,
    ExactValue(EsdtTokenPayment<M>),
    Percentage(u32),
}
