use core::convert::TryInto;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{
        BigIntApiImpl, HandleConstraints, ManagedBufferApiImpl, ManagedTypeApi, ManagedTypeApiImpl,
        RawHandle, const_handles, quick_signal_error, use_raw_handle,
    },
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
        NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
    },
    contract_base::ErrorHelper,
    err_msg,
    formatter::{FormatByteReceiver, SCDisplay, hex_util::encode_bytes_as_hex},
    types::{
        BigInt, Decimals, LnDecimals, ManagedBuffer, ManagedDecimal, ManagedRef, ManagedType,
        NonZeroBigUint, heap::BoxedBytes,
    },
};

/// A big, unsigned number.
///
/// Guaranteed to never be negative by construction and guarded operations.
#[repr(transparent)]
pub struct BigUint<M: ManagedTypeApi> {
    pub(crate) value: BigInt<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigUint<M> {
    type OwnHandle = M::BigIntHandle;

    unsafe fn from_handle(handle: M::BigIntHandle) -> Self {
        unsafe {
            BigUint {
                value: BigInt::from_handle(handle),
            }
        }
    }

    fn get_handle(&self) -> M::BigIntHandle {
        self.value.handle.clone()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        unsafe { self.value.forget_into_handle() }
    }

    fn transmute_from_handle_ref(handle_ref: &M::BigIntHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }

    fn transmute_from_handle_ref_mut(handle_ref: &mut M::BigIntHandle) -> &mut Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> From<u128> for BigUint<M> {
    fn from(value: u128) -> Self {
        BigUint::from_bytes_be(&value.to_be_bytes()[..])
    }
}

impl<M: ManagedTypeApi> TypeAbiFrom<u128> for BigUint<M> {}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for BigUint<M> {
    #[inline]
    fn from(item: ManagedBuffer<M>) -> Self {
        BigUint::from_bytes_be_buffer(&item)
    }
}

impl<M: ManagedTypeApi> From<&ManagedBuffer<M>> for BigUint<M> {
    #[inline]
    fn from(item: &ManagedBuffer<M>) -> Self {
        BigUint::from_bytes_be_buffer(item)
    }
}

impl<M: ManagedTypeApi> BigUint<M> {
    /// Creates a new BigUint from a BigInt without checking invariants.
    ///
    /// ## Safety
    ///
    /// The value needs to be >= 0, otherwise the invariant is broken and subtle bugs can appear.
    pub unsafe fn new_unchecked(bi: BigInt<M>) -> Self {
        BigUint { value: bi }
    }

    /// Creates a new object, without initializing it.
    ///
    /// ## Safety
    ///
    /// The value needs to be initialized after creation, otherwise the VM will halt the first time
    /// the value is attempted to be read.
    ///
    /// ## Panic / unwind safety
    ///
    /// If the caller unwinds (panics) before the returned value is fully initialized, the
    /// `BigUint` is dropped normally — which issues a `drop_big_int` call for an uninitialized
    /// handle. This may corrupt the VM handle table. Callers must ensure initialization
    /// completes before any panic can occur.
    pub unsafe fn new_uninit() -> Self {
        unsafe { Self::new_unchecked(BigInt::new_uninit()) }
    }

    /// Creates a new object and initializes it via a closure that receives the raw handle.
    ///
    /// ## Safety
    ///
    /// The closure `init_fn` must fully initialize the value behind the handle before returning.
    /// The initialized value must also be non-negative, otherwise the `BigUint` invariant is broken.
    ///
    /// ## Panic / unwind safety
    ///
    /// If `init_fn` unwinds (panics), the partially-constructed `BigUint` is leaked — its
    /// destructor is **not** called. This means no `drop_big_int` call will be issued for the
    /// allocated handle, which may leave the VM handle table in an inconsistent state.
    /// Callers must ensure that `init_fn` does not panic.
    pub unsafe fn new_init_handle<F>(init_fn: F) -> Self
    where
        F: FnOnce(M::BigIntHandle),
    {
        unsafe { Self::new_unchecked(BigInt::new_init_handle(init_fn)) }
    }

    pub(crate) fn set_value<T>(handle: M::BigIntHandle, value: T)
    where
        T: TryInto<i64> + num_traits::Unsigned,
    {
        BigInt::<M>::set_value(handle, value);
    }

    pub(crate) fn new_from_num<T>(value: T) -> Self
    where
        T: TryInto<i64> + num_traits::Unsigned,
    {
        unsafe {
            Self::new_init_handle(|handle| {
                Self::set_value(handle, value);
            })
        }
    }

    pub(crate) fn make_temp<T>(handle: RawHandle, value: T) -> M::BigIntHandle
    where
        T: TryInto<i64> + num_traits::Unsigned,
    {
        let temp: M::BigIntHandle = use_raw_handle(handle);
        Self::set_value(temp.clone(), value);
        temp
    }

    pub fn as_big_int(&self) -> &BigInt<M> {
        &self.value
    }

    pub fn into_big_int(self) -> BigInt<M> {
        self.value
    }

    /// Converts this `BigUint` into a `NonZeroBigUint`, returning `Some` if the value is non-zero, or `None` if it is zero.
    ///
    /// # Returns
    /// - `Some(NonZeroBigUint)` if the value is non-zero.
    /// - `None` if the value is zero.
    ///
    /// # Example
    /// ```ignore
    /// let big = BigUint::from(5u32);
    /// let non_zero = big.into_non_zero();
    /// assert!(non_zero.is_some());
    /// ```
    pub fn into_non_zero(self) -> Option<NonZeroBigUint<M>> {
        NonZeroBigUint::new(self)
    }

    /// Converts this `BigUint` into a `NonZeroBigUint`, panicking if the value is zero.
    ///
    /// # Panics
    /// Panics if the value is zero.
    ///
    /// # Example
    /// ```ignore
    /// let big = BigUint::from(5u32);
    /// let non_zero = big.into_non_zero_or_panic();
    /// // Succeeds if value is non-zero, panics otherwise
    /// ```
    pub fn into_non_zero_or_panic(self) -> NonZeroBigUint<M> {
        NonZeroBigUint::new_or_panic(self)
    }
}

macro_rules! big_uint_conv_num {
    ($num_ty:ty) => {
        impl<M: ManagedTypeApi> From<$num_ty> for BigUint<M> {
            #[inline]
            fn from(value: $num_ty) -> Self {
                Self::new_from_num(value)
            }
        }

        impl<M: ManagedTypeApi> TypeAbiFrom<$num_ty> for BigUint<M> {}
    };
}

big_uint_conv_num! {u64}
big_uint_conv_num! {u32}
big_uint_conv_num! {usize}
big_uint_conv_num! {u16}
big_uint_conv_num! {u8}

#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> TypeAbiFrom<crate::codec::num_bigint::BigUint> for BigUint<M> {}
#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> TypeAbiFrom<BigUint<M>> for crate::codec::num_bigint::BigUint {}

impl<M> TypeAbiFrom<Self> for BigUint<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&Self> for BigUint<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> TypeAbi for BigUint<M> {
    #[cfg(feature = "num-bigint")]
    type Unmanaged = crate::codec::num_bigint::BigUint;

    #[cfg(not(feature = "num-bigint"))]
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from("BigUint")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("BigUint<$API>")
    }
}

#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> From<&crate::codec::num_bigint::BigUint> for BigUint<M> {
    fn from(alloc_big_uint: &crate::codec::num_bigint::BigUint) -> Self {
        BigUint::from_bytes_be(alloc_big_uint.to_bytes_be().as_slice())
    }
}
#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> From<crate::codec::num_bigint::BigUint> for BigUint<M> {
    fn from(alloc_big_uint: crate::codec::num_bigint::BigUint) -> Self {
        BigUint::from(&alloc_big_uint)
    }
}
#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> BigUint<M> {
    pub fn to_alloc(&self) -> crate::codec::num_bigint::BigUint {
        crate::codec::num_bigint::BigUint::from_bytes_be(self.to_bytes_be().as_slice())
    }
}

