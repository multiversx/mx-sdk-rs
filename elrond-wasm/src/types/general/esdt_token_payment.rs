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
    pub fn no_payment() -> Self {
        EsdtTokenPayment {
            token_type: EsdtTokenType::Invalid,
            token_identifier: TokenIdentifier::egld(),
            token_nonce: 0,
            amount: BigUint::zero(),
        }
    }

    pub fn new(token_identifier: TokenIdentifier<M>, token_nonce: u64, amount: BigUint<M>) -> Self {
        let token_type = if amount != 0 && token_identifier.is_esdt() {
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

fn managed_vec_item_from_slice<T>(arr: &[u8], index: &mut usize) -> T
where
    T: ManagedVecItem,
{
    ManagedVecItem::from_byte_reader(|bytes| {
        let size = T::PAYLOAD_SIZE;
        bytes.copy_from_slice(&arr[*index..*index + size]);
        *index += size;
    })
}

fn managed_vec_item_to_slice<T>(arr: &mut [u8], index: &mut usize, item: &T)
where
    T: ManagedVecItem,
{
    ManagedVecItem::to_byte_writer(item, |bytes| {
        let size = T::PAYLOAD_SIZE;
        arr[*index..*index + size].copy_from_slice(bytes);
        *index += size;
    });
}

impl<M: ManagedTypeApi> ManagedVecItem for EsdtTokenPayment<M> {
    const PAYLOAD_SIZE: usize = 16;
    const SKIPS_RESERIALIZATION: bool = false;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
        // const SELF_PAYLOAD_SIZE: usize = <EsdtTokenPayment<M> as ManagedVecItem>::PAYLOAD_SIZE;
        let mut arr: [u8; 16] = [0u8; 16];
        reader(&mut arr[..]);
        let mut index = 0;

        let token_identifier = managed_vec_item_from_slice(&arr, &mut index);
        let token_nonce = managed_vec_item_from_slice(&arr, &mut index);
        let amount = managed_vec_item_from_slice(&arr, &mut index);

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

        managed_vec_item_to_slice(&mut arr, &mut index, &self.token_identifier);
        managed_vec_item_to_slice(&mut arr, &mut index, &self.token_nonce);
        managed_vec_item_to_slice(&mut arr, &mut index, &self.amount);

        writer(&arr[..])
    }
}
