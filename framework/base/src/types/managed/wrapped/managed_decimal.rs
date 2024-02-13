use crate::{
    abi::{TypeAbi, TypeName},
    api::{const_handles, use_raw_handle, BigIntApiImpl, ManagedTypeApi, StaticVarApiImpl},
    types::{BigFloat, BigInt, BigUint},
};

use multiversx_sc_codec::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

use core::{
    cmp::Ordering,
    ops::{Add, Deref, Div, Mul, Sub},
};

use super::ManagedRef;

fn scaling_factor<M: ManagedTypeApi>(
    num_decimals: NumDecimals,
) -> ManagedRef<'static, M, BigUint<M>> {
    let handle: M::BigIntHandle =
        use_raw_handle(const_handles::get_scaling_factor_handle(num_decimals));

    if !M::static_var_api_impl().is_scaling_factor_cached(num_decimals) {
        let temp1: M::BigIntHandle = use_raw_handle(const_handles::BIG_INT_TEMPORARY_1);
        let temp2: M::BigIntHandle = use_raw_handle(const_handles::BIG_INT_TEMPORARY_2);
        let api = M::managed_type_impl();
        api.bi_set_int64(temp1.clone(), 10);
        api.bi_set_int64(temp2.clone(), num_decimals as i64);
        api.bi_pow(handle.clone(), temp1, temp2);
    }

    unsafe { ManagedRef::<'static, M, BigUint<M>>::wrap_handle(handle) }
}

pub trait Decimals {
    fn num_decimals(&self) -> NumDecimals;

    fn scaling_factor<M: ManagedTypeApi>(&self) -> ManagedRef<'static, M, BigUint<M>> {
        scaling_factor(self.num_decimals())
    }
}

impl Decimals for NumDecimals {
    fn num_decimals(&self) -> NumDecimals {
        *self
    }
}

pub type NumDecimals = usize;

#[derive(Clone, Debug)]
pub struct ConstDecimals<const DECIMALS: NumDecimals>;

impl<const DECIMALS: NumDecimals> Decimals for ConstDecimals<DECIMALS> {
    fn num_decimals(&self) -> NumDecimals {
        DECIMALS
    }

    fn scaling_factor<M: ManagedTypeApi>(&self) -> ManagedRef<'static, M, BigUint<M>> {
        scaling_factor(self.num_decimals())
    }
}

#[derive(Debug, Clone)]
pub struct ManagedDecimal<M: ManagedTypeApi, D: Decimals> {
    data: BigUint<M>,
    decimals: D,
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

    pub fn rescale<T: Decimals>(self, scale_to: T) -> ManagedDecimal<M, T>
    where
        M: ManagedTypeApi,
    {
        let from_num_decimals = self.decimals.num_decimals();
        let scale_to_num_decimals = scale_to.num_decimals();

        match from_num_decimals.cmp(&scale_to_num_decimals) {
            Ordering::Less => {
                let delta_decimals = scale_to_num_decimals - from_num_decimals;
                let scaling_factor: &BigUint<M> = &delta_decimals.scaling_factor();
                ManagedDecimal::from_raw_units(&self.data * scaling_factor, scale_to)
            },
            Ordering::Equal => ManagedDecimal::from_raw_units(self.data, scale_to),
            Ordering::Greater => {
                let delta_decimals = from_num_decimals - scale_to_num_decimals;
                let scaling_factor: &BigUint<M> = &delta_decimals.scaling_factor();
                ManagedDecimal::from_raw_units(&self.data * scaling_factor, scale_to)
            },
        }
    }

    pub fn to_big_float(&self) -> BigFloat<M> {
        BigFloat::from_big_uint(&self.data)
    }

    pub fn to_big_int(self) -> BigInt<M> {
        BigInt::from_biguint(crate::types::Sign::Plus, self.data)
    }

