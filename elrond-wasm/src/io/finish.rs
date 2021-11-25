use elrond_codec::TryStaticCast;

use crate::{
    api::{EndpointFinishApi, ErrorApi, ManagedTypeApi},
    elrond_codec::{EncodeError, TopEncode, TopEncodeOutput},
    err_msg,
    types::{BigInt, BigUint, ManagedBuffer, ManagedBufferCachedBuilder, ManagedType},
};

struct ApiOutputAdapter<FA>
where
    FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
{
    api: FA,
}

impl<FA> ApiOutputAdapter<FA>
where
    FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
{
    #[inline]
    fn new(api: FA) -> Self {
        ApiOutputAdapter { api }
    }
}

impl<FA> TopEncodeOutput for ApiOutputAdapter<FA>
where
    FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
{
    type NestedBuffer = ManagedBufferCachedBuilder<FA>;

    fn set_slice_u8(self, bytes: &[u8]) {
        self.api.finish_slice_u8(bytes);
    }

    fn set_u64(self, value: u64) {
        self.api.finish_u64(value);
    }

    fn set_i64(self, value: i64) {
        self.api.finish_i64(value);
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
            self.api.finish_managed_buffer_raw(managed_buffer.handle);
            Ok(())
        } else if let Some(big_uint) = value.try_cast_ref::<BigUint<FA>>() {
            self.api.finish_big_uint_raw(big_uint.handle);
            Ok(())
        } else if let Some(big_int) = value.try_cast_ref::<BigInt<FA>>() {
            self.api.finish_big_int_raw(big_int.handle);
            Ok(())
        } else {
            else_serialization(self)
        }
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        ManagedBufferCachedBuilder::new_from_slice(&[])
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        self.api
            .finish_managed_buffer_raw(nb.into_managed_buffer().get_raw_handle());
    }
}

/// All types that are returned from endpoints need to implement this trait.
pub trait EndpointResult: Sized {
    /// Indicates how the result of the endpoint can be interpreted when called via proxy.
    /// `Self` for most types.
    type DecodeAs;

    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static;
}

pub fn finish_all<FA, I, T>(api: FA, items: I)
where
    FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    I: Iterator<Item = T>,
    T: EndpointResult,
{
    for item in items {
        item.finish(api.clone());
    }
}

/// All serializable objects can be used as smart contract function result.
impl<T> EndpointResult for T
where
    T: TopEncode,
{
    type DecodeAs = Self;

    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        self.top_encode_or_exit(ApiOutputAdapter::new(api.clone()), api, finish_exit);
    }
}

#[inline(always)]
fn finish_exit<FA>(api: FA, encode_err: EncodeError) -> !
where
    FA: ManagedTypeApi + EndpointFinishApi + ErrorApi + 'static,
{
    let mut message_buffer = ManagedBuffer::<FA>::new_from_bytes(err_msg::FINISH_ENCODE_ERROR);
    message_buffer.append_bytes(encode_err.message_bytes());
    api.signal_error_from_buffer(message_buffer.get_raw_handle())
}
