use crate::{
    api::ManagedTypeApi,
    types::{BigUint, EsdtTokenPaymentMultiValue, EsdtTokenType, ManagedVecItem, TokenIdentifier},
};

use crate as elrond_wasm; // needed by the TypeAbi generated code

#[derive(Clone, PartialEq, Debug)]
pub struct EsdtTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> EsdtTokenPayment<M> {
    pub fn no_payment() -> Self {
        EsdtTokenPayment {
            token_identifier: TokenIdentifier::empty(),
            token_nonce: 0,
            amount: BigUint::zero(),
        }
    }

    pub fn new(token_identifier: TokenIdentifier<M>, token_nonce: u64, amount: BigUint<M>) -> Self {
        EsdtTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    pub fn token_type(&self) -> EsdtTokenType {
        if self.amount != 0 {
            if self.token_nonce == 0 {
                EsdtTokenType::Fungible
            } else if self.amount == 1u64 {
                EsdtTokenType::NonFungible
            } else {
                EsdtTokenType::SemiFungible
            }
        } else {
            EsdtTokenType::Invalid
        }
    }

    #[inline]
    pub fn into_tuple(self) -> (TokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }

    #[inline]
    pub fn into_multi_value(self) -> EsdtTokenPaymentMultiValue<M> {
        self.into()
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
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
        let mut arr: [u8; 16] = [0u8; 16];
        reader(&mut arr[..]);
        let mut index = 0;

        let token_identifier = managed_vec_item_from_slice(&arr, &mut index);
        let token_nonce = managed_vec_item_from_slice(&arr, &mut index);
        let amount = managed_vec_item_from_slice(&arr, &mut index);

        EsdtTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
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

// Methods below generated from older code derives, in order to preserve the old encoding.
// TODO: see if we can change the encoding and revert to standard derives.

impl<M: ManagedTypeApi> elrond_codec::TopDecode for EsdtTokenPayment<M> {
    fn top_decode_or_handle_err<I, H>(
        top_input: I,
        h: H,
    ) -> core::result::Result<Self, H::HandledErr>
    where
        I: elrond_codec::TopDecodeInput,
        H: elrond_codec::DecodeErrorHandler,
    {
        let mut nested_buffer = top_input.into_nested_buffer();
        let _: EsdtTokenType =
            <EsdtTokenType as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                &mut nested_buffer,
                h,
            )?;
        let result = EsdtTokenPayment {
            token_identifier:
                <TokenIdentifier<M> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                    &mut nested_buffer,
                    h,
                )?,
            token_nonce: <u64 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                &mut nested_buffer,
                h,
            )?,
            amount: <BigUint<M> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                &mut nested_buffer,
                h,
            )?,
        };
        if !elrond_codec::NestedDecodeInput::is_depleted(&nested_buffer) {
            return core::result::Result::Err(
                h.handle_error(elrond_codec::DecodeError::INPUT_TOO_LONG),
            );
        }
        core::result::Result::Ok(result)
    }
}

impl<M: ManagedTypeApi> elrond_codec::TopEncode for EsdtTokenPayment<M> {
    fn top_encode_or_handle_err<O, H>(
        &self,
        output: O,
        h: H,
    ) -> core::result::Result<(), H::HandledErr>
    where
        O: elrond_codec::TopEncodeOutput,
        H: elrond_codec::EncodeErrorHandler,
    {
        let mut buffer = output.start_nested_encode();
        let dest = &mut buffer;
        elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.token_type(), dest, h)?;
        elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.token_identifier, dest, h)?;
        elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.token_nonce, dest, h)?;
        elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.amount, dest, h)?;
        output.finalize_nested_encode(buffer);
        core::result::Result::Ok(())
    }
}

impl<M: ManagedTypeApi> elrond_codec::NestedDecode for EsdtTokenPayment<M> {
    fn dep_decode_or_handle_err<I, H>(
        input: &mut I,
        h: H,
    ) -> core::result::Result<Self, H::HandledErr>
    where
        I: elrond_codec::NestedDecodeInput,
        H: elrond_codec::DecodeErrorHandler,
    {
        let _: EsdtTokenType =
            <EsdtTokenType as elrond_codec::NestedDecode>::dep_decode_or_handle_err(input, h)?;
        core::result::Result::Ok(EsdtTokenPayment {
            token_identifier:
                <TokenIdentifier<M> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                    input, h,
                )?,
            token_nonce: <u64 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(input, h)?,
            amount: <BigUint<M> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(input, h)?,
        })
    }
}

impl<M: ManagedTypeApi> elrond_codec::NestedEncode for EsdtTokenPayment<M> {
    fn dep_encode_or_handle_err<O, H>(
        &self,
        dest: &mut O,
        h: H,
    ) -> core::result::Result<(), H::HandledErr>
    where
        O: elrond_codec::NestedEncodeOutput,
        H: elrond_codec::EncodeErrorHandler,
    {
        elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.token_type(), dest, h)?;
        elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.token_identifier, dest, h)?;
        elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.token_nonce, dest, h)?;
        elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.amount, dest, h)?;
        core::result::Result::Ok(())
    }
}

impl<M: ManagedTypeApi> elrond_wasm::abi::TypeAbi for EsdtTokenPayment<M> {
    fn type_name() -> elrond_wasm::abi::TypeName {
        "EsdtTokenPayment".into()
    }

    #[allow(clippy::vec_init_then_push)]
    fn provide_type_descriptions<TDC: elrond_wasm::abi::TypeDescriptionContainer>(
        accumulator: &mut TDC,
    ) {
        let type_name = Self::type_name();
        if !accumulator.contains_type(&type_name) {
            accumulator.reserve_type_name(type_name.clone());
            let mut field_descriptions = elrond_wasm::types::heap::Vec::new();
            field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                docs: &[],
                name: "token_type",
                field_type: <EsdtTokenType>::type_name(),
            });
            <EsdtTokenType>::provide_type_descriptions(accumulator);
            field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                docs: &[],
                name: "token_identifier",
                field_type: <TokenIdentifier<M>>::type_name(),
            });
            <TokenIdentifier<M>>::provide_type_descriptions(accumulator);
            field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                docs: &[],
                name: "token_nonce",
                field_type: <u64>::type_name(),
            });
            <u64>::provide_type_descriptions(accumulator);
            field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                docs: &[],
                name: "amount",
                field_type: <BigUint<M>>::type_name(),
            });
            <BigUint<M>>::provide_type_descriptions(accumulator);
            accumulator.insert(
                type_name.clone(),
                elrond_wasm::abi::TypeDescription {
                    docs: &[],
                    name: type_name,
                    contents: elrond_wasm::abi::TypeContents::Struct(field_descriptions),
                },
            );
        }
    }
}
