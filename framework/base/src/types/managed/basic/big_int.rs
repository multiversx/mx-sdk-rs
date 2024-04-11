use core::{convert::TryInto, marker::PhantomData};

use crate::{
    abi::TypeName,
    api::{
        BigIntApiImpl, const_handles, HandleConstraints, ManagedBufferApiImpl, ManagedTypeApi,
        ManagedTypeApiImpl, RawHandle, StaticVarApiImpl, use_raw_handle,
    },
    codec::{
        CodecFrom, CodecFromSelf, DecodeErrorHandler, EncodeErrorHandler, NestedDecode,
        NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode,
        TopEncodeOutput, TryStaticCast,
    },
    formatter::{FormatByteReceiver, hex_util::encode_bytes_as_hex, SCDisplay},
    types::{BigUint, heap::BoxedBytes, ManagedBuffer, ManagedOption, ManagedType, Sign},
};

use super::cast_to_i64::cast_to_i64;

#[repr(transparent)]
pub struct BigInt<'a, M: ManagedTypeApi<'a>> {
    pub(crate) handle: M::BigIntHandle,
    _phantom: PhantomData<M>,
}

impl<'a, M: ManagedTypeApi<'a>> ManagedType<'a, M> for BigInt<'a, M> {
    type OwnHandle = M::BigIntHandle;

    fn from_handle(handle: M::BigIntHandle) -> Self {
        BigInt {
            handle,
            _phantom: PhantomData,
        }
    }

    unsafe fn get_handle(&self) -> M::BigIntHandle {
        self.handle.clone()
    }

    fn take_handle(mut self) -> Self::OwnHandle {
        core::mem::take(&mut self.handle)
    }

    fn transmute_from_handle_ref(handle_ref: &M::BigIntHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<'a, M: ManagedTypeApi<'a>> Default for BigInt<'a, M> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<&ManagedBuffer<'a, M>> for BigInt<'a, M> {
    #[inline]
    fn from(item: &ManagedBuffer<'a, M>) -> Self {
        BigInt::from_signed_bytes_be_buffer(item)
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<ManagedBuffer<'a, M>> for BigInt<'a, M> {
    #[inline]
    fn from(item: ManagedBuffer<'a, M>) -> Self {
        BigInt::from_signed_bytes_be_buffer(&item)
    }
}

impl<'a, M: ManagedTypeApi<'a>> BigInt<'a, M> {
    pub(crate) fn set_value<T>(handle: M::BigIntHandle, value: T)
    where
        T: TryInto<i64>,
    {
        M::managed_type_impl().bi_set_int64(handle, cast_to_i64::<'a, M, _>(value));
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

impl<'a, M: ManagedTypeApi<'a>> From<BigUint<'a, M>> for BigInt<'a, M> {
    #[inline]
    fn from(item: BigUint<'a, M>) -> Self {
        BigInt::from_handle(item.take_handle())
    }
}

macro_rules! big_int_conv_num {
    ($num_ty:ty) => {
        impl<'a, M: ManagedTypeApi<'a>> From<$num_ty> for BigInt<'a, M> {
            #[inline]
            fn from(value: $num_ty) -> Self {
                let handle: M::BigIntHandle =
                    use_raw_handle(M::static_var_api_impl().next_handle());
                Self::set_value(handle.clone(), value);
                BigInt::from_handle(handle)
            }
        }

        impl<'a, M: ManagedTypeApi<'a>> CodecFrom<$num_ty> for BigInt<'a, M> {}
    };
}

// TODO: more coverage, only from i64 currently tested
big_int_conv_num! {i64}
big_int_conv_num! {i32}
big_int_conv_num! {isize}
big_int_conv_num! {i16}
big_int_conv_num! {i8}

impl<'a, M> CodecFromSelf for BigInt<'a, M> where M: ManagedTypeApi<'a> {}

#[cfg(feature = "num-bigint")]
impl<'a, M: ManagedTypeApi<'a>> CodecFrom<crate::codec::num_bigint::BigInt> for BigInt<'a, M> {}
#[cfg(feature = "num-bigint")]
impl<'a, M: ManagedTypeApi<'a>> CodecFrom<BigInt<'a, M>> for crate::codec::num_bigint::BigInt {}

#[cfg(feature = "num-bigint")]
impl<'a, M: ManagedTypeApi<'a>> From<&crate::codec::num_bigint::BigInt> for BigInt<'a, M> {
    fn from(alloc_big_int: &crate::codec::num_bigint::BigInt) -> Self {
        BigInt::from_signed_bytes_be(alloc_big_int.to_signed_bytes_be().as_slice())
    }
}
#[cfg(feature = "num-bigint")]
impl<'a, M: ManagedTypeApi<'a>> From<crate::codec::num_bigint::BigInt> for BigInt<'a, M> {
    fn from(alloc_big_int: crate::codec::num_bigint::BigInt) -> Self {
        BigInt::from(&alloc_big_int)
    }
}

impl<'a, M: ManagedTypeApi<'a>> BigInt<'a, M> {
    #[inline]
    pub fn zero() -> Self {
        let handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        // TODO: seting 0 will no longer be needed once we fix VM handle error
        M::managed_type_impl().bi_set_int64(handle.clone(), 0);
        BigInt::from_handle(handle)
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
        let handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mb_to_big_int_signed(mb_handle, handle.clone());
        BigInt::from_handle(handle)
    }

    #[inline]
    pub fn to_signed_bytes_be(&self) -> BoxedBytes {
        let mb_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().mb_from_big_int_signed(self.handle.clone(), mb_handle.clone());
        M::managed_type_impl().mb_to_boxed_bytes(mb_handle)
    }

    #[inline]
    pub fn from_signed_bytes_be_buffer(managed_buffer: &ManagedBuffer<'a, M>) -> Self {
        let handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mb_to_big_int_signed(managed_buffer.handle.clone(), handle.clone());
        BigInt::from_handle(handle)
    }

    #[inline]
    pub fn to_signed_bytes_be_buffer(&self) -> ManagedBuffer<'a, M> {
        let mb_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mb_from_big_int_signed(self.handle.clone(), mb_handle.clone());
        ManagedBuffer::from_handle(mb_handle)
    }
}

