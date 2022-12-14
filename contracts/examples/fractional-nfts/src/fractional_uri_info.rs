elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use elrond_wasm::contract_base::ManagedSerializer;

#[derive(TopEncode, TopDecode)]
pub struct FractionalUriInfo<M: ManagedTypeApi> {
    pub original_payment: EsdtTokenPayment<M>,
    pub initial_fractional_amount: BigUint<M>,
}

impl<M: ManagedTypeApi> FractionalUriInfo<M> {
    pub fn new(
        original_payment: EsdtTokenPayment<M>,
        initial_fractional_amount: BigUint<M>,
    ) -> Self {
        Self {
            original_payment,
            initial_fractional_amount,
        }
    }

    pub fn from_uris(uris: ManagedVec<M, ManagedBuffer<M>>) -> Self {
        let first_uri = uris.get(0);
        let serializer = ManagedSerializer::new();
        serializer.top_decode_from_managed_buffer(&first_uri)
    }

    pub fn to_uris(&self) -> ManagedVec<M, ManagedBuffer<M>> {
        let first_uri = ManagedSerializer::new().top_encode_to_managed_buffer(&self);
        let mut uris = ManagedVec::new();
        uris.push(first_uri);
        uris
    }
}
