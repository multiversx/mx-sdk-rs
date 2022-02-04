use crate::{
    api::{ErrorApiImpl, ManagedTypeApi},
    types::{BigUint, ManagedAddress, ManagedBuffer, ManagedType, ManagedVec},
};
use elrond_codec::*;

use super::EsdtTokenType;

use elrond_codec::elrond_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode};

use crate as elrond_wasm; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

const DECODE_ATTRIBUTE_ERROR_PREFIX: &[u8] = b"error decoding ESDT attributes: ";

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Debug)]
pub struct EsdtTokenData<M: ManagedTypeApi> {
    pub token_type: EsdtTokenType,
    pub amount: BigUint<M>,
    pub frozen: bool,
    pub hash: ManagedBuffer<M>,
    pub name: ManagedBuffer<M>,
    pub attributes: ManagedBuffer<M>,
    pub creator: ManagedAddress<M>,
    pub royalties: BigUint<M>,
    pub uris: ManagedVec<M, ManagedBuffer<M>>,
}

impl<M: ManagedTypeApi> EsdtTokenData<M> {
    pub fn decode_attributes<T: TopDecode>(&self) -> Result<T, DecodeError> {
        T::top_decode(self.attributes.clone()) // TODO: remove clone
    }

    pub fn decode_attributes_or_exit<T: TopDecode>(&self) -> T {
        self.decode_attributes().unwrap_or_else(|err| {
            let mut message = ManagedBuffer::<M>::new_from_bytes(DECODE_ATTRIBUTE_ERROR_PREFIX);
            message.append_bytes(err.message_bytes());
            M::error_api_impl().signal_error_from_buffer(message.get_raw_handle())
        })
    }
}
