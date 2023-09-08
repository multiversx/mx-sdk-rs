use core::{borrow::Borrow, marker::PhantomData};

use crate::codec::{DecodeError, EncodeError};

use crate::{
    api::{ErrorApiImpl, ManagedTypeApi},
    types::{heap::BoxedBytes, ManagedBuffer, ManagedSCError, ManagedType},
};

#[derive(Default)]
pub struct ErrorHelper<M: ManagedTypeApi> {
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> ErrorHelper<M> {
    pub fn new() -> Self {
        ErrorHelper {
            _phantom: PhantomData,
        }
    }

    pub fn new_error(&self) -> ManagedSCError<M> {
        ManagedSCError::new_empty()
    }

    pub fn signal_error_with_message<T>(message: T) -> !
    where
        T: IntoSignalError<M>,
    {
        message.signal_error_with_message()
    }
}

/// Indicates how an object can be used as the basis for performing `signal_error` with itself as message.
pub trait IntoSignalError<M: ManagedTypeApi> {
    fn signal_error_with_message(self) -> !;
}

impl<M: ManagedTypeApi> IntoSignalError<M> for &str {
    #[inline]
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error(self.as_bytes())
    }
}

impl<M: ManagedTypeApi> IntoSignalError<M> for &[u8] {
    #[inline]
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error(self)
    }
}

impl<M: ManagedTypeApi> IntoSignalError<M> for BoxedBytes {
    #[inline]
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error(self.as_slice())
    }
}

impl<M: ManagedTypeApi> IntoSignalError<M> for EncodeError {
    #[inline]
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error(self.message_bytes())
    }
}

impl<M: ManagedTypeApi> IntoSignalError<M> for DecodeError {
    #[inline]
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error(self.message_bytes())
    }
}

// Handles `ManagedBuffer`, `&ManagedBuffer` and `ManagedRef<ManagedBuffer>`.
impl<M, B> IntoSignalError<M> for B
where
    M: ManagedTypeApi,
    B: Borrow<ManagedBuffer<M>>,
{
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error_from_buffer(self.borrow().get_handle())
    }
}