impl<'a, M: ManagedTypeApi<'a>> Clone for BigInt<'a, M> {
    fn clone(&self) -> Self {
        let api = M::managed_type_impl();
        let clone_handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        api.bi_set_int64(clone_handle.clone(), 0);
        api.bi_add(
            clone_handle.clone(),
            clone_handle.clone(),
            self.handle.clone(),
        );
        BigInt::from_handle(clone_handle)
    }
}

impl<'a, M: ManagedTypeApi<'a>> BigInt<'a, M> {
    pub fn from_biguint(sign: Sign, unsigned: BigUint<'a, M>) -> Self {
        let api = M::managed_type_impl();
        if sign.is_minus() {
            api.bi_neg(unsigned.handle.clone(), unsigned.handle.clone());
        }
        BigInt::from_handle(unsigned.handle)
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
    pub fn magnitude(&self) -> BigUint<'a, M> {
        let api = M::managed_type_impl();
        let result_handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        api.bi_abs(result_handle.clone(), self.handle.clone());
        BigUint::from_handle(result_handle)
    }

    /// Convert this `BigInt` into its `Sign` and `BigUint` magnitude,
    /// the reverse of `BigInt::from_biguint`.
    pub fn to_parts(self) -> (Sign, BigUint<'a, M>) {
        (self.sign(), self.magnitude())
    }

    /// Converts this `BigInt` into a `BigUint`, if it's not negative.
    pub fn into_big_uint(self) -> ManagedOption<'a, M, BigUint<'a, M>> {
        if let Sign::Minus = self.sign() {
            ManagedOption::none()
        } else {
            ManagedOption::some(BigUint::from_handle(self.handle))
        }
    }
}

impl<'a, M: ManagedTypeApi<'a>> TryStaticCast for BigInt<'a, M> {}

impl<'a, M: ManagedTypeApi<'a>> TopEncode for BigInt<'a, M> {
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

impl<'a, M: ManagedTypeApi<'a>> NestedEncode for BigInt<'a, M> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.to_signed_bytes_be_buffer()
            .dep_encode_or_handle_err(dest, h)
    }
}

impl<'a, M: ManagedTypeApi<'a>> NestedDecode for BigInt<'a, M> {
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

impl<'a, M: ManagedTypeApi<'a>> TopDecode for BigInt<'a, M> {
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

impl<'a, M: ManagedTypeApi<'a>> crate::abi::TypeAbi for BigInt<'a, M> {
    fn type_name() -> TypeName {
        TypeName::from("BigInt")
    }
}

impl<'a, M: ManagedTypeApi<'a>> BigInt<'a, M> {
    #[must_use]
    pub fn pow(&self, exp: u32) -> Self {
        let result_handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        let exp_handle = BigUint::<'a, M>::make_temp(const_handles::BIG_INT_TEMPORARY_1, exp);
        M::managed_type_impl().bi_pow(result_handle.clone(), self.handle.clone(), exp_handle);
        BigInt::from_handle(result_handle)
    }
}

impl<'a, M: ManagedTypeApi<'a>> SCDisplay<'a> for BigInt<'a, M> {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        let str_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().bi_to_string(self.handle.clone(), str_handle.clone());
        f.append_managed_buffer(&ManagedBuffer::from_handle(
            str_handle.cast_or_signal_error::<'a, M, _>(),
        ));
    }
}

impl<'a, M: ManagedTypeApi<'a>> core::fmt::Debug for BigInt<'a, M> {
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
