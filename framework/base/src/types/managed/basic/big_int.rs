use core::{convert::TryInto, marker::PhantomData};

use crate::{
    abi::{TypeAbiFrom, TypeName},
    api::{
        BigIntApiImpl, HandleConstraints, ManagedBufferApiImpl, ManagedTypeApi, ManagedTypeApiImpl,
        RawHandle, StaticVarApiImpl, const_handles, use_raw_handle,
    },
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
        NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
    },
    formatter::{FormatByteReceiver, SCDisplay, hex_util::encode_bytes_as_hex},
    types::{
        BigUint, ManagedBuffer, ManagedOption, ManagedRef, ManagedType, Sign, heap::BoxedBytes,
    },
};

use super::cast_to_i64::cast_to_i64;

#[repr(transparent)]
pub struct BigInt<M: ManagedTypeApi> {
    pub(crate) handle: M::BigIntHandle,
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigInt<M> {
    type OwnHandle = M::BigIntHandle;

    unsafe fn from_handle(handle: M::BigIntHandle) -> Self {
        BigInt {
            handle,
            _phantom: PhantomData,
        }
    }

    fn get_handle(&self) -> M::BigIntHandle {
        self.handle.clone()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        unsafe {
            let handle = core::ptr::read(&self.handle);
            core::mem::forget(self);
            handle
        }
    }

    fn transmute_from_handle_ref(handle_ref: &M::BigIntHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }

    fn transmute_from_handle_ref_mut(handle_ref: &mut M::BigIntHandle) -> &mut Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> Default for BigInt<M> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl<M: ManagedTypeApi> From<&ManagedBuffer<M>> for BigInt<M> {
    #[inline]
    fn from(item: &ManagedBuffer<M>) -> Self {
        BigInt::from_signed_bytes_be_buffer(item)
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for BigInt<M> {
    #[inline]
    fn from(item: ManagedBuffer<M>) -> Self {
        BigInt::from_signed_bytes_be_buffer(&item)
    }
}

impl<M: ManagedTypeApi> BigInt<M> {
    /// Creates a new object, without initializing it.
    ///
    /// ## Safety
    ///
    /// The value needs to be initialized after creation, otherwise the VM will halt the first time the value is attempted to be read.
    pub unsafe fn new_uninit() -> Self {
        unsafe {
            let new_handle: M::BigIntHandle =
                use_raw_handle(M::static_var_api_impl().next_handle());
            BigInt::from_handle(new_handle)
        }
    }

    pub(crate) fn set_value<T>(handle: M::BigIntHandle, value: T)
    where
        T: TryInto<i64>,
    {
        M::managed_type_impl().bi_set_int64(handle, cast_to_i64::<M, _>(value));
    }

    pub(crate) fn make_temp<T>(handle: RawHandle, value: T) -> M::BigIntHandle
    where
        T: TryInto<i64>,
    {
        let temp: M::BigIntHandle = use_raw_handle(handle);
        Self::set_value(temp.clone(), value);
        temp
    }
}

impl<M: ManagedTypeApi> From<BigUint<M>> for BigInt<M> {
    #[inline]
    fn from(item: BigUint<M>) -> Self {
        item.into_big_int()
    }
}

macro_rules! big_int_conv_num {
    ($num_ty:ty) => {
        impl<M: ManagedTypeApi> From<$num_ty> for BigInt<M> {
            #[inline]
            fn from(value: $num_ty) -> Self {
                unsafe {
                    let result = BigInt::new_uninit();
                    Self::set_value(result.get_handle(), value);
                    result
                }
            }
        }

        impl<M: ManagedTypeApi> TypeAbiFrom<$num_ty> for BigInt<M> {}
        impl<M: ManagedTypeApi> TypeAbiFrom<&$num_ty> for BigInt<M> {}
    };
}

// TODO: more coverage, only from i64 currently tested
big_int_conv_num! {i64}
big_int_conv_num! {i32}
big_int_conv_num! {isize}
big_int_conv_num! {i16}
big_int_conv_num! {i8}

#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> TypeAbiFrom<crate::codec::num_bigint::BigInt> for BigInt<M> {}
#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> TypeAbiFrom<BigInt<M>> for crate::codec::num_bigint::BigInt {}

impl<M> TypeAbiFrom<Self> for BigInt<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&Self> for BigInt<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> crate::abi::TypeAbi for BigInt<M> {
    #[cfg(feature = "num-bigint")]
    type Unmanaged = crate::codec::num_bigint::BigInt;

    #[cfg(not(feature = "num-bigint"))]
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from("BigInt")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("BigInt<$API>")
    }
}

#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> From<&crate::codec::num_bigint::BigInt> for BigInt<M> {
    fn from(alloc_big_int: &crate::codec::num_bigint::BigInt) -> Self {
        BigInt::from_signed_bytes_be(alloc_big_int.to_signed_bytes_be().as_slice())
    }
}
#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> From<crate::codec::num_bigint::BigInt> for BigInt<M> {
    fn from(alloc_big_int: crate::codec::num_bigint::BigInt) -> Self {
        BigInt::from(&alloc_big_int)
    }
}

impl<M: ManagedTypeApi> BigInt<M> {
    #[inline]
    pub fn zero() -> Self {
        unsafe {
            let result = BigInt::new_uninit();
            M::managed_type_impl().bi_set_int64(result.get_handle(), 0);
            result
        }
    }

