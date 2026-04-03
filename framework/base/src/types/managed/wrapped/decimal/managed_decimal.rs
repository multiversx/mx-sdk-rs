use alloc::string::ToString;
use core::{cmp::Ordering, ops::Deref};
use multiversx_sc_codec::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{ManagedTypeApi, ManagedTypeApiImpl, quick_signal_error},
    err_msg,
    formatter::{FormatBuffer, FormatByteReceiver, SCDisplay},
    typenum::{U4, U8, Unsigned},
    types::{
        BigUint, ManagedBufferCachedBuilder, ManagedRef, ManagedVecItem,
        ManagedVecItemPayloadBuffer, Ref, managed_vec_item_read_from_payload_index,
        managed_vec_item_save_to_payload_index,
    },
};

use super::{ConstDecimals, Decimals, ManagedDecimalSigned, NumDecimals};

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

    /// Returns the multiplicative identity `1` at the given `decimals` precision.
    ///
    /// The raw value is `10^decimals` (the scaling factor), so that
    /// `self.trunc()` returns `1` and all arithmetic treats it as unity.
    pub fn one(decimals: D) -> Self {
        let data = (*decimals.scaling_factor::<M>()).clone();
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
            }
            Ordering::Equal => self.data.clone(),
            Ordering::Greater => {
                let delta_decimals = from_num_decimals - scale_to_num_decimals;
                let scaling_factor: &BigUint<M> = &delta_decimals.scaling_factor();
                &self.data / scaling_factor
            }
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

impl<M: ManagedTypeApi, DECIMALS: Unsigned> From<BigUint<M>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    fn from(mut value: BigUint<M>) -> Self {
        let decimals = ConstDecimals::new();
        value *= decimals.scaling_factor().deref();
        ManagedDecimal {
            data: value,
            decimals,
        }
    }
}

impl<M: ManagedTypeApi, DECIMALS: Unsigned> ManagedDecimal<M, ConstDecimals<DECIMALS>> {
    pub fn const_decimals_from_raw(data: BigUint<M>) -> Self {
        ManagedDecimal {
            data,
            decimals: ConstDecimals::new(),
        }
    }

    /// Converts from constant (compile-time) number of decimals to a variable number of decimals.
    pub fn into_var_decimals(self) -> ManagedDecimal<M, NumDecimals> {
        ManagedDecimal {
            data: self.data,
            decimals: DECIMALS::to_usize(),
        }
    }
}

impl<M: ManagedTypeApi, DECIMALS: Unsigned> From<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, NumDecimals>
{
    fn from(value: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self {
        value.into_var_decimals()
    }
}

impl<M: ManagedTypeApi, D: Decimals> ManagedDecimal<M, D> {
    /// Integer part of the k-th root, preserving the decimal scale.
    ///
    /// Internally pre-scales the raw data by `scaling_factor^(k-1)` so that after
    /// taking the integer root the decimal point lands in the correct position:
    ///
    /// ```text
    /// self.data = v * 10^d
    ///   →  scaled = self.data * (10^d)^(k-1) = v * 10^(d*k)
    ///   →  root   = floor(scaled^(1/k)) = floor(v^(1/k) * 10^d)
    /// ```
    ///
    /// Returns `0` (with the same scale) when `self` is zero.
    ///
    /// # Panics
    /// Panics if `k` is zero.
    pub fn nth_root(&self, k: u32) -> Self {
        if k == 0 {
            quick_signal_error::<M>(err_msg::BIG_UINT_NTH_ROOT_ZERO);
        }

        if k == 1 {
            return self.clone();
        }

        let sf = self.decimals.scaling_factor::<M>();
        // Multiply by sf^(k-1) before rooting so the decimal position is preserved.
        // For k==0, the check in BigUint::nth_root handles the error signal.
        let scaled = &self.data * &sf.pow(k.saturating_sub(1));
        ManagedDecimal::from_raw_units(scaled.nth_root_unchecked(k), self.decimals.clone())
    }

    /// Approximates e^`self` using a 5-term Taylor series.
    ///
    /// Treats `self` as the exponent `x` and computes:
    ///
    /// ```text
    /// e^x ≈ 1 + x + x²/2! + x³/3! + x⁴/4! + x⁵/5!
    /// ```
    ///
    /// The result has the same precision as `self`; all intermediate steps use
    /// [`mul_half_up`] / [`div_half_up`] to prevent rounding errors from
    /// accumulating toward zero.
    ///
    /// Accurate for small `x` (i.e. when `x ≪ 1`). Error is O(x⁶/720).
    pub fn exp_approx(&self) -> ManagedDecimal<M, D>
    where
        ManagedDecimal<M, D>: core::ops::Add<Output = ManagedDecimal<M, D>>,
    {
        let one = ManagedDecimal::<M, D>::one(self.decimals.clone());

        // Higher powers of x (x = self)
        let x_sq = self.mul_half_up(self, self.decimals.clone());
        let x_cub = x_sq.mul_half_up(self, self.decimals.clone());
        let x_pow4 = x_cub.mul_half_up(self, self.decimals.clone());
        let x_pow5 = x_pow4.mul_half_up(self, self.decimals.clone());

        // x^n / n! — reuse one ManagedDecimal, overwriting its data for each factorial
        const FACT_2: u64 = 2;
        const FACT_3: u64 = 6;
        const FACT_4: u64 = 24;
        const FACT_5: u64 = 120;
        let mut factor =
            ManagedDecimal::<M, NumDecimals>::from_raw_units(BigUint::from(FACT_2), 0usize);
        let term2 = x_sq.div_half_up(&factor, self.decimals.clone());
        factor.data.overwrite_u64(FACT_3);
        let term3 = x_cub.div_half_up(&factor, self.decimals.clone());
        factor.data.overwrite_u64(FACT_4);
        let term4 = x_pow4.div_half_up(&factor, self.decimals.clone());
        factor.data.overwrite_u64(FACT_5);
        let term5 = x_pow5.div_half_up(&factor, self.decimals.clone());

        // 1 + x + x²/2! + x³/3! + x⁴/4! + x⁵/5!
        let mut result = one;
        result += self; // using += allows us to avoid cloning self
        result += term2;
        result += term3;
        result += term4;
        result += term5;
        result
    }

