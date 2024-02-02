use crate::{
    abi::{TypeAbi, TypeName},
    api::ManagedTypeApi,
    types::{BigFloat, BigInt, BigUint},
};

use multiversx_sc_codec::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

use num_traits::ToPrimitive;

use core::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

fn scaling_factor<M: ManagedTypeApi>(num_decimals: NumDecimals) -> BigUint<M> {
    // TODO: cache
    BigUint::from(10u32).pow(num_decimals as u32)
}

// Constants for the Random algorithm
const MULTIPLIER: u64 = 6364136223846793005;
const INCREMENT: u64 = 1442695040888963407;
const MODULUS: u64 = (1 << 63) - 1;

// Custom random number generator
struct Random {
    state: u64,
}

impl Random {
    fn new(seed: u64) -> Self {
        Random { state: seed }
    }

    fn next(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT) % MODULUS;
        self.state as f64 / MODULUS as f64
    }
}

// Function to generate random f64 numbers within a range
fn random_float_in_range(min: f64, max: f64, seed: u64) -> f64 {
    let mut rng = Random::new(seed);

    let scaled = rng.next() * (max - min);
    scaled + min
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

    pub fn log<T: Decimals>(self, target_base: BigUint<M>, precision: T) -> ManagedDecimal<M, T> {
        let num_decimals = precision.num_decimals() as u32;
        let number = self.data;

        assert!(
            number >= BigUint::from(1u64) && target_base >= BigUint::from(1u64),
            "wrong input"
        );

        let no_with_prec = number.pow(num_decimals);
        let precise = no_with_prec.log2() / target_base.log2();

        ManagedDecimal::from_raw_units(BigUint::<M>::from(precise), precision)
    }

    pub fn root<T: Decimals>(self, degree: f64, precision: T) -> ManagedDecimal<M, T> {
        let value = self.data.to_u64().unwrap().to_f64().unwrap();
        let mut x_pre = random_float_in_range(0.0, 10.0, 123456u64); //Random seed
        let eps = 1e-3; // Smaller epsilon for accuracy
        let mut del_x = f64::MAX; // Initializing difference between roots

        let mut x_k = 0f64;

        while del_x > eps {
            x_k = ((degree - 1.0) * x_pre + value / x_pre.powf(degree - 1.0)) / degree;
            del_x = (x_k - x_pre).abs();

            x_pre = x_k;
        }

        let rounded = (x_k * precision.num_decimals() as f64)
            .round()
            .to_u64()
            .unwrap();
        ManagedDecimal::from_raw_units(BigUint::from(rounded), precision)
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