impl<M: ManagedTypeApi> Default for BigUint<M> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

/// More conversions here.
impl<M: ManagedTypeApi> BigUint<M> {
    #[inline]
    pub fn zero() -> Self {
        unsafe {
            let result = Self::new_uninit();
            M::managed_type_impl().bi_set_int64(result.get_handle(), 0);
            result
        }
    }

    pub fn zero_ref() -> ManagedRef<'static, M, BigUint<M>> {
        let handle: M::BigIntHandle = use_raw_handle(const_handles::BIG_INT_CONST_ZERO);
        M::managed_type_impl().bi_set_int64(handle.clone(), 0);
        unsafe { ManagedRef::wrap_handle(handle) }
    }

    #[inline]
    pub fn to_u64(&self) -> Option<u64> {
        let api = M::managed_type_impl();
        api.bi_to_i64(self.value.handle.clone()).map(|bi| bi as u64)
    }

    #[inline]
    pub fn overwrite_u64(&mut self, value: u64) {
        Self::set_value(self.value.handle.clone(), value);
    }

    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        let mb_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().mb_overwrite(mb_handle.clone(), bytes);
        unsafe {
            let result = Self::new_uninit();
            M::managed_type_impl().mb_to_big_int_unsigned(mb_handle, result.get_handle());
            result
        }
    }

    pub fn to_bytes_be(&self) -> BoxedBytes {
        let mb_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl()
            .mb_from_big_int_unsigned(self.value.handle.clone(), mb_handle.clone());
        M::managed_type_impl().mb_to_boxed_bytes(mb_handle)
    }

    pub fn from_bytes_be_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        unsafe {
            let result = BigUint::new_uninit();
            M::managed_type_impl()
                .mb_to_big_int_unsigned(managed_buffer.handle.clone(), result.get_handle());
            result
        }
    }

    pub fn to_bytes_be_buffer(&self) -> ManagedBuffer<M> {
        unsafe {
            let result = ManagedBuffer::new_uninit();
            M::managed_type_impl().mb_from_big_int_unsigned(self.get_handle(), result.get_handle());
            result
        }
    }
}

