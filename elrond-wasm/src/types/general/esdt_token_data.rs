use crate::{
    abi::TypeAbi,
    api::ManagedTypeApi,
    types::{BigUint, ManagedBytesTopDecodeInput, ManagedType},
};
use alloc::string::String;
use elrond_codec::*;

use super::{Address, BoxedBytes, EsdtTokenType};

pub struct EsdtTokenData<M: ManagedTypeApi> {
    pub token_type: EsdtTokenType,
    pub amount: BigUint<M>,
    pub frozen: bool,
    pub hash: BoxedBytes,
    pub name: BoxedBytes,
    pub attributes: BoxedBytes,
    pub creator: Address,
    pub royalties: BigUint<M>,
    pub uris: Vec<BoxedBytes>,
}

impl<M: ManagedTypeApi> EsdtTokenData<M> {
    fn type_manager(&self) -> M {
        self.amount.type_manager()
    }

    pub fn decode_attributes<T: TopDecode>(&self) -> Result<T, DecodeError> {
        let managed_input =
            ManagedBytesTopDecodeInput::new(self.type_manager(), self.attributes.clone());
        T::top_decode(managed_input)
    }
}

#[allow(clippy::redundant_clone)]
impl<M: ManagedTypeApi> NestedEncode for EsdtTokenData<M> {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.token_type.dep_encode(dest)?;
        self.amount.dep_encode(dest)?;
        self.frozen.dep_encode(dest)?;
        self.hash.dep_encode(dest)?;
        self.name.dep_encode(dest)?;
        self.attributes.dep_encode(dest)?;
        self.creator.dep_encode(dest)?;
        self.royalties.dep_encode(dest)?;
        self.uris.dep_encode(dest)?;

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
        self.amount.dep_encode_or_exit(dest, c.clone(), exit);
        self.frozen.dep_encode_or_exit(dest, c.clone(), exit);
        self.hash.dep_encode_or_exit(dest, c.clone(), exit);
        self.name.dep_encode_or_exit(dest, c.clone(), exit);
        self.attributes.dep_encode_or_exit(dest, c.clone(), exit);
        self.creator.dep_encode_or_exit(dest, c.clone(), exit);
        self.royalties.dep_encode_or_exit(dest, c.clone(), exit);
        self.uris.dep_encode_or_exit(dest, c.clone(), exit);
    }
}

impl<M: ManagedTypeApi> TopEncode for EsdtTokenData<M> {
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
impl<M: ManagedTypeApi> NestedDecode for EsdtTokenData<M> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        let token_type = EsdtTokenType::dep_decode(input)?;
        let amount = BigUint::<M>::dep_decode(input)?;
        let frozen = bool::dep_decode(input)?;
        let hash = BoxedBytes::dep_decode(input)?;
        let name = BoxedBytes::dep_decode(input)?;
        let attributes = BoxedBytes::dep_decode(input)?;
        let creator = Address::dep_decode(input)?;
        let royalties = BigUint::<M>::dep_decode(input)?;
        let uris = Vec::<BoxedBytes>::dep_decode(input)?;

        Ok(Self {
            token_type,
            amount,
            frozen,
            hash,
            name,
            attributes,
            creator,
            royalties,
            uris,
        })
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        let token_type = EsdtTokenType::dep_decode_or_exit(input, c.clone(), exit);
        let amount = BigUint::dep_decode_or_exit(input, c.clone(), exit);
        let frozen = bool::dep_decode_or_exit(input, c.clone(), exit);
        let hash = BoxedBytes::dep_decode_or_exit(input, c.clone(), exit);
        let name = BoxedBytes::dep_decode_or_exit(input, c.clone(), exit);
        let attributes = BoxedBytes::dep_decode_or_exit(input, c.clone(), exit);
        let creator = Address::dep_decode_or_exit(input, c.clone(), exit);
        let royalties = BigUint::dep_decode_or_exit(input, c.clone(), exit);
        let uris = Vec::<BoxedBytes>::dep_decode_or_exit(input, c.clone(), exit);

        Self {
            token_type,
            amount,
            frozen,
            hash,
            name,
            attributes,
            creator,
            royalties,
            uris,
        }
    }
}

impl<M: ManagedTypeApi> TopDecode for EsdtTokenData<M> {
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

impl<M: ManagedTypeApi> TypeAbi for EsdtTokenData<M> {
    fn type_name() -> String {
        "EsdtTokenData".into()
    }
}
