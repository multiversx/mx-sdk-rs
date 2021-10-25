use crate::{
    api::{Handle, ManagedTypeApi},
    types::{BigUint, ManagedBuffer, ManagedType, ManagedVecItem},
};

use super::{EsdtTokenType, TokenIdentifier};

use elrond_codec::elrond_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode};

use crate as elrond_wasm; // needed by the TypeAbi generated code
use elrond_wasm_derive::TypeAbi;

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

    pub fn from(
        token_identifier: TokenIdentifier<M>,
        token_nonce: u64,
        amount: BigUint<M>,
    ) -> Self {
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
    const NEEDS_RESERIALIZATION: bool = true;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(api: M, mut reader: Reader) -> Self {
        let mut arr: [u8; 16] = [0u8; 16];
        reader(&mut arr[..]);

        let mut token_id_handle_raw = [0u8; 4];
        token_id_handle_raw.copy_from_slice(&arr[0..4]);
        let token_id_handle = u32::from_be_bytes(token_id_handle_raw);
        let token_identifier_buf =
            ManagedBuffer::from_raw_handle(api.clone(), token_id_handle as Handle);

        let mut nonce_raw = [0u8; 8];
        nonce_raw.copy_from_slice(&arr[4..12]);
        let token_nonce = u64::from_be_bytes(nonce_raw);

        let mut amount_handle_raw = [0u8; 4];
        amount_handle_raw.copy_from_slice(&arr[12..16]);
        let amount_handle = u32::from_be_bytes(amount_handle_raw);
        let amount = BigUint::from_raw_handle(api, amount_handle as Handle);

        let token_type = if token_nonce > 0 {
            EsdtTokenType::SemiFungible
        } else {
            EsdtTokenType::Fungible
        };

        EsdtTokenPayment {
            token_type,
            token_identifier: TokenIdentifier::from(token_identifier_buf),
            token_nonce,
            amount,
        }
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
        let mut arr: [u8; 16] = [0u8; 16];

        let token_id_handle_raw = self
            .token_identifier
            .as_managed_buffer()
            .get_raw_handle()
            .to_be_bytes();
        arr[0..4].copy_from_slice(&token_id_handle_raw[..]);

        let nonce_raw = self.token_nonce.to_be_bytes();
        arr[4..12].copy_from_slice(&nonce_raw[..]);

        let amount_handle_raw = self.amount.get_raw_handle().to_be_bytes();
        arr[12..16].copy_from_slice(&amount_handle_raw[..]);

        writer(&arr[..])
    }
}