impl<M: ManagedTypeApi> BigUint<M> {
    pub fn sqrt(&self) -> Self {
        unsafe {
            let result = BigUint::new_uninit();
            M::managed_type_impl().bi_sqrt(result.get_handle(), self.get_handle());
            result
        }
    }

    /// Assigns `self = base^exp`.
    pub fn pow_assign(&mut self, base: &BigUint<M>, exp: u32) {
        let exp_handle = BigUint::<M>::make_temp(const_handles::BIG_INT_TEMPORARY_1, exp);
        M::managed_type_impl().bi_pow(self.get_handle(), base.get_handle(), exp_handle);
    }

    pub fn pow(&self, exp: u32) -> Self {
        unsafe {
            let mut result = BigUint::new_uninit();
            result.pow_assign(self, exp);
            result
        }
    }

    /// The integer part of the k-th root, computed via Newton's method.
    ///
    /// The initial guess is derived from the number of significant bits (`log2_floor`):
    /// `x0 = 2^(floor(log2(self) / k) + 1)`, which is always an overestimate.
    ///
    /// Returns `0` when `self` is zero.
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

        // log2 is None for the number zero,
        // but in this case we can return early with the correct result of zero without doing any computation
        let Some(log2) = self.log2_floor() else {
            return BigUint::zero();
        };

        // Initial overestimate: 2^(floor(log2 / k) + 1)
        let mut x = BigUint::from(1u64) << ((log2 / k + 1) as usize);

