use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{
        const_handles, use_raw_handle, BigFloatApiImpl, BigIntApiImpl, HandleConstraints,
        ManagedBufferApiImpl, ManagedTypeApi,
    },
    err_msg,
    formatter::{FormatBuffer, FormatByteReceiver, SCDisplay},
    types::{
        managed_vec_item_read_from_payload_index, managed_vec_item_save_to_payload_index, BigFloat,
        BigInt, BigUint, ManagedVecItem, ManagedVecItemPayloadBuffer, ManagedVecRef, Sign,
    },
};

use alloc::string::ToString;
use generic_array::typenum::{U4, U8};
use multiversx_sc_codec::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

use core::{cmp::Ordering, ops::Deref};

use super::{
    decimals::{ConstDecimals, Decimals, NumDecimals},
    ManagedDecimal,
};
use super::{ManagedBufferCachedBuilder, ManagedRef};

/// Fixed-point decimal numbers that accept either a constant or variable number of decimals.
///
/// Unlike for `ManagedDecimal`, ngative numbers are also allowed.
#[derive(Clone)]
pub struct ManagedDecimalSigned<M: ManagedTypeApi, D: Decimals> {
    pub(crate) data: BigInt<M>,
    pub(crate) decimals: D,
}

impl<M: ManagedTypeApi, D: Decimals> ManagedDecimalSigned<M, D> {
    pub fn trunc(&self) -> BigInt<M> {
        &self.data / self.decimals.scaling_factor().deref()
    }

    pub fn into_raw_units(&self) -> &BigInt<M> {
        &self.data
    }

    pub fn from_raw_units(data: BigInt<M>, decimals: D) -> Self {
        ManagedDecimalSigned { data, decimals }
    }

    pub fn scale(&self) -> usize {
        self.decimals.num_decimals()
    }