    /// Approximates e^(`self` × `expiration`) using a 5-term Taylor series.
    ///
    /// This is the standard compound-interest growth factor calculation used in
    /// continuous-compounding models (e.g. DeFi lending indices):
    ///
    /// ```text
    /// e^(rate * t) ≈ 1 + x + x²/2! + x³/3! + x⁴/4! + x⁵/5!,  where x = rate * t
    /// ```
    ///
    /// Returns `1` (at `precision`) when `expiration == 0`.
    ///
    /// # Credits
    /// Original implementation by [@mihaieremia](https://github.com/mihaieremia).
    pub fn compounded_interest<Precision: Decimals>(
        &self,
        expiration: u64,
        precision: Precision,
    ) -> ManagedDecimal<M, Precision>
    where
        ManagedDecimal<M, Precision>: core::ops::Add<Output = ManagedDecimal<M, Precision>>,
    {
        if expiration == 0 {
            return ManagedDecimal::<M, Precision>::one(precision.clone());
        }

        // Represent the time delta as an exact integer decimal (0 dp)
        let expiration_decimal =
            ManagedDecimal::<M, NumDecimals>::from_raw_units(BigUint::from(expiration), 0usize);

        // x = rate * time_delta
        let x = self.mul_half_up(&expiration_decimal, precision.clone());
        x.exp_approx()
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for ManagedDecimal<M, NumDecimals> {
    type PAYLOAD = ManagedVecItemPayloadBuffer<U8>; // 4 bigUint + 4 usize

    const SKIPS_RESERIALIZATION: bool = false;

    type Ref<'a> = Ref<'a, Self>;

    unsafe fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let mut index = 0;
        unsafe {
            Self {
                data: managed_vec_item_read_from_payload_index(payload, &mut index),
                decimals: managed_vec_item_read_from_payload_index(payload, &mut index),
            }
        }
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        unsafe { Ref::new(Self::read_from_payload(payload)) }
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        let mut index = 0;
        unsafe {
            managed_vec_item_save_to_payload_index(self.data, payload, &mut index);
            managed_vec_item_save_to_payload_index(self.decimals, payload, &mut index);
        }
    }

    fn requires_drop() -> bool {
        M::managed_type_impl().requires_managed_type_drop()
    }
}

impl<M: ManagedTypeApi, DECIMALS: Unsigned> ManagedVecItem
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type PAYLOAD = ManagedVecItemPayloadBuffer<U4>; // data only

    const SKIPS_RESERIALIZATION: bool = false;

    type Ref<'a> = Ref<'a, Self>;

    unsafe fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        unsafe { Self::const_decimals_from_raw(BigUint::read_from_payload(payload)) }
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        unsafe { Ref::new(Self::read_from_payload(payload)) }
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        self.data.save_to_payload(payload);
    }

    fn requires_drop() -> bool {
        M::managed_type_impl().requires_managed_type_drop()
    }
}

impl<M: ManagedTypeApi, DECIMALS: Unsigned> TopEncode
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

impl<M: ManagedTypeApi, DECIMALS: Unsigned> TopDecode
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

impl<M: ManagedTypeApi, DECIMALS: Unsigned> NestedEncode
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

impl<M: ManagedTypeApi, DECIMALS: Unsigned> NestedDecode
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

impl<M: ManagedTypeApi, DECIMALS: Unsigned> TypeAbiFrom<Self>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
}

impl<M: ManagedTypeApi, DECIMALS: Unsigned> TypeAbi for ManagedDecimal<M, ConstDecimals<DECIMALS>> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from(alloc::format!("ManagedDecimal<{}>", DECIMALS::to_usize()))
    }

    fn type_name_rust() -> TypeName {
        TypeName::from(alloc::format!(
            "ManagedDecimal<$API, ConstDecimals<U{}>>",
            DECIMALS::to_usize()
        ))
    }

    fn is_variadic() -> bool {
        false
    }
}
impl<M: ManagedTypeApi, D: Decimals> SCDisplay for ManagedDecimal<M, D> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        super::managed_decimal_signed::managed_decimal_fmt(
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