        // Newton's iteration: x = ((k-1)*x + self / x^(k-1)) / k
        // Converges from above; stop when the estimate stops decreasing.
        let k_big = BigUint::<M>::from(k as u64);
        let k_minus_1_big = BigUint::<M>::from((k - 1) as u64);

        // Pre-allocate buffers reused across iterations to avoid per-iteration allocations.
        // SAFETY: both are fully written before being read in every iteration.
        let mut x_pow_k_minus_1 = unsafe { BigUint::new_uninit() };
        let mut new_x = unsafe { BigUint::new_uninit() };
        let api = M::managed_type_impl();
        loop {
            // x_pow_k_minus_1 = x^(k-1)
            x_pow_k_minus_1.pow_assign(&x, k - 1);

            // Reuse x_pow_k_minus_1's handle for self / x^(k-1).
            // The VM reads both operands before writing, so dest == divisor is safe.
            api.bi_t_div(
                x_pow_k_minus_1.get_handle(),
                self.get_handle(),
                x_pow_k_minus_1.get_handle(),
            );

            // new_x = (k-1)*x + self/x^(k-1)
            api.bi_mul(
                new_x.get_handle(),
                k_minus_1_big.get_handle(),
                x.get_handle(),
            );
            new_x += &x_pow_k_minus_1;

            // new_x /= k
            new_x /= &k_big;

            if new_x >= x {
                break;
            }

            // Swap handles instead of cloning: zero API calls, no allocation.
            core::mem::swap(&mut x, &mut new_x);
        }

        x
    }

    /// The whole part of the base-2 logarithm.
    ///
    /// Obtained by counting the significant bits.
    /// More specifically, the log2 floor is the position of the most significant bit minus one.
    ///
    /// Will return `None` for the number zero (the logarithm in this case would approach -inf).
    pub fn log2_floor(&self) -> Option<u32> {
        let api = M::managed_type_impl();
        let result = api.bi_log2(self.value.handle.clone());
        if result < 0 {
            None
        } else {
            Some(result as u32)
        }
    }

    /// Calculates proportion of this value, consuming self.
    ///
    /// # Arguments
    /// * `part` - The numerator value (e.g., 1000 for 1% when total is 100000)
    /// * `total` - The denominator value for the ratio calculation (e.g., 100 for percentage, 100000 for basis points)
    ///
    /// # Returns
    /// The proportional amount as BigUint (self * part / total)
    ///
    /// # Notes
    /// Both `part` and `total` must be at most `i64::MAX`. Values exceeding this limit will cause an error.
    ///
    /// # Example
    /// ```ignore
    /// let amount = BigUint::from(1000u32);
    /// let result = amount.into_proportion(5u32, 100u32); // 5/100 of 1000 = 50
    /// ```
    pub fn into_proportion(self, part: u64, total: u64) -> Self {
        let part_signed: i64 = part
            .try_into()
            .unwrap_or_else(|_| quick_signal_error::<M>(err_msg::PROPORTION_OVERFLOW_ERR));
        let total_signed: i64 = total
            .try_into()
            .unwrap_or_else(|_| quick_signal_error::<M>(err_msg::PROPORTION_OVERFLOW_ERR));

        // mathematically, the result of this operation cannot be negative, so it is safe to skip sign check
        unsafe {
            Self::new_unchecked(
                self.into_big_int()
                    .into_proportion(part_signed, total_signed),
            )
        }
    }

    /// Calculates proportion of this value.
    ///
    /// # Arguments
    /// * `part` - The numerator value (e.g., 1000 for 1% when total is 100000)
    /// * `total` - The denominator value for the ratio calculation (e.g., 100 for percentage, 100000 for basis points)
    ///
    /// # Returns
    /// The proportional amount as BigUint (self * part / total)
    ///
    /// # Notes
    /// Both `part` and `total` must be at most `i64::MAX`. Values exceeding this limit will cause an error.
    ///
    /// # Example
    /// ```ignore
    /// let amount = BigUint::from(1000u32);
    /// let result = amount.proportion(5u32, 100u32); // 5/100 of 1000 = 50
    /// ```
    pub fn proportion(&self, part: u64, total: u64) -> Self {
        self.clone().into_proportion(part, total)
    }

    /// Natural logarithm of a number.
    ///
    /// Returns `None` for 0.
    pub fn ln(&self) -> Option<ManagedDecimal<M, LnDecimals>> {
        // start with approximation, based on position of the most significant bit
        let Some(log2_floor) = self.log2_floor() else {
            // means the input was zero
            return None;
        };

        let scaling_factor_9 = LnDecimals::new().scaling_factor();
        let divisor = BigUint::from(1u64) << log2_floor as usize;
        let normalized = self * &*scaling_factor_9 / divisor;

        let x = normalized
            .to_u64()
            .unwrap_or_else(|| ErrorHelper::<M>::signal_error_with_message("ln internal error"))
            as i64;

        let mut result = crate::math::internal_logarithm_i64::ln_polynomial(x);
        crate::math::internal_logarithm_i64::ln_add_bit_log2(&mut result, log2_floor);

        debug_assert!(result > 0);

        let mut result_bi = normalized; // reuse handle
        result_bi.overwrite_u64(result as u64);

        Some(ManagedDecimal::const_decimals_from_raw(result_bi))
    }
}

