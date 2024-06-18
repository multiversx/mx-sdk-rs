use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{
        const_handles, use_raw_handle, BigFloatApiImpl, BigIntApiImpl, ManagedTypeApi,
        StaticVarApiImpl,
    },
    types::{BigFloat, BigUint},
};

use multiversx_sc_codec::{
    num_bigint::ToBigUint, DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode,
    NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode,
    TopEncodeOutput,
};

use core::{cmp::Ordering, ops::Deref};

use super::{ManagedRef, MulToPrecision};

fn scaling_factor<M: ManagedTypeApi>(
    num_decimals: NumDecimals,
) -> ManagedRef<'static, M, BigUint<M>> {
    let handle: M::BigIntHandle =
        use_raw_handle(const_handles::get_scaling_factor_handle(num_decimals));

    if !M::static_var_api_impl().is_scaling_factor_cached(num_decimals) {
        cache_scaling_factor::<M>(handle.clone(), num_decimals);
        M::static_var_api_impl().set_scaling_factor_cached(num_decimals);
    }

    unsafe { ManagedRef::<'static, M, BigUint<M>>::wrap_handle(handle) }
}

fn cache_scaling_factor<M: ManagedTypeApi>(handle: M::BigIntHandle, num_decimals: NumDecimals) {
    let temp1: M::BigIntHandle = use_raw_handle(const_handles::BIG_INT_TEMPORARY_1);
    let temp2: M::BigIntHandle = use_raw_handle(const_handles::BIG_INT_TEMPORARY_2);
    let api = M::managed_type_impl();
    api.bi_set_int64(temp1.clone(), 10);
    api.bi_set_int64(temp2.clone(), num_decimals as i64);
    api.bi_pow(handle, temp1, temp2);
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

    pub fn rescale<T: Decimals>(&self, scale_to: T) -> ManagedDecimal<M, T>
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
            Ordering::Equal => ManagedDecimal::from_raw_units(self.data.clone(), scale_to),
            Ordering::Greater => {
                let delta_decimals = from_num_decimals - scale_to_num_decimals;
                let scaling_factor: &BigUint<M> = &delta_decimals.scaling_factor();
                ManagedDecimal::from_raw_units(&self.data * scaling_factor, scale_to)
            },
        }
    }

    pub fn to_big_float(&self) -> BigFloat<M> {
        let result = BigFloat::from_big_uint(&self.data);
        let temp_handle: M::BigFloatHandle = use_raw_handle(const_handles::BIG_FLOAT_TEMPORARY);
        let denominator = self.decimals.scaling_factor::<M>();
        M::managed_type_impl().bf_set_bi(temp_handle.clone(), denominator.handle);
        M::managed_type_impl().bf_div(result.handle.clone(), result.handle.clone(), temp_handle);
        result
    }

    pub fn from_big_float<T: Decimals>(
        big_float: BigFloat<M>,
        num_decimals: T,
    ) -> ManagedDecimal<M, T> {
        let scaling_factor: &BigUint<M> = &num_decimals.scaling_factor();
        let magnitude = big_float.magnitude();

        let scaled = &BigFloat::from(scaling_factor) * &magnitude;
        let fixed_big_int = scaled.trunc();

        ManagedDecimal::from_raw_units(
            fixed_big_int
                .into_big_uint()
                .unwrap_or_sc_panic("failed to cast BigInt to BigUint"),
            num_decimals,
        )
    }

    pub fn nth_root(self, _root: ManagedDecimal<M, D>, _precision: D) -> ManagedDecimal<M, D> {
        todo!()
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> ManagedDecimal<M, ConstDecimals<DECIMALS>> {
    pub fn const_decimals_from_raw(data: BigUint<M>) -> Self {
        ManagedDecimal {
            data,
            decimals: ConstDecimals,
        }
    }

    // pub fn log(
    //     &self,
    //     target_base: &ManagedDecimal<M, D>,
    //     precision: D,
    // ) -> ManagedDecimal<M, NumDecimals> {
    //     let num_decimals = precision.num_decimals();
    //     // should verify >= 1
    //     let one = ManagedDecimal::from_raw_units(BigUint::from(1u64), 0usize);
    //     one.rescale(self.scale());
    //     assert!(self >= &one, "wrong input for self");
    //     one.rescale(target_base.scale());
    //     assert!(target_base >= &one, "wrong input for target base");

    //     self.ln(&precision)
    //         * ManagedDecimal::from_raw_units(BigUint::from(num_decimals), num_decimals)
    //         / target_base.ln(&precision)
    //     //this should be done with precision
    // }

    pub fn ln<const PREC: usize>(
        self,
        precision: ConstDecimals<PREC>,
    ) -> ManagedDecimal<M, ConstDecimals<PREC>> {
        let num_decimals = self.decimals.num_decimals() as u32;
        // find the highest power of 2 less than or equal to self
        let log2 = self.data.log2() - num_decimals * BigUint::<M>::from(10u64).log2(); // most significant bit for the actual number
        let divisor = 1 << log2;
        let divisor_scaled = BigUint::<M>::from(divisor.to_biguint().unwrap())
            * self.decimals.scaling_factor().clone_value();
        let _normalized = self.data / divisor_scaled; // normalize to [1.0, 2.0]
        let x_dec = ManagedDecimal::<M, ConstDecimals<0>>::const_decimals_from_raw(_normalized);
        let x = x_dec.rescale(precision.clone());

        // approximating polynom to get the result
        let mut result = ManagedDecimal::<M, ConstDecimals<9>>::const_decimals_from_raw(
            BigUint::from(56570851u64), // 0.056570851, 9 decimals˝
        )
        .mul_with_precision(x.clone(), precision.clone());
        result = ManagedDecimal::<M, ConstDecimals<8>>::const_decimals_from_raw(BigUint::from(
            44717955u64, // 0.44717955, 8 decimals
        ))
        .rescale(precision.clone())
            - result;
        result = result.mul_with_precision(x.clone(), precision.clone());
        result -= ManagedDecimal::<M, ConstDecimals<7>>::const_decimals_from_raw(BigUint::from(
            14699568u64, // 1.4699568, 7 decimals
        ))
        .rescale(precision.clone());
        result = result.mul_with_precision(x.clone(), precision.clone());
        result += ManagedDecimal::<M, ConstDecimals<7>>::const_decimals_from_raw(BigUint::from(
            28212026u64, // 2.8212026, 7 decimals
        ))
        .rescale(precision.clone());
        result = result.mul_with_precision(x.clone(), precision.clone());
        result -= ManagedDecimal::<M, ConstDecimals<7>>::const_decimals_from_raw(BigUint::from(
            17417939u64, // 1.7417939, 7 decimals
        ))
        .rescale(precision.clone());

        let log_2 =
            ManagedDecimal::<M, ConstDecimals<0>>::const_decimals_from_raw(BigUint::from(log2));
        let ln_of_2 = ManagedDecimal::<M, ConstDecimals<8>>::const_decimals_from_raw(
            BigUint::from(69314718u64),
        ); // 0.69314718 8 decimals

        result + log_2.mul_with_precision(ln_of_2, precision.clone())
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
    fn from(mut value: BigUint<M>) -> Self {
        let decimals = ConstDecimals;
        value *= decimals.scaling_factor().deref();
        ManagedDecimal {
            data: value,
            decimals,
        }
    }
}

impl<M: ManagedTypeApi> TypeAbiFrom<Self> for ManagedDecimal<M, NumDecimals> {}

impl<M: ManagedTypeApi> TypeAbi for ManagedDecimal<M, NumDecimals> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from("ManagedDecimal<usize>")
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
