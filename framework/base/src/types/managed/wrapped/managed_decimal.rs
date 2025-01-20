mod decimals;
mod managed_decimal_cmp;
mod managed_decimal_cmp_signed;
mod managed_decimal_logarithm;
mod managed_decimal_macros;
mod managed_decimal_op_add;
mod managed_decimal_op_add_signed;
mod managed_decimal_op_div;
mod managed_decimal_op_div_signed;
mod managed_decimal_op_mul;
mod managed_decimal_op_mul_signed;
mod managed_decimal_op_sub;
mod managed_decimal_op_sub_signed;
mod managed_decimal_signed;

pub use decimals::{ConstDecimals, Decimals, NumDecimals};
pub use managed_decimal_signed::ManagedDecimalSigned;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::ManagedTypeApi,
    formatter::{FormatBuffer, FormatByteReceiver, SCDisplay},
    types::BigUint,
};

use alloc::string::ToString;
use multiversx_sc_codec::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

use core::{cmp::Ordering, ops::Deref};

use super::{
    managed_vec_item_read_from_payload_index, managed_vec_item_save_to_payload_index,
    ManagedBufferCachedBuilder, ManagedRef, ManagedVecItem, ManagedVecItemPayloadBuffer,
    ManagedVecRef,
};

/// Fixed-point decimal numbers that accept either a constant or variable number of decimals.
///
/// Negative numbers are not allowed. It is especially designed for denominated token amounts.
/// If negative numbers are needed, use `ManagedDecimalSigned` instead.
#[derive(Clone)]
pub struct ManagedDecimal<M: ManagedTypeApi, D: Decimals> {
    pub(crate) data: BigUint<M>,
    pub(crate) decimals: D,
}

impl<M: ManagedTypeApi, D: Decimals> ManagedDecimal<M, D> {
    pub fn trunc(&self) -> BigUint<M> {
        &self.data / self.decimals.scaling_factor().deref()
    }

    pub fn into_raw_units(&self) -> &BigUint<M> {
        &self.data
    }

    pub fn from_raw_units(data: BigUint<M>, decimals: D) -> Self {
        ManagedDecimal { data, decimals }
    }

    pub fn scale(&self) -> usize {
        self.decimals.num_decimals()
    }

    pub fn scaling_factor(&self) -> ManagedRef<'static, M, BigUint<M>> {
        self.decimals.scaling_factor()
    }

    pub(crate) fn rescale_data(&self, scale_to_num_decimals: NumDecimals) -> BigUint<M> {
        let from_num_decimals = self.decimals.num_decimals();

        match from_num_decimals.cmp(&scale_to_num_decimals) {
            Ordering::Less => {
                let delta_decimals = scale_to_num_decimals - from_num_decimals;
                let scaling_factor: &BigUint<M> = &delta_decimals.scaling_factor();
                &self.data * scaling_factor
            },
            Ordering::Equal => self.data.clone(),
            Ordering::Greater => {
                let delta_decimals = from_num_decimals - scale_to_num_decimals;
                let scaling_factor: &BigUint<M> = &delta_decimals.scaling_factor();
                &self.data / scaling_factor
            },
        }
    }

    pub fn rescale<T: Decimals>(&self, scale_to: T) -> ManagedDecimal<M, T>
    where
        M: ManagedTypeApi,
    {
        let scale_to_num_decimals = scale_to.num_decimals();
        ManagedDecimal::from_raw_units(self.rescale_data(scale_to_num_decimals), scale_to)
    }

    pub fn into_signed(self) -> ManagedDecimalSigned<M, D> {
        ManagedDecimalSigned {
            data: self.data.into_big_int(),
            decimals: self.decimals,
        }
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> From<BigUint<M>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    fn from(mut value: BigUint<M>) -> Self {
        let decimals = ConstDecimals;
        value *= decimals.scaling_factor().deref();
        ManagedDecimal {
            data: value,
            decimals,
        }
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> ManagedDecimal<M, ConstDecimals<DECIMALS>> {
    pub fn const_decimals_from_raw(data: BigUint<M>) -> Self {
        ManagedDecimal {
            data,
            decimals: ConstDecimals,
        }
    }

    /// Converts from constant (compile-time) number of decimals to a variable number of decimals.
    pub fn into_var_decimals(self) -> ManagedDecimal<M, NumDecimals> {
        ManagedDecimal {
            data: self.data,
            decimals: DECIMALS,
        }
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals>
    From<ManagedDecimal<M, ConstDecimals<DECIMALS>>> for ManagedDecimal<M, NumDecimals>
{
    fn from(value: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self {
        value.into_var_decimals()
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for ManagedDecimal<M, NumDecimals> {
    type PAYLOAD = ManagedVecItemPayloadBuffer<8>; // 4 bigUint + 4 usize

    const SKIPS_RESERIALIZATION: bool = false;

    type Ref<'a> = ManagedVecRef<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let mut index = 0;
        unsafe {
            Self {
                data: managed_vec_item_read_from_payload_index(payload, &mut index),
                decimals: managed_vec_item_read_from_payload_index(payload, &mut index),
            }
        }
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        ManagedVecRef::new(Self::read_from_payload(payload))
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        let mut index = 0;
        unsafe {
            managed_vec_item_save_to_payload_index(self.data, payload, &mut index);
            managed_vec_item_save_to_payload_index(self.decimals, payload, &mut index);
        }
    }
}

// impl<M: ManagedTypeApi, const N: NumDecimals> ManagedVecItem
//     for ManagedDecimal<M, ConstDecimals<N>>
// {
//     type PAYLOAD = ManagedVecItemPayloadBuffer<8>; // 4 bigUint + 4 usize

//     const SKIPS_RESERIALIZATION: bool = false;

//     type Ref<'a> = ManagedVecRef<'a, Self>;

//     fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
//         let mut index = 0;
//         unsafe {
//             Self {
//                 data: managed_vec_item_read_from_payload_index::<
//                     BigUint<M>,
//                     ManagedVecItemPayloadBuffer<8>,
//                 >(payload, &mut index),
//                 decimals: managed_vec_item_read_from_payload_index::<
//                     ConstDecimals<N>,
//                     ManagedVecItemPayloadBuffer<8>,
//                 >(payload, &mut index),
//             }
//         }
//     }

//     unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
//         ManagedVecRef::new(Self::read_from_payload(payload))
//     }

//     fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
//         let mut index = 0;
//         unsafe {
//             managed_vec_item_save_to_payload_index(self.data, payload, &mut index);
//             managed_vec_item_save_to_payload_index(
//                 self.decimals.num_decimals(),
//                 payload,
//                 &mut index,
//             );
//         }
//     }
// }

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> TopEncode
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.data.top_encode_or_handle_err(output, h)
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> TopDecode
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(ManagedDecimal::const_decimals_from_raw(
            BigUint::top_decode_or_handle_err(input, h)?,
        ))
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> NestedEncode
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        NestedEncode::dep_encode_or_handle_err(&self.data, dest, h)?;

        Result::Ok(())
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> NestedDecode
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Result::Ok(ManagedDecimal::const_decimals_from_raw(
            <BigUint<M> as NestedDecode>::dep_decode_or_handle_err(input, h)?,
        ))
    }
}

impl<M: ManagedTypeApi> NestedEncode for ManagedDecimal<M, NumDecimals> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        NestedEncode::dep_encode_or_handle_err(&self.data, dest, h)?;
        NestedEncode::dep_encode_or_handle_err(&self.decimals, dest, h)?;

        Result::Ok(())
    }
}

impl<M: ManagedTypeApi> TopEncode for ManagedDecimal<M, NumDecimals> {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        let mut buffer = output.start_nested_encode();
        let dest = &mut buffer;
        NestedEncode::dep_encode_or_handle_err(&self.data, dest, h)?;
        NestedEncode::dep_encode_or_handle_err(&self.decimals, dest, h)?;

        output.finalize_nested_encode(buffer);
        Result::Ok(())
    }
}

impl<M: ManagedTypeApi> NestedDecode for ManagedDecimal<M, NumDecimals> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Result::Ok(ManagedDecimal::from_raw_units(
            <BigUint<M> as NestedDecode>::dep_decode_or_handle_err(input, h)?,
            <NumDecimals as NestedDecode>::dep_decode_or_handle_err(input, h)?,
        ))
    }
}

