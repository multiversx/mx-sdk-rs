use core::marker::PhantomData;

use elrond_codec::TryStaticCast;

use crate::{
    api::{EndpointFinishApi, EndpointFinishApiImpl, ErrorApi, ErrorApiImpl, ManagedTypeApi},
    elrond_codec::{EncodeError, TopEncode, TopEncodeOutput},
    err_msg,
    types::{BigFloat, BigInt, BigUint, ManagedBuffer, ManagedBufferCachedBuilder, ManagedType},
};

struct ApiOutputAdapter<FA>
where
    FA: ManagedTypeApi + EndpointFinishApi,
{
    _phantom: PhantomData<FA>,
}

impl<FA> ApiOutputAdapter<FA>
where
    FA: ManagedTypeApi + EndpointFinishApi,
{
    #[inline]
    fn new() -> Self {
        ApiOutputAdapter {
            _phantom: PhantomData,
        }
    }
}

impl<FA> TopEncodeOutput for ApiOutputAdapter<FA>
where
    FA: ManagedTypeApi + EndpointFinishApi,
{
    type NestedBuffer = ManagedBufferCachedBuilder<FA>;

    fn set_slice_u8(self, bytes: &[u8]) {
        FA::finish_api_impl().finish_slice_u8(bytes);
    }

    fn set_u64(self, value: u64) {
        FA::finish_api_impl().finish_u64(value);
    }

    fn set_i64(self, value: i64) {
        FA::finish_api_impl().finish_i64(value);
    }

    #[inline]
    fn set_unit(self) {
        // nothing: no result produced
    }

    #[inline]
    fn set_specialized<T, F>(self, value: &T, else_serialization: F) -> Result<(), EncodeError>
    where
        T: TryStaticCast,
        F: FnOnce(Self) -> Result<(), EncodeError>,
    {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<FA>>() {
            FA::finish_api_impl().finish_managed_buffer_raw(managed_buffer.handle);
            Ok(())
        } else if let Some(big_uint) = value.try_cast_ref::<BigUint<FA>>() {
            FA::finish_api_impl().finish_big_uint_raw(big_uint.handle);
            Ok(())
        } else if let Some(big_int) = value.try_cast_ref::<BigInt<FA>>() {
            FA::finish_api_impl().finish_big_int_raw(big_int.handle);
            Ok(())
        } else if let Some(big_float) = value.try_cast_ref::<BigFloat<FA>>() {
            FA::finish_api_impl().finish_big_float(big_float.handle);
            Ok(())
        } else {
            else_serialization(self)
        }
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        ManagedBufferCachedBuilder::new_from_slice(&[])
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        FA::finish_api_impl().finish_managed_buffer_raw(nb.into_managed_buffer().get_raw_handle());
    }
}

/// All types that are returned from endpoints need to implement this trait.
pub trait EndpointResult: Sized {
    /// Indicates how the result of the endpoint can be interpreted when called via proxy.
    /// `Self` for most types.
    type DecodeAs;

    fn finish<FA>(&self)
    where
        FA: ManagedTypeApi + EndpointFinishApi;
}

pub fn finish_all<FA, I, T>(items: I)
where
    FA: ManagedTypeApi + EndpointFinishApi,
    I: Iterator<Item = T>,
    T: EndpointResult,
{
    for item in items {
        item.finish::<FA>();
    }
}

/// All serializable objects can be used as smart contract function result.
impl<T> EndpointResult for T
where
    T: TopEncode,
{
    type DecodeAs = Self;

    fn finish<FA>(&self)
    where
        FA: ManagedTypeApi + EndpointFinishApi,
    {
        self.top_encode_or_exit(ApiOutputAdapter::<FA>::new(), (), finish_exit::<FA>);
    }
}

#[inline(always)]
fn finish_exit<FA>(_: (), encode_err: EncodeError) -> !
where
    FA: ManagedTypeApi + EndpointFinishApi + ErrorApi + 'static,
{
    let mut message_buffer = ManagedBuffer::<FA>::new_from_bytes(err_msg::FINISH_ENCODE_ERROR);
    message_buffer.append_bytes(encode_err.message_bytes());
    FA::error_api_impl().signal_error_from_buffer(message_buffer.get_raw_handle())
}