    pub fn from_big_int<T: Decimals>(big_int: BigInt<M>, num_decimals: T) -> ManagedDecimal<M, T> {
        ManagedDecimal::from_raw_units(
            big_int
                .into_big_uint()
                .unwrap_or_sc_panic("failed to cast BigInt to BigUint"),
            num_decimals,
        )
    }

    pub fn from_big_float<T: Decimals>(
        big_float: BigFloat<M>,
        num_decimals: T,
    ) -> ManagedDecimal<M, T> {
        let scaling_factor: &BigUint<M> = &num_decimals.scaling_factor();
        let magnitude = big_float.magnitude();

        let scaled = &BigFloat::from(scaling_factor) * &magnitude;
        let fixed_big_int = scaled.trunc();

        ManagedDecimal::<M, T>::from_big_int(fixed_big_int, num_decimals)
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> ManagedDecimal<M, ConstDecimals<DECIMALS>> {
    pub fn const_decimals_from_raw(data: BigUint<M>) -> Self {
        ManagedDecimal {
            data,
            decimals: ConstDecimals,
        }
    }
}

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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> From<BigUint<M>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    fn from(value: BigUint<M>) -> Self {
        let decimals = ConstDecimals;
        ManagedDecimal {
            data: &value * decimals.scaling_factor().deref(),
            decimals,
        }
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> Add<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Output = Self;

    fn add(self, other: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        ManagedDecimal::const_decimals_from_raw(self.data + other.data)
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> Sub<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Output = Self;

    fn sub(self, other: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        ManagedDecimal::const_decimals_from_raw(self.data - other.data)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: ManagedTypeApi, const DECIMALS: NumDecimals, const OTHER_DECIMALS: NumDecimals>
    Mul<ManagedDecimal<M, ConstDecimals<OTHER_DECIMALS>>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
where
    [(); DECIMALS + OTHER_DECIMALS]:,
{
    type Output = ManagedDecimal<M, ConstDecimals<{ DECIMALS + OTHER_DECIMALS }>>;

    fn mul(self, other: ManagedDecimal<M, ConstDecimals<OTHER_DECIMALS>>) -> Self::Output {
        ManagedDecimal::const_decimals_from_raw(self.data * other.data)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: ManagedTypeApi, const DECIMALS: NumDecimals, const OTHER_DECIMALS: NumDecimals>
    Div<ManagedDecimal<M, ConstDecimals<OTHER_DECIMALS>>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
where
    [(); DECIMALS - OTHER_DECIMALS]:,
{
    type Output = ManagedDecimal<M, ConstDecimals<{ DECIMALS - OTHER_DECIMALS }>>;

    fn div(self, other: ManagedDecimal<M, ConstDecimals<OTHER_DECIMALS>>) -> Self::Output {
        ManagedDecimal::const_decimals_from_raw(self.data / other.data)
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> PartialEq<ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
{
    fn eq(&self, other: &ManagedDecimal<M, D2>) -> bool {
        match self
            .decimals
            .num_decimals()
            .cmp(&other.decimals.num_decimals())
        {
            Ordering::Less => {
                let diff_decimals = other.decimals.num_decimals() - self.decimals.num_decimals();
                let scaling_factor: &BigUint<M> = &diff_decimals.scaling_factor();
                &self.data * scaling_factor == other.data
            },
            Ordering::Equal => self.data == other.data,
            Ordering::Greater => {
                let diff_decimals = self.decimals.num_decimals() - other.decimals.num_decimals();
                let scaling_factor: &BigUint<M> = &diff_decimals.scaling_factor();
                &other.data * scaling_factor == self.data
            },
        }
    }
}

impl<M: ManagedTypeApi> TypeAbi for ManagedDecimal<M, NumDecimals> {
    fn type_name() -> TypeName {
        TypeName::from("ManagedDecimal<usize>")
    }

    fn is_variadic() -> bool {
        false
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> TypeAbi
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    fn type_name() -> TypeName {
        TypeName::from(alloc::format!("ManagedDecimal<{}>", DECIMALS))
    }

    fn is_variadic() -> bool {
        false
    }
}
