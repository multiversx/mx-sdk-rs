use core::convert::TryInto;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{
        const_handles, use_raw_handle, BigIntApiImpl, HandleConstraints, ManagedBufferApiImpl,
        ManagedTypeApi, ManagedTypeApiImpl, RawHandle,
    },
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
        NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
    },
    contract_base::ErrorHelper,
    formatter::{hex_util::encode_bytes_as_hex, FormatBuffer, FormatByteReceiver, SCDisplay},
    types::{
        heap::BoxedBytes, BigInt, Decimals, LnDecimals, ManagedBuffer, ManagedBufferCachedBuilder,
        ManagedDecimal, ManagedRef, ManagedType,
    },
};

#[repr(transparent)]
pub struct BigUint<M: ManagedTypeApi> {
    pub(crate) value: BigInt<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigUint<M> {
    type OwnHandle = M::BigIntHandle;

    unsafe fn from_handle(handle: M::BigIntHandle) -> Self {
        BigUint {
            value: BigInt::from_handle(handle),
        }
    }

    fn get_handle(&self) -> M::BigIntHandle {
        self.value.handle.clone()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        self.value.forget_into_handle()
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
    /// Creates a new object, without initializing it.
    ///
    /// ## Safety
    ///
    /// The value needs to be initialized after creation, otherwise the VM will halt the first time the value is attempted to be read.
    pub unsafe fn new_uninit() -> Self {
        BigUint {
            value: BigInt::new_uninit(),
        }
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
            let result = Self::new_uninit();
            Self::set_value(result.get_handle(), value);
            result
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

    pub fn pow(&self, exp: u32) -> Self {
        let big_int_temp_1 = BigUint::<M>::make_temp(const_handles::BIG_INT_TEMPORARY_1, exp);
        unsafe {
            let result = BigUint::new_uninit();
            M::managed_type_impl().bi_pow(result.get_handle(), self.get_handle(), big_int_temp_1);
            result
        }
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

        let mut result = crate::types::math_util::logarithm_i64::ln_polynomial(x);
        crate::types::math_util::logarithm_i64::ln_add_bit_log2(&mut result, log2_floor);

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

impl<M: ManagedTypeApi> SCDisplay for BigUint<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let str_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().bi_to_string(self.value.handle.clone(), str_handle.clone());
        let cast_handle = str_handle.cast_or_signal_error::<M, _>();
        let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
        f.append_managed_buffer(&wrap_cast);
    }
}

impl<M: ManagedTypeApi> BigUint<M> {
    /// Creates to a managed buffer containing the textual representation of the number.
    pub fn to_display(&self) -> ManagedBuffer<M> {
        let mut result = ManagedBufferCachedBuilder::new_from_slice(&[]);
        result.append_display(self);
        result.into_managed_buffer()
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
