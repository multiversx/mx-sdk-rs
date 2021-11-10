use crate::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedVecItem},
};

use super::{EsdtTokenType, TokenIdentifier};

use elrond_codec::elrond_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode};

use crate as elrond_wasm; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Debug)]
pub struct EsdtTokenPayment<M: ManagedTypeApi> {
    pub token_type: EsdtTokenType,
    pub token_identifier: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> EsdtTokenPayment<M> {
    pub fn no_payment(api: M) -> Self {
        EsdtTokenPayment {
            token_type: EsdtTokenType::Invalid,
            token_identifier: TokenIdentifier::egld(api.clone()),
            token_nonce: 0,
            amount: BigUint::zero(api),
        }
    }

    pub fn new(token_identifier: TokenIdentifier<M>, token_nonce: u64, amount: BigUint<M>) -> Self {
        let token_type = if amount != 0 && token_identifier.is_valid_esdt_identifier() {
            if token_nonce == 0 {
                EsdtTokenType::Fungible
            } else if amount == 1u64 {
                EsdtTokenType::NonFungible
            } else {
                EsdtTokenType::SemiFungible
            }
        } else {
            EsdtTokenType::Invalid
        };

        EsdtTokenPayment {
            token_type,
            token_identifier,
            token_nonce,
            amount,
        }
    }
}

impl<M: ManagedTypeApi> ManagedVecItem<M> for EsdtTokenPayment<M> {
    const PAYLOAD_SIZE: usize = 16;
    const SKIPS_RESERIALIZATION: bool = true;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(api: M, mut reader: Reader) -> Self {
        let mut arr: [u8; 16] = [0u8; 16];
        reader(&mut arr[..]);
        let mut index = 0;

        let token_identifier = ManagedVecItem::<M>::from_byte_reader(api.clone(), |bytes| {
            let size = TokenIdentifier::<M>::PAYLOAD_SIZE;
            bytes.copy_from_slice(&arr[index..index + size]);
            index += size
        });

        let token_nonce = ManagedVecItem::<M>::from_byte_reader(api.clone(), |bytes| {
            let size = <u64 as ManagedVecItem<M>>::PAYLOAD_SIZE;
            bytes.copy_from_slice(&arr[index..index + size]);
            index += size
        });

        let amount = ManagedVecItem::<M>::from_byte_reader(api.clone(), |bytes| {
            let size = BigUint::<M>::PAYLOAD_SIZE;
            bytes.copy_from_slice(&arr[index..index + size]);
            index += size
        });

        let token_type = if token_nonce > 0 {
            EsdtTokenType::SemiFungible
        } else {
            EsdtTokenType::Fungible
        };

        EsdtTokenPayment {
            token_type,
            token_identifier,
            token_nonce,
            amount,
        }
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
        let mut arr: [u8; 16] = [0u8; 16];
        let mut index = 0;

        ManagedVecItem::<M>::to_byte_writer(&self.token_identifier, |bytes| {
            let size = TokenIdentifier::<M>::PAYLOAD_SIZE;
            arr[index..index + size].copy_from_slice(bytes);
            index += size
        });

        ManagedVecItem::<M>::to_byte_writer(&self.token_nonce, |bytes| {
            let size = <u64 as ManagedVecItem<M>>::PAYLOAD_SIZE;
            arr[index..index + size].copy_from_slice(bytes);
            index += size
        });

        ManagedVecItem::<M>::to_byte_writer(&self.amount, |bytes| {
            let size = BigUint::<M>::PAYLOAD_SIZE;
            arr[index..index + size].copy_from_slice(bytes);
            index += size
        });

        writer(&arr[..])
    }
}