    #[inline]
    pub fn to_i64(&self) -> Option<i64> {
        M::managed_type_impl().bi_to_i64(self.handle.clone())
    }

    #[inline]
    pub fn overwrite_i64(&self, value: i64) {
        Self::set_value(self.handle.clone(), value);
    }

    #[inline]
    pub fn from_signed_bytes_be(bytes: &[u8]) -> Self {
        let mb_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().mb_overwrite(mb_handle.clone(), bytes);
        unsafe {
            let result = BigInt::new_uninit();
            M::managed_type_impl().mb_to_big_int_signed(mb_handle, result.get_handle());
            result
        }
    }

    #[inline]
    pub fn to_signed_bytes_be(&self) -> BoxedBytes {
        let mb_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().mb_from_big_int_signed(self.handle.clone(), mb_handle.clone());
        M::managed_type_impl().mb_to_boxed_bytes(mb_handle)
    }

    #[inline]
    pub fn from_signed_bytes_be_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        unsafe {
            let result = BigInt::new_uninit();
            M::managed_type_impl()
                .mb_to_big_int_signed(managed_buffer.handle.clone(), result.get_handle());
            result
        }
    }

    #[inline]
    pub fn to_signed_bytes_be_buffer(&self) -> ManagedBuffer<M> {
        unsafe {
            let result = ManagedBuffer::new_uninit();
            M::managed_type_impl().mb_from_big_int_signed(self.handle.clone(), result.get_handle());
            result
        }
    }

    pub(crate) fn clone_to_handle(source_handle: M::BigIntHandle, dest_handle: M::BigIntHandle) {
        let api = M::managed_type_impl();
        api.bi_set_int64(dest_handle.clone(), 0);
        api.bi_add(dest_handle.clone(), dest_handle, source_handle);
    }
}

impl<M: ManagedTypeApi> Clone for BigInt<M> {
    fn clone(&self) -> Self {
        unsafe {
            let result = BigInt::new_uninit();
            BigInt::<M>::clone_to_handle(self.get_handle(), result.get_handle());
            result
        }
    }
}

impl<M: ManagedTypeApi> BigInt<M> {
    pub fn from_biguint(sign: Sign, unsigned: BigUint<M>) -> Self {
        let result = unsigned.into_big_int();
        if sign.is_minus() {
            M::managed_type_impl().bi_neg(result.handle.clone(), result.handle.clone());
        }
        result
    }

