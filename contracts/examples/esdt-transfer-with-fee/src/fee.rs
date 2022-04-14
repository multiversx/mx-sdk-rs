elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub(crate) const PERCENTAGE_DIVISOR: u32 = 10_000; // dividing the percentage fee by this number will result in a 4 decimal percentage

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone)]
pub enum Fee<M>
where
    M: ManagedTypeApi,
{
    ExactValue(EsdtTokenPayment<M>),
    Percentage(u32),
}
