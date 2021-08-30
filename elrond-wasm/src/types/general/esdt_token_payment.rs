use crate::{
    abi::TypeAbi,
    api::{Handle, ManagedTypeApi},
    types::{BigUint, ManagedBuffer, ManagedType, ManagedVecItem},
};
use alloc::string::String;
use elrond_codec::*;

use super::{EsdtTokenType, TokenIdentifier};

#[derive(Clone)]
pub struct EsdtTokenPayment<M: ManagedTypeApi> {
    pub token_type: EsdtTokenType,
    pub token_name: TokenIdentifier,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

#[allow(clippy::redundant_clone)]
impl<M: ManagedTypeApi> NestedEncode for EsdtTokenPayment<M> {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.token_type.dep_encode(dest)?;
        self.token_name.dep_encode(dest)?;
        self.token_nonce.dep_encode(dest)?;
        self.amount.dep_encode(dest)?;

        Ok(())
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.token_type.dep_encode_or_exit(dest, c.clone(), exit);
        self.token_name.dep_encode_or_exit(dest, c.clone(), exit);
        self.token_nonce.dep_encode_or_exit(dest, c.clone(), exit);
        self.amount.dep_encode_or_exit(dest, c.clone(), exit);
    }
}

impl<M: ManagedTypeApi> TopEncode for EsdtTokenPayment<M> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        top_encode_from_nested(self, output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        top_encode_from_nested_or_exit(self, output, c, exit);
    }
}

#[allow(clippy::redundant_clone)]
impl<M: ManagedTypeApi> NestedDecode for EsdtTokenPayment<M> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        let token_type = EsdtTokenType::dep_decode(input)?;
        let token_name = TokenIdentifier::dep_decode(input)?;
        let token_nonce = u64::dep_decode(input)?;
        let amount = BigUint::dep_decode(input)?;

        Ok(Self {
            token_type,
            token_name,
            token_nonce,
            amount,
        })
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        let token_type = EsdtTokenType::dep_decode_or_exit(input, c.clone(), exit);
        let token_name = TokenIdentifier::dep_decode_or_exit(input, c.clone(), exit);
        let token_nonce = u64::dep_decode_or_exit(input, c.clone(), exit);
        let amount = BigUint::dep_decode_or_exit(input, c.clone(), exit);

        Self {
            token_type,
            token_name,
            token_nonce,
            amount,
        }
    }
}

impl<M: ManagedTypeApi> TopDecode for EsdtTokenPayment<M> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        top_decode_from_nested(input)
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        top_decode_from_nested_or_exit(input, c, exit)
    }
}

impl<M: ManagedTypeApi> TypeAbi for EsdtTokenPayment<M> {
    fn type_name() -> String {
        "EsdtTokenPayment".into()
    }
}

impl<M: ManagedTypeApi> EsdtTokenPayment<M> {
    pub fn no_payment(api: M) -> Self {
        EsdtTokenPayment {
            token_type: EsdtTokenType::Invalid,
            token_name: TokenIdentifier::egld(),
            token_nonce: 0,
            amount: BigUint::zero(api),
        }
    }

    pub fn from(token_name: TokenIdentifier, token_nonce: u64, amount: BigUint<M>) -> Self {
        let token_type = if amount != 0 && token_name.is_valid_esdt_identifier() {
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
            token_name,
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
        let token_name_buf = ManagedBuffer::from_raw_handle(api.clone(), token_id_handle as Handle);

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
            token_name: TokenIdentifier::from(token_name_buf.to_boxed_bytes()),
            token_nonce,
            amount,
        }
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
        let mut arr: [u8; 16] = [0u8; 16];

        let api = self.amount.type_manager();
        let token_name_buf = ManagedBuffer::new_from_bytes(api, self.token_name.as_slice());

        let token_id_handle_raw = token_name_buf.get_raw_handle().to_be_bytes();
        arr[0..4].copy_from_slice(&token_id_handle_raw[..]);

        let nonce_raw = self.token_nonce.to_be_bytes();
        arr[4..12].copy_from_slice(&nonce_raw[..]);

        let amount_handle_raw = self.amount.get_raw_handle().to_be_bytes();
        arr[12..16].copy_from_slice(&amount_handle_raw[..]);

        writer(&arr[..])
    }
}
