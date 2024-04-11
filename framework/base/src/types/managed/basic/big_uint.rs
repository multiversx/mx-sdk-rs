use core::convert::TryInto;

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
    types::{heap::BoxedBytes, ManagedBuffer, ManagedType},
};

use super::cast_to_i64::cast_to_i64;

#[repr(transparent)]
pub struct BigUint<M: ManagedTypeApi> {
    pub(crate) handle: M::BigIntHandle,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigUint<M> {
    type OwnHandle = M::BigIntHandle;

    fn from_handle(handle: M::BigIntHandle) -> Self {
        BigUint { handle }
    }

    fn get_handle(&self) -> M::BigIntHandle {
        self.handle.clone()
    }

    fn transmute_from_handle_ref(handle_ref: &M::BigIntHandle) -> &Self {
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
    pub(crate) fn set_value<T>(handle: M::BigIntHandle, value: T)
    where
        T: TryInto<i64> + num_traits::Unsigned,
    {
        M::managed_type_impl().bi_set_int64(handle, cast_to_i64::<M, _>(value));
    }

    pub(crate) fn new_from_num<T>(value: T) -> Self
    where
        T: TryInto<i64> + num_traits::Unsigned,
    {
        let handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        Self::set_value(handle.clone(), value);
        BigUint::from_handle(handle)
    }

    pub(crate) fn make_temp<T>(handle: RawHandle, value: T) -> M::BigIntHandle
    where
        T: TryInto<i64> + num_traits::Unsigned,
    {
        let temp: M::BigIntHandle = use_raw_handle(handle);
        Self::set_value(temp.clone(), value);
        temp
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

        impl<M: ManagedTypeApi> CodecFrom<$num_ty> for BigUint<M> {}
    };
}

big_uint_conv_num! {u64}
big_uint_conv_num! {u32}
big_uint_conv_num! {usize}
big_uint_conv_num! {u16}
big_uint_conv_num! {u8}

impl<M> CodecFromSelf for BigUint<M> where M: ManagedTypeApi {}

#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> CodecFrom<crate::codec::num_bigint::BigUint> for BigUint<M> {}
#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> CodecFrom<BigUint<M>> for crate::codec::num_bigint::BigUint {}

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
        let handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().bi_set_int64(handle.clone(), 0);
        BigUint::from_handle(handle)
    }

    #[inline]
    pub fn to_u64(&self) -> Option<u64> {
        let api = M::managed_type_impl();
        api.bi_to_i64(self.handle.clone()).map(|bi| bi as u64)
    }

    #[inline]
    pub fn overwrite_u64(&self, value: u64) {
        Self::set_value(self.handle.clone(), value);
    }

    #[inline]
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        let mb_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().mb_overwrite(mb_handle.clone(), bytes);
        let handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mb_to_big_int_unsigned(mb_handle, handle.clone());
        BigUint::from_handle(handle)
    }

    #[inline]
    pub fn to_bytes_be(&self) -> BoxedBytes {
        let mb_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().mb_from_big_int_unsigned(self.handle.clone(), mb_handle.clone());
        M::managed_type_impl().mb_to_boxed_bytes(mb_handle)
    }

    #[inline]
    pub fn from_bytes_be_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        let handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl()
            .mb_to_big_int_unsigned(managed_buffer.handle.clone(), handle.clone());
        BigUint::from_handle(handle)
    }

    #[inline]
    pub fn to_bytes_be_buffer(&self) -> ManagedBuffer<M> {
        let mb_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mb_from_big_int_unsigned(self.handle.clone(), mb_handle.clone());
        ManagedBuffer::from_handle(mb_handle)
    }
}

impl<M: ManagedTypeApi> BigUint<M> {
    #[inline]
    #[must_use]
    pub fn sqrt(&self) -> Self {
        let api = M::managed_type_impl();
        let result_handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        api.bi_sqrt(result_handle.clone(), self.handle.clone());
        BigUint::from_handle(result_handle)
    }

    #[must_use]
    pub fn pow(&self, exp: u32) -> Self {
        let result_handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        let big_int_temp_1 = BigUint::<M>::make_temp(const_handles::BIG_INT_TEMPORARY_1, exp);
        M::managed_type_impl().bi_pow(result_handle.clone(), self.handle.clone(), big_int_temp_1);
        BigUint::from_handle(result_handle)
    }

    #[inline]
    pub fn log2(&self) -> u32 {
        let api = M::managed_type_impl();
        api.bi_log2(self.handle.clone())
    }
}

impl<M: ManagedTypeApi> Clone for BigUint<M> {
    fn clone(&self) -> Self {
        let api = M::managed_type_impl();
        let clone_handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        api.bi_set_int64(clone_handle.clone(), 0);
        api.bi_add(
            clone_handle.clone(),
            clone_handle.clone(),
            self.handle.clone(),
        );
        BigUint::from_handle(clone_handle)
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

impl<M: ManagedTypeApi> crate::abi::TypeAbi for BigUint<M> {
    fn type_name() -> TypeName {
        TypeName::from("BigUint")
    }
}

impl<M: ManagedTypeApi> SCDisplay for BigUint<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let str_handle: M::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().bi_to_string(self.handle.clone(), str_handle.clone());
        f.append_managed_buffer(&ManagedBuffer::from_handle(
            str_handle.cast_or_signal_error::<M, _>(),
        ));
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for BigUint<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BigUint")
            .field("handle", &self.handle.clone())
            .field(
                "hex-value-be",
                &encode_bytes_as_hex(self.to_bytes_be().as_slice()),
            )
            .finish()
    }
}