impl<M: ManagedTypeApi> TopDecode for ManagedDecimal<M, NumDecimals> {
    fn top_decode_or_handle_err<I, H>(top_input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let mut nested_buffer = top_input.into_nested_buffer();
        let result = ManagedDecimal::from_raw_units(
            <BigUint<M> as NestedDecode>::dep_decode_or_handle_err(&mut nested_buffer, h)?,
            <NumDecimals as NestedDecode>::dep_decode_or_handle_err(&mut nested_buffer, h)?,
        );
        if !NestedDecodeInput::is_depleted(&nested_buffer) {
            return Result::Err(h.handle_error(DecodeError::INPUT_TOO_LONG));
        }
        Result::Ok(result)
    }
}

impl<M: ManagedTypeApi> TypeAbiFrom<Self> for ManagedDecimal<M, NumDecimals> {}

impl<M: ManagedTypeApi> TypeAbi for ManagedDecimal<M, NumDecimals> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from("ManagedDecimal<usize>")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("ManagedDecimal<$API, usize>")
    }

    fn is_variadic() -> bool {
        false
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> TypeAbiFrom<Self>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> TypeAbi
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from(alloc::format!("ManagedDecimal<{}>", DECIMALS))
    }

    fn type_name_rust() -> TypeName {
        TypeName::from(alloc::format!(
            "ManagedDecimal<$API, ConstDecimals<{}>>",
            DECIMALS
        ))
    }

    fn is_variadic() -> bool {
        false
    }
}
impl<M: ManagedTypeApi, D: Decimals> SCDisplay for ManagedDecimal<M, D> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        managed_decimal_signed::managed_decimal_fmt(
            &self.data.value,
            self.decimals.num_decimals(),
            f,
        );
    }
}

impl<M: ManagedTypeApi, D: Decimals> core::fmt::Display for ManagedDecimal<M, D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut result = ManagedBufferCachedBuilder::<M>::new_from_slice(&[]);
        result.append_display(self);
        core::fmt::Display::fmt(&result.into_managed_buffer(), f)
    }
}

impl<M: ManagedTypeApi, D: Decimals> core::fmt::Debug for ManagedDecimal<M, D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ManagedDecimal")
            .field("handle", &self.data.value.handle.clone())
            .field("number", &self.to_string())
            .finish()
    }
}
