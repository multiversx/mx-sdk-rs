use multiversx_sc_derive::ManagedVecItem;

use crate::{
    api::ManagedTypeApi,
    codec,
    codec::{
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
        *,
    },
    contract_base::ExitCodecErrorHandler,
    types::{BigUint, EsdtTokenType, ManagedAddress, ManagedBuffer, ManagedVec},
};

use crate as multiversx_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

const DECODE_ATTRIBUTE_ERROR_PREFIX: &[u8] = b"error decoding ESDT attributes: ";

#[derive(
    Clone, TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Debug, ManagedVecItem,
)]
pub struct EsdtTokenData<'a, M: ManagedTypeApi<'a>> {
    pub token_type: EsdtTokenType,
    pub amount: BigUint<'a, M>,
    pub frozen: bool,
    pub hash: ManagedBuffer<'a, M>,
    pub name: ManagedBuffer<'a, M>,
    pub attributes: ManagedBuffer<'a, M>,
    pub creator: ManagedAddress<'a, M>,
    pub royalties: BigUint<'a, M>,
    pub uris: ManagedVec<'a, M, ManagedBuffer<'a, M>>,
}

impl<'a, M: ManagedTypeApi<'a>> Default for EsdtTokenData<'a, M> {
    fn default() -> Self {
        EsdtTokenData {
            token_type: EsdtTokenType::Fungible,
            amount: BigUint::zero(),
            frozen: false,
            hash: ManagedBuffer::new(),
            name: ManagedBuffer::new(),
            attributes: ManagedBuffer::new(),
            creator: ManagedAddress::zero(),
            royalties: BigUint::zero(),
            uris: ManagedVec::new(),
        }
    }
}

impl<'a, M: ManagedTypeApi<'a>> EsdtTokenData<'a, M> {
    pub fn try_decode_attributes<T: TopDecode>(&self) -> Result<T, DecodeError> {
        T::top_decode(self.attributes.clone()) // TODO: remove clone
    }

    pub fn decode_attributes<T: TopDecode>(&self) -> T {
        let Ok(value) = T::top_decode_or_handle_err(
            self.attributes.clone(), // TODO: remove clone
            ExitCodecErrorHandler::<'a, M>::from(DECODE_ATTRIBUTE_ERROR_PREFIX),
        );
        value
    }
}
