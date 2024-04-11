use multiversx_sc::{derive_imports::*, imports::*};

use multiversx_sc::contract_base::ManagedSerializer;

#[derive(TopEncode, TopDecode)]
pub struct FractionalUriInfo<'a, M: ManagedTypeApi<'a>> {
    pub original_payment: EsdtTokenPayment<'a, M>,
    pub initial_fractional_amount: BigUint<'a, M>,
}

impl<'a, M: ManagedTypeApi<'a>> FractionalUriInfo<'a, M> {
    pub fn new(
        original_payment: EsdtTokenPayment<'a, M>,
        initial_fractional_amount: BigUint<'a, M>,
    ) -> Self {
        Self {
            original_payment,
            initial_fractional_amount,
        }
    }

    pub fn from_uris(uris: ManagedVec<'a, M, ManagedBuffer<'a, M>>) -> Self {
        let first_uri = uris
            .try_get(0)
            .unwrap_or_else(|| M::error_api_impl().signal_error(b"No URIs in fractional token"));
        let serializer = ManagedSerializer::new();
        serializer.top_decode_from_managed_buffer_custom_message(
            &first_uri,
            b"Invalid Fractional URI info",
        )
    }

    pub fn to_uris(&self) -> ManagedVec<'a, M, ManagedBuffer<'a, M>> {
        let first_uri = ManagedSerializer::new().top_encode_to_managed_buffer(&self);
        ManagedVec::from_single_item(first_uri)
    }
}
