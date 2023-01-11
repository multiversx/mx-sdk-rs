multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub(crate) const PERCENTAGE_DIVISOR: u32 = 10_000; // dividing the percentage fee by this number will result in a 2 decimal percentage

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Eq, Clone)]
pub enum Fee<M>
where
    M: ManagedTypeApi,
{
    Unset,
    ExactValue(EsdtTokenPayment<M>),
    Percentage(u32),
}