    pub fn scaling_factor(&self) -> ManagedRef<'static, M, BigUint<M>> {
        self.decimals.scaling_factor()
    }

    pub(crate) fn rescale_data(&self, scale_to_num_decimals: NumDecimals) -> BigInt<M> {
        let from_num_decimals = self.decimals.num_decimals();

        match from_num_decimals.cmp(&scale_to_num_decimals) {
            Ordering::Less => {
                let delta_decimals = scale_to_num_decimals - from_num_decimals;
                let scaling_factor: &BigUint<M> = &delta_decimals.scaling_factor();
                &self.data * &scaling_factor.value
            },
            Ordering::Equal => self.data.clone(),
            Ordering::Greater => {
                let delta_decimals = from_num_decimals - scale_to_num_decimals;
                let scaling_factor: &BigUint<M> = &delta_decimals.scaling_factor();
                &self.data / &scaling_factor.value
            },
        }
    }

    pub fn rescale<T: Decimals>(&self, scale_to: T) -> ManagedDecimalSigned<M, T> {
        let scale_to_num_decimals = scale_to.num_decimals();
        ManagedDecimalSigned::from_raw_units(self.rescale_data(scale_to_num_decimals), scale_to)
    }

    pub fn into_unsigned_or_fail(self) -> ManagedDecimal<M, D> {
        ManagedDecimal {
            data: self
                .data
                .into_big_uint()
                .unwrap_or_sc_panic(err_msg::UNSIGNED_NEGATIVE),
            decimals: self.decimals,
        }
    }

    pub fn sign(&self) -> Sign {
        self.data.sign()
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals>
    ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    pub fn const_decimals_from_raw(data: BigInt<M>) -> Self {
        ManagedDecimalSigned {
            data,
            decimals: ConstDecimals,
        }
    }

    /// Converts from constant (compile-time) number of decimals to a variable number of decimals.
    pub fn into_var_decimals(self) -> ManagedDecimalSigned<M, NumDecimals> {
        ManagedDecimalSigned {
            data: self.data,
            decimals: DECIMALS,
        }
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> From<BigInt<M>>
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    fn from(mut value: BigInt<M>) -> Self {
        let decimals = ConstDecimals;
        value *= decimals.scaling_factor().as_big_int();
        ManagedDecimalSigned {
            data: value,
            decimals,
        }
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> From<i64>
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    fn from(value: i64) -> Self {
        Self::from(BigInt::from(value))
    }
}

impl<M: ManagedTypeApi, D: Decimals> ManagedDecimalSigned<M, D> {
    pub fn to_big_float(&self) -> BigFloat<M> {
        let result = BigFloat::from_big_int(&self.data);
        let temp_handle: M::BigFloatHandle = use_raw_handle(const_handles::BIG_FLOAT_TEMPORARY);
        let denominator = self.decimals.scaling_factor::<M>();
        M::managed_type_impl().bf_set_bi(temp_handle.clone(), denominator.handle);
        M::managed_type_impl().bf_div(result.handle.clone(), result.handle.clone(), temp_handle);
        result
    }

    pub fn from_big_float<T: Decimals>(
        big_float: &BigFloat<M>,
        num_decimals: T,
    ) -> ManagedDecimalSigned<M, T> {
        let scaling_factor: &BigUint<M> = &num_decimals.scaling_factor();

        let scaled = &BigFloat::from(scaling_factor) * big_float;
        let fixed_big_int = scaled.trunc();

        ManagedDecimalSigned::from_raw_units(fixed_big_int, num_decimals)
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> From<&BigFloat<M>>
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    fn from(value: &BigFloat<M>) -> Self {
        Self::from_big_float(value, ConstDecimals)
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> From<BigFloat<M>>
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    #[inline]
    fn from(value: BigFloat<M>) -> Self {
        Self::from(&value)
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> From<f64>
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    fn from(x: f64) -> Self {
        Self::from(BigFloat::from(x))
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> From<f32>
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    fn from(x: f32) -> Self {
        Self::from(x as f64)
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for ManagedDecimalSigned<M, NumDecimals> {
    type PAYLOAD = ManagedVecItemPayloadBuffer<U8>; // 4 bigInt + 4 usize

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

impl<M: ManagedTypeApi, const N: NumDecimals> ManagedVecItem
    for ManagedDecimalSigned<M, ConstDecimals<N>>
{
    type PAYLOAD = ManagedVecItemPayloadBuffer<U4>; // data only

    const SKIPS_RESERIALIZATION: bool = false;

    type Ref<'a> = ManagedVecRef<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        Self::const_decimals_from_raw(BigInt::read_from_payload(payload))
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        ManagedVecRef::new(Self::read_from_payload(payload))
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        self.data.save_to_payload(payload);
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> TopEncode
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
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
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(ManagedDecimalSigned::const_decimals_from_raw(
            BigInt::top_decode_or_handle_err(input, h)?,
        ))
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> NestedEncode
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
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
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Result::Ok(ManagedDecimalSigned::const_decimals_from_raw(
            <BigInt<M> as NestedDecode>::dep_decode_or_handle_err(input, h)?,
        ))
    }
}

impl<M: ManagedTypeApi> NestedEncode for ManagedDecimalSigned<M, NumDecimals> {
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

impl<M: ManagedTypeApi> TopEncode for ManagedDecimalSigned<M, NumDecimals> {
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

impl<M: ManagedTypeApi> NestedDecode for ManagedDecimalSigned<M, NumDecimals> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Result::Ok(ManagedDecimalSigned::from_raw_units(
            <BigInt<M> as NestedDecode>::dep_decode_or_handle_err(input, h)?,
            <NumDecimals as NestedDecode>::dep_decode_or_handle_err(input, h)?,
        ))
    }
}

impl<M: ManagedTypeApi> TopDecode for ManagedDecimalSigned<M, NumDecimals> {
    fn top_decode_or_handle_err<I, H>(top_input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let mut nested_buffer = top_input.into_nested_buffer();
        let result = ManagedDecimalSigned::from_raw_units(
            <BigInt<M> as NestedDecode>::dep_decode_or_handle_err(&mut nested_buffer, h)?,
            <NumDecimals as NestedDecode>::dep_decode_or_handle_err(&mut nested_buffer, h)?,
        );
        if !NestedDecodeInput::is_depleted(&nested_buffer) {
            return Result::Err(h.handle_error(DecodeError::INPUT_TOO_LONG));
        }
        Result::Ok(result)
    }
}

impl<M: ManagedTypeApi> TypeAbiFrom<Self> for ManagedDecimalSigned<M, NumDecimals> {}

impl<M: ManagedTypeApi> TypeAbi for ManagedDecimalSigned<M, NumDecimals> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from("ManagedDecimalSigned<usize>")
    }

    fn is_variadic() -> bool {
        false
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> TypeAbiFrom<Self>
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> TypeAbi
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from(alloc::format!("ManagedDecimalSigned<{}>", DECIMALS))
    }

    fn type_name_rust() -> TypeName {
        TypeName::from(alloc::format!(
            "ManagedDecimalSigned<$API, ConstDecimals<{}>>",
            DECIMALS
        ))
    }

    fn is_variadic() -> bool {
        false
    }
}

pub(super) fn managed_decimal_fmt<M: ManagedTypeApi, F: FormatByteReceiver>(
    value: &BigInt<M>,
    num_dec: NumDecimals,
    f: &mut F,
) {
    let full_str_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
    M::managed_type_impl().bi_to_string(value.handle.clone(), full_str_handle.clone());
    let len = M::managed_type_impl().mb_len(full_str_handle.clone());

    if len > num_dec {
        let temp_str_handle: M::ManagedBufferHandle =
            use_raw_handle(const_handles::MBUF_TEMPORARY_2);
        let cast_handle = temp_str_handle.clone().cast_or_signal_error::<M, _>();
        let temp_str_ref = unsafe { ManagedRef::wrap_handle(cast_handle) };
        let _ = M::managed_type_impl().mb_copy_slice(
            full_str_handle.clone(),
            0,
            len - num_dec,
            temp_str_handle.clone(),
        );
        f.append_managed_buffer(&temp_str_ref);
        f.append_bytes(b".");
        let _ = M::managed_type_impl().mb_copy_slice(
            full_str_handle.clone(),
            len - num_dec,
            num_dec,
            temp_str_handle.clone(),
        );
        f.append_managed_buffer(&temp_str_ref);
    } else {
        f.append_bytes(b"0.");
        for _ in len..num_dec {
            f.append_bytes(b"0");
        }
        let cast_handle = full_str_handle.clone().cast_or_signal_error::<M, _>();
        let full_str_ref = unsafe { ManagedRef::wrap_handle(cast_handle) };
        f.append_managed_buffer(&full_str_ref);
    }
}

impl<M: ManagedTypeApi, D: Decimals> SCDisplay for ManagedDecimalSigned<M, D> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        managed_decimal_fmt(&self.data, self.decimals.num_decimals(), f);
    }
}

impl<M: ManagedTypeApi, D: Decimals> core::fmt::Display for ManagedDecimalSigned<M, D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut result = ManagedBufferCachedBuilder::<M>::new_from_slice(&[]);
        result.append_display(self);
        core::fmt::Display::fmt(&result.into_managed_buffer(), f)
    }
}

impl<M: ManagedTypeApi, D: Decimals> core::fmt::Debug for ManagedDecimalSigned<M, D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ManagedDecimalSigned")
            .field("handle", &self.data.handle.clone())
            .field("number", &self.to_string())
            .finish()
    }
}
