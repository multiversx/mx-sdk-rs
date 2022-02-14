use core::marker::PhantomData;

use elrond_codec::{EncodeErrorHandler, TopEncodeMulti, TopEncodeMultiOutput, TryStaticCast};

use crate::{
    api::{EndpointFinishApi, EndpointFinishApiImpl, ManagedTypeApi},
    contract_base::ExitCodecErrorHandler,
    elrond_codec::{EncodeError, TopEncode, TopEncodeOutput},
    err_msg,
    types::{
        BigInt, BigUint, ManagedBuffer, ManagedBufferCachedBuilder, ManagedSCError, ManagedType,
        SCError, StaticSCError,
    },
};

#[derive(Clone)]
pub struct ApiOutputAdapter<FA>
where
    FA: ManagedTypeApi + EndpointFinishApi,
{
    _phantom: PhantomData<FA>,
}

impl<FA> Default for ApiOutputAdapter<FA>
where
    FA: ManagedTypeApi + EndpointFinishApi,
{
    #[inline]
    fn default() -> Self {
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
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<FA>>()
            || T::type_eq::<BigUint<FA>>()
            || T::type_eq::<BigInt<FA>>()
    }

    #[inline]
    fn set_specialized<T, H>(self, value: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TryStaticCast,
        H: EncodeErrorHandler,
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
        } else {
            Err(h.handle_error(EncodeError::UNSUPPORTED_OPERATION))
        }
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        ManagedBufferCachedBuilder::new_from_slice(&[])
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        FA::finish_api_impl().finish_managed_buffer_raw(nb.into_managed_buffer().get_raw_handle());
    }
}

impl<FA> TopEncodeMultiOutput for ApiOutputAdapter<FA>
where
    FA: ManagedTypeApi + EndpointFinishApi,
{
    type ValueOutput = Self;

    fn push_single_value<T, H>(&mut self, arg: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TopEncode,
        H: EncodeErrorHandler,
    {
        arg.top_encode_or_handle_err(self.clone(), h)
    }

    fn push_multi_specialized<T, H>(&mut self, arg: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TryStaticCast,
        H: EncodeErrorHandler,
    {
        if let Some(static_err) = arg.try_cast_ref::<StaticSCError>() {
            static_err.finish_err::<FA>()
        } else if let Some(managed_err) = arg.try_cast_ref::<ManagedSCError<FA>>() {
            managed_err.finish_err::<FA>()
        } else {
            Err(h.handle_error(EncodeError::UNSUPPORTED_OPERATION))
        }
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
    T: TopEncodeMulti,
{
    type DecodeAs = Self;

    fn finish<FA>(&self)
    where
        FA: ManagedTypeApi + EndpointFinishApi,
    {
        let Ok(()) = self.multi_encode_or_handle_err(
            &mut ApiOutputAdapter::<FA>::default(),
            ExitCodecErrorHandler::<FA>::from(err_msg::FINISH_ENCODE_ERROR),
        );
    }
}