impl<M: ManagedTypeApi> Clone for BigUint<M> {
    fn clone(&self) -> Self {
        unsafe { self.as_big_int().clone().into_big_uint_unchecked() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.value.clone_from(&source.value);
    }
}

impl<M: ManagedTypeApi> TryStaticCast for BigUint<M> {}

impl<M: ManagedTypeApi> TopEncode for BigUint<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if O::supports_specialized_type::<Self>() {
            output.set_specialized(self, h)
        } else {
            output.set_slice_u8(self.to_bytes_be().as_slice());
            Ok(())
        }
    }
}

impl<M: ManagedTypeApi> NestedEncode for BigUint<M> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.to_bytes_be_buffer().dep_encode_or_handle_err(dest, h)
    }
}

impl<M: ManagedTypeApi> NestedDecode for BigUint<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        if I::supports_specialized_type::<Self>() {
            input.read_specialized((), h)
        } else {
            let boxed_bytes = BoxedBytes::dep_decode_or_handle_err(input, h)?;
            Ok(Self::from_bytes_be(boxed_bytes.as_slice()))
        }
    }
}

impl<M: ManagedTypeApi> TopDecode for BigUint<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        if I::supports_specialized_type::<Self>() {
            input.into_specialized(h)
        } else {
            let boxed_bytes = BoxedBytes::top_decode_or_handle_err(input, h)?;
            Ok(Self::from_bytes_be(boxed_bytes.as_slice()))
        }
    }
}

impl<M: ManagedTypeApi> BigUint<M> {
    /// Creates a managed buffer containing the textual representation of the number.
    pub fn to_display(&self) -> ManagedBuffer<M> {
        self.value.to_display()
    }
}

impl<M: ManagedTypeApi> SCDisplay for BigUint<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let str_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().bi_to_string(self.value.handle.clone(), str_handle.clone());
        let cast_handle = str_handle.cast_or_signal_error::<M, _>();
        let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
        f.append_managed_buffer(&wrap_cast);
    }
}

#[cfg(feature = "alloc")]
impl<M: ManagedTypeApi> core::fmt::Display for BigUint<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.to_display(), f)
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for BigUint<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BigUint")
            .field("handle", &self.value.handle.clone())
            .field(
                "hex-value-be",
                &encode_bytes_as_hex(self.to_bytes_be().as_slice()),
            )
            .finish()
    }
}
