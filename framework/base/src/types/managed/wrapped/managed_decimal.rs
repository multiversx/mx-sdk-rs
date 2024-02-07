use crate::{
    abi::{TypeAbi, TypeName},
    api::{const_handles, ManagedTypeApi, StaticVarApiImpl},
    types::{BigFloat, BigInt, BigUint},
};

use multiversx_sc_codec::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, Vec,
};
use num_traits::One;

use core::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
    let mut bits = Vec::new();
    for &byte in bytes {
        for i in (0..8).rev() {
            // iterate over each bit in the byte
            bits.push((byte >> i) & 1 == 1);
        }
    }
    bits
}

fn reconstruct<M: ManagedTypeApi>(
    sf_init: [bool; const_handles::SCALING_FACTOR_LENGTH as usize],
) -> BigUint<M> {
    let mut bits = Vec::with_capacity(sf_init.len());
    for (index, init) in sf_init.iter().enumerate() {
        if !init {
            break;
        }
        // get value from handle = start + index;
        let actual_handle = const_handles::SCALING_FACTOR_START + index as i32;
        let bit = M::static_var_api_impl().get_i64_from_handle(actual_handle);
        bits.push(bit.is_one());
    }

    // reconstruct u64 from bits
    let mut result_u64 = 0;
    for bit in &bits {
        result_u64 = (result_u64 << 1) | (*bit as u64);
    }

    BigUint::from(result_u64)
}

fn calc_scaling_factor<M: ManagedTypeApi>(num_decimals: NumDecimals) -> BigUint<M> {
    BigUint::from(10u32).pow(num_decimals as u32)
}

fn scaling_factor<M: ManagedTypeApi>(num_decimals: NumDecimals) -> BigUint<M> {
    let mut sf_init = M::static_var_api_impl().get_scaling_factor_init();
    // not cached
    if !sf_init[0] {
        let scaling_factor = calc_scaling_factor(num_decimals);
        // turn big uint into bits
        let bits = bytes_to_bits(scaling_factor.to_bytes_be().as_slice());
        // cache everything (set handles with bit value, set initialized for each bit)
        for (index, bit) in bits.iter().enumerate() {
            let actual_handle = const_handles::SCALING_FACTOR_START + index as i32;

            // set handle
            M::static_var_api_impl().set_i64_to_handle(actual_handle, i64::from(*bit));

            // set initialized
            sf_init[index] = true;
        }
        // set new sf_init
        M::static_var_api_impl().set_scaling_factor_init(sf_init);

        // return calculated scaling factor
        return scaling_factor;
    }

    // reconstruct number from cache
    reconstruct(sf_init)
}

pub trait Decimals {
    fn num_decimals(&self) -> NumDecimals;

    fn scaling_factor<M: ManagedTypeApi>(&self) -> BigUint<M> {
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

    fn scaling_factor<M: ManagedTypeApi>(&self) -> BigUint<M> {
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
        &self.data / &self.decimals.scaling_factor()
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
                ManagedDecimal::from_raw_units(
                    &self.data * &scaling_factor(delta_decimals),
                    scale_to,
                )
            },
            Ordering::Equal => ManagedDecimal::from_raw_units(self.data, scale_to),
            Ordering::Greater => {
                let delta_decimals = from_num_decimals - scale_to_num_decimals;
                ManagedDecimal::from_raw_units(
                    &self.data * &scaling_factor(delta_decimals),
                    scale_to,
                )
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
        let scaling_factor = num_decimals.scaling_factor();
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
            data: &value * &decimals.scaling_factor(),
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
                &self.data * &scaling_factor(diff_decimals) == other.data
            },
            Ordering::Equal => self.data == other.data,
            Ordering::Greater => {
                let diff_decimals = self.decimals.num_decimals() - other.decimals.num_decimals();
                &other.data * &scaling_factor(diff_decimals) == self.data
            },
        }
    }
}

impl<M: ManagedTypeApi, D: Decimals> TypeAbi for ManagedDecimal<M, D> {
    fn type_name() -> TypeName {
        TypeName::from("ManagedDecimal")
    }

    fn is_variadic() -> bool {
        false
    }
}
