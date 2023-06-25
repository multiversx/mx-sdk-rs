use core::{convert::TryInto, marker::PhantomData};

use crate::{
    abi::TypeName,
    api::{
        const_handles, use_raw_handle, BigIntApiImpl, HandleConstraints, ManagedBufferApiImpl,
        ManagedTypeApi, ManagedTypeApiImpl, RawHandle, StaticVarApiImpl,
    },
    codec::{
        CodecFrom, CodecFromSelf, DecodeErrorHandler, EncodeErrorHandler, NestedDecode,
        NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode,
        TopEncodeOutput, TryStaticCast,
    },
    formatter::{hex_util::encode_bytes_as_hex, FormatByteReceiver, SCDisplay},
    types::{heap::BoxedBytes, BigUint, ManagedBuffer, ManagedOption, ManagedType, Sign},
};

use super::cast_to_i64::cast_to_i64;

#[repr(transparent)]
pub struct BigInt<M: ManagedTypeApi> {
    pub(crate) handle: M::BigIntHandle,
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigInt<M> {
    type OwnHandle = M::BigIntHandle;

    fn from_handle(handle: M::BigIntHandle) -> Self {
        BigInt {
            handle,
            _phantom: PhantomData,
        }
    }

    fn get_handle(&self) -> M::BigIntHandle {
        self.handle.clone()
    }

    fn transmute_from_handle_ref(handle_ref: &M::BigIntHandle) -> &Self {
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
        BigInt::from_handle(item.get_handle())
    }
}

macro_rules! big_int_conv_num {
    ($num_ty:ty) => {
        impl<M: ManagedTypeApi> From<$num_ty> for BigInt<M> {
            #[inline]
            fn from(value: $num_ty) -> Self {
                let handle: M::BigIntHandle =
                    use_raw_handle(M::static_var_api_impl().next_handle());
                Self::set_value(handle.clone(), value);
                BigInt::from_handle(handle)
            }
        }

        impl<M: ManagedTypeApi> CodecFrom<$num_ty> for BigInt<M> {}
    };
}

// TODO: more coverage, only from i64 currently tested
big_int_conv_num! {i64}
big_int_conv_num! {i32}
big_int_conv_num! {isize}
big_int_conv_num! {i16}
big_int_conv_num! {i8}

impl<M> CodecFromSelf for BigInt<M> where M: ManagedTypeApi {}

#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> CodecFrom<crate::codec::num_bigint::BigInt> for BigInt<M> {}
#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> CodecFrom<BigInt<M>> for crate::codec::num_bigint::BigInt {}

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
    pub fn from_signed_bytes_be_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        let handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mb_to_big_int_signed(managed_buffer.handle.clone(), handle.clone());
        BigInt::from_handle(handle)
    }

    #[inline]
    pub fn to_signed_bytes_be_buffer(&self) -> ManagedBuffer<M> {
        let mb_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mb_from_big_int_signed(self.handle.clone(), mb_handle.clone());
        ManagedBuffer::from_handle(mb_handle)
    }
}

impl<M: ManagedTypeApi> Clone for BigInt<M> {
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

impl<M: ManagedTypeApi> BigInt<M> {
    pub fn from_biguint(sign: Sign, unsigned: BigUint<M>) -> Self {
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
    pub fn magnitude(&self) -> BigUint<M> {
        let api = M::managed_type_impl();
        let result_handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        api.bi_abs(result_handle.clone(), self.handle.clone());
        BigUint::from_handle(result_handle)
    }

    /// Convert this `BigInt` into its `Sign` and `BigUint` magnitude,
    /// the reverse of `BigInt::from_biguint`.
    pub fn to_parts(self) -> (Sign, BigUint<M>) {
        (self.sign(), self.magnitude())
    }

    /// Converts this `BigInt` into a `BigUint`, if it's not negative.
    pub fn into_big_uint(self) -> ManagedOption<M, BigUint<M>> {
        if let Sign::Minus = self.sign() {
            ManagedOption::none()
        } else {
            ManagedOption::some(BigUint::from_handle(self.handle))
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

impl<M: ManagedTypeApi> crate::abi::TypeAbi for BigInt<M> {
    fn type_name() -> TypeName {
        TypeName::from("BigInt")
    }
}

impl<M: ManagedTypeApi> BigInt<M> {
    #[must_use]
    pub fn pow(&self, exp: u32) -> Self {
        let result_handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        let exp_handle = BigUint::<M>::make_temp(const_handles::BIG_INT_TEMPORARY_1, exp);
        M::managed_type_impl().bi_pow(result_handle.clone(), self.handle.clone(), exp_handle);
        BigInt::from_handle(result_handle)
    }
}

impl<M: ManagedTypeApi> SCDisplay for BigInt<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let str_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().bi_to_string(self.handle.clone(), str_handle.clone());
        f.append_managed_buffer(&ManagedBuffer::from_handle(
            str_handle.cast_or_signal_error::<M, _>(),
        ));
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
