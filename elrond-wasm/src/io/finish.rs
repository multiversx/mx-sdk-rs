use crate::api::{EndpointFinishApi, ErrorApi};
use crate::elrond_codec::{EncodeError, TopEncode, TopEncodeOutput};
use crate::Vec;

struct ApiOutputAdapter<FA>
where
    FA: EndpointFinishApi + Clone + 'static,
{
    api: FA,
}

impl<FA> ApiOutputAdapter<FA>
where
    FA: EndpointFinishApi + Clone + 'static,
{
    #[inline]
    fn new(api: FA) -> Self {
        ApiOutputAdapter { api }
    }
}

impl<FA> TopEncodeOutput for ApiOutputAdapter<FA>
where
    FA: EndpointFinishApi + Clone + 'static,
{
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
    fn set_big_int_handle_or_bytes<F: FnOnce() -> Vec<u8>>(self, handle: i32, _else_bytes: F) {
        self.api.finish_big_int_raw(handle);
    }

    #[inline]
    fn set_big_uint_handle_or_bytes<F: FnOnce() -> Vec<u8>>(self, handle: i32, _else_bytes: F) {
        self.api.finish_big_uint_raw(handle);
    }
}

/// All types that are returned from endpoints need to implement this trait.
pub trait EndpointResult: Sized {
    /// Indicates how the result of the endpoint can be interpreted when called via proxy.
    /// `Self` for most types.
    type DecodeAs;

    fn finish<FA>(&self, api: FA)
    where
        FA: EndpointFinishApi + Clone + 'static;
}

/// All serializable objects can be used as smart contract function result.
impl<T> EndpointResult for T
where
    T: TopEncode,
{
    type DecodeAs = Self;

    fn finish<FA>(&self, api: FA)
    where
        FA: EndpointFinishApi + Clone + 'static,
    {
        self.top_encode_or_exit(ApiOutputAdapter::new(api.clone()), api, finish_exit);
    }
}

#[inline(always)]
fn finish_exit<FA>(api: FA, en_err: EncodeError) -> !
where
    FA: EndpointFinishApi + ErrorApi + 'static,
{
    api.signal_error(en_err.message_bytes())
}
