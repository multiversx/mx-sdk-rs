use core::marker::PhantomData;

use crate::{
    abi::TypeName,
    api::{
        const_handles, BigIntApi, Handle, ManagedBufferApi, ManagedTypeApi, ManagedTypeApiImpl,
        StaticVarApiImpl,
    },
    formatter::{hex_util::encode_bytes_as_hex, FormatByteReceiver, SCDisplay},
    types::{heap::BoxedBytes, ManagedBuffer, ManagedType},
};
use elrond_codec::{
    CodecFrom, CodecFromSelf, DecodeErrorHandler, EncodeErrorHandler, NestedDecode,
    NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode,
    TopEncodeOutput, TryStaticCast,
};

#[repr(transparent)]
pub struct BigUint<M: ManagedTypeApi> {
    pub(crate) handle: Handle,
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigUint<M> {
    fn from_raw_handle(handle: Handle) -> Self {
        BigUint {
            handle,
            _phantom: PhantomData,
        }
    }

    fn get_raw_handle(&self) -> Handle {
        self.handle
    }

    fn transmute_from_handle_ref(handle_ref: &Handle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
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

macro_rules! big_uint_conv_num {
    ($num_ty:ty) => {
        impl<M: ManagedTypeApi> From<$num_ty> for BigUint<M> {
            #[inline]
            fn from(value: $num_ty) -> Self {
                let handle = M::static_var_api_impl().next_handle();
                M::managed_type_impl().bi_set_int64(handle, value as i64);
                BigUint::from_raw_handle(handle)
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
impl<M: ManagedTypeApi> CodecFrom<elrond_codec::num_bigint::BigUint> for BigUint<M> {}
#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> CodecFrom<BigUint<M>> for elrond_codec::num_bigint::BigUint {}

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
        let handle = M::static_var_api_impl().next_handle();
        M::managed_type_impl().bi_set_int64(handle, 0);
        BigUint::from_raw_handle(handle)
    }

    #[inline]
    pub fn to_u64(&self) -> Option<u64> {
        let api = M::managed_type_impl();
        api.bi_to_i64(self.handle).map(|bi| bi as u64)
    }

    #[inline]
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        let api = M::managed_type_impl();
        let result_handle = M::static_var_api_impl().next_handle();
        api.bi_set_unsigned_bytes(result_handle, bytes);
        BigUint::from_raw_handle(result_handle)
    }

    #[inline]
    pub fn to_bytes_be(&self) -> BoxedBytes {
        let api = M::managed_type_impl();
        api.bi_get_unsigned_bytes(self.handle)
    }

    #[inline]
    pub fn from_bytes_be_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        let handle = M::static_var_api_impl().next_handle();
        M::managed_type_impl().mb_to_big_int_unsigned(managed_buffer.handle, handle);
        BigUint::from_raw_handle(handle)
    }

    #[inline]
    pub fn to_bytes_be_buffer(&self) -> ManagedBuffer<M> {
        let mb_handle = M::static_var_api_impl().next_handle();
        M::managed_type_impl().mb_from_big_int_unsigned(self.handle, mb_handle);
        ManagedBuffer::from_raw_handle(mb_handle)
    }

    pub fn copy_to_array_big_endian_pad_right(&self, target: &mut [u8; 32]) {
        let api = M::managed_type_impl();
        api.mb_from_big_int_unsigned(self.handle, const_handles::MBUF_TEMPORARY_1);
        api.mb_copy_to_slice_pad_right(const_handles::MBUF_TEMPORARY_1, &mut target[..]);
    }
}

impl<M: ManagedTypeApi> BigUint<M> {
    #[inline]
    #[must_use]
    pub fn sqrt(&self) -> Self {
        let api = M::managed_type_impl();
        let result_handle = M::static_var_api_impl().next_handle();
        api.bi_sqrt(result_handle, self.handle);
        BigUint::from_raw_handle(result_handle)
    }

    #[must_use]
    pub fn pow(&self, exp: u32) -> Self {
        let result_handle = M::static_var_api_impl().next_handle();
        M::managed_type_impl().bi_set_int64(const_handles::BIG_INT_TEMPORARY_1, exp as i64);
        M::managed_type_impl().bi_pow(
            result_handle,
            self.handle,
            const_handles::BIG_INT_TEMPORARY_1,
        );
        BigUint::from_raw_handle(result_handle)
    }

    #[inline]
    pub fn log2(&self) -> u32 {
        let api = M::managed_type_impl();
        api.bi_log2(self.handle)
    }
}

impl<M: ManagedTypeApi> Clone for BigUint<M> {
    fn clone(&self) -> Self {
        let api = M::managed_type_impl();
        let clone_handle = M::static_var_api_impl().next_handle();
        M::managed_type_impl().bi_set_int64(clone_handle, 0);
        api.bi_add(clone_handle, clone_handle, self.handle);
        BigUint::from_raw_handle(clone_handle)
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

// TODO: should become part of the VM
fn format_big_uint_rec<M, F>(mut num: BigUint<M>, base: &BigUint<M>, f: &mut F)
where
    M: ManagedTypeApi,
    F: FormatByteReceiver,
{
    if num > 0 {
        let last_digit: BigUint<M> = &num % base;
        if let Some(last_digit_u64) = last_digit.to_u64() {
            num /= base;
            format_big_uint_rec(num, base, f);
            let ascii_last_digit = b'0' + last_digit_u64 as u8;
            f.append_bytes(&[ascii_last_digit][..]);
        }
    }
}

impl<M: ManagedTypeApi> SCDisplay for BigUint<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if self == &0 {
            f.append_bytes(&b"0"[..]);
        } else {
            format_big_uint_rec(self.clone(), &10u64.into(), f);
        }
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for BigUint<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BigUint")
            .field("handle", &self.handle)
            .field(
                "hex-value-be",
                &encode_bytes_as_hex(self.to_bytes_be().as_slice()),
            )
            .finish()
    }
}