    /// Returns the sign of the `BigInt` as a `Sign`.
    pub fn sign(&self) -> Sign {
        let api = M::managed_type_impl();
        match api.bi_sign(self.handle.clone()) {
            crate::api::Sign::Plus => Sign::Plus,
            crate::api::Sign::NoSign => Sign::NoSign,
            crate::api::Sign::Minus => Sign::Minus,
        }
    }

    /// Returns the magnitude of the `BigInt` as a `BigUint`.
    pub fn magnitude(&self) -> BigUint<M> {
        unsafe {
            let result = BigUint::new_uninit();
            M::managed_type_impl().bi_abs(result.get_handle(), self.get_handle());
            result
        }
    }

    /// Convert this `BigInt` into its `Sign` and `BigUint` magnitude,
    /// the reverse of `BigInt::from_biguint`.
    pub fn to_parts(self) -> (Sign, BigUint<M>) {
        (self.sign(), self.magnitude())
    }

    /// Converts to an unsigned `BigUint`, without performing any checks.
    ///
    /// # Safety
    ///
    /// If the number is negative, undefined behavior might occur further down the execution.
    pub unsafe fn into_big_uint_unchecked(self) -> BigUint<M> {
        BigUint { value: self }
    }

    /// Converts this `BigInt` into a `BigUint`, if it's not negative.
    pub fn into_big_uint(self) -> ManagedOption<M, BigUint<M>> {
        if let Sign::Minus = self.sign() {
            ManagedOption::none()
        } else {
            ManagedOption::some(unsafe { self.into_big_uint_unchecked() })
        }
    }
}

impl<M: ManagedTypeApi> TryStaticCast for BigInt<M> {}

impl<M: ManagedTypeApi> TopEncode for BigInt<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if O::supports_specialized_type::<Self>() {
            output.set_specialized(self, h)
        } else {
            output.set_slice_u8(self.to_signed_bytes_be().as_slice());
            Ok(())
        }
    }
}

impl<M: ManagedTypeApi> NestedEncode for BigInt<M> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.to_signed_bytes_be_buffer()
            .dep_encode_or_handle_err(dest, h)
    }
}

impl<M: ManagedTypeApi> NestedDecode for BigInt<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        if I::supports_specialized_type::<Self>() {
            input.read_specialized((), h)
        } else {
            let boxed_bytes = BoxedBytes::dep_decode_or_handle_err(input, h)?;
            Ok(Self::from_signed_bytes_be(boxed_bytes.as_slice()))
        }
    }
}

impl<M: ManagedTypeApi> TopDecode for BigInt<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        if I::supports_specialized_type::<Self>() {
            input.into_specialized(h)
        } else {
            let boxed_bytes = BoxedBytes::top_decode_or_handle_err(input, h)?;
            Ok(Self::from_signed_bytes_be(boxed_bytes.as_slice()))
        }
    }
}

impl<M: ManagedTypeApi> BigInt<M> {
    #[must_use]
    pub fn pow(&self, exp: u32) -> Self {
        let exp_handle = BigUint::<M>::make_temp(const_handles::BIG_INT_TEMPORARY_1, exp);
        unsafe {
            let result = BigInt::new_uninit();
            M::managed_type_impl().bi_pow(result.get_handle(), self.get_handle(), exp_handle);
            result
        }
    }
}

impl<M: ManagedTypeApi> SCDisplay for BigInt<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let str_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().bi_to_string(self.handle.clone(), str_handle.clone());
        let cast_handle = str_handle.cast_or_signal_error::<M, _>();
        let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
        f.append_managed_buffer(&wrap_cast);
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for BigInt<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BigInt")
            .field("handle", &self.handle.clone())
            .field(
                "hex-value-be",
                &encode_bytes_as_hex(self.to_signed_bytes_be().as_slice()),
            )
            .finish()
    }
}
